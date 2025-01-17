// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::error::Error;
use crate::linkers::{GccMode, Linker};
use crate::report::Reporter;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::path::Path;

static SUPPORTED_TARGETS: &[TargetSpec] = &[
    TargetSpec {
        triple: "x86_64-unknown-linux-gnu",
        std: true,
        linker: Linker::GccUbuntu18 { target: "x86_64-linux-gnu", mode: GccMode::Normal },
    },
    TargetSpec {
        triple: "aarch64-unknown-linux-gnu",
        std: true,
        linker: Linker::GccUbuntu18 { target: "aarch64-linux-gnu", mode: GccMode::Normal },
    },
    TargetSpec { triple: "aarch64-unknown-none", std: false, linker: Linker::BundledLld },
];

#[derive(Debug)]
pub(crate) struct TargetSpec {
    pub(crate) triple: &'static str,
    pub(crate) std: bool,
    pub(crate) linker: Linker,
}

#[derive(Debug)]
pub(crate) struct Target {
    pub(crate) spec: &'static TargetSpec,
    pub(crate) rustflags: Vec<String>,
}

impl Deref for Target {
    type Target = TargetSpec;

    fn deref(&self) -> &Self::Target {
        &self.spec
    }
}

pub(crate) fn check(reporter: &dyn Reporter, sysroot: &Path) -> Result<Vec<Target>, Error> {
    let mut found = Vec::new();
    for target in SUPPORTED_TARGETS {
        match check_target(reporter, sysroot, target)? {
            CheckTargetOutcome::Missing => {}
            CheckTargetOutcome::Found => found.push(Target { spec: target, rustflags: Vec::new() }),
        }
    }
    Ok(found)
}

fn check_target(
    reporter: &dyn Reporter,
    sysroot: &Path,
    target: &TargetSpec,
) -> Result<CheckTargetOutcome, Error> {
    let target_dir = sysroot.join("lib").join("rustlib").join(&target.triple);
    if !target_dir.is_dir() {
        // Target not present, ignore it.
        return Ok(CheckTargetOutcome::Missing);
    }

    check_libraries(
        target,
        &target_dir,
        if target.std {
            &["core", "alloc", "std", "test", "proc_macro"]
        } else {
            &["core", "alloc"]
        },
    )?;

    reporter.success(&format!("target installed correctly: {}", target.triple));
    Ok(CheckTargetOutcome::Found)
}

#[derive(Debug, PartialEq, Eq)]
enum CheckTargetOutcome {
    Missing,
    Found,
}

fn check_libraries(target: &TargetSpec, target_dir: &Path, expected: &[&str]) -> Result<(), Error> {
    let lib_dir = target_dir.join("lib");

    let mut expected_to_find = expected.into_iter().map(|s| s.to_string()).collect::<HashSet<_>>();
    for (library, count) in find_libraries_in(&lib_dir)?.into_iter() {
        if count > 1 {
            return Err(Error::DuplicateTargetLibrary { target: target.triple.into(), library });
        }
        expected_to_find.remove(&library);
    }

    if let Some(library) = expected_to_find.drain().next() {
        return Err(Error::TargetLibraryMissing {
            target: target.triple.into(),
            library: (*library).into(),
        });
    }

    Ok(())
}

fn find_libraries_in(path: &Path) -> Result<HashMap<String, usize>, Error> {
    let map_err = |e| Error::TargetLibraryDiscoveryFailed { path: path.into(), error: e };

    let mut found = HashMap::new();
    for entry in path.read_dir().map_err(map_err)? {
        let path = entry.map_err(map_err)?.path();
        if !path.is_file() {
            continue;
        }
        let Some(library) = extract_library_name(&path) else { continue };

        *found.entry(library.to_string()).or_insert(0) += 1;
    }

    Ok(found)
}

fn extract_library_name(file_path: &Path) -> Option<&str> {
    let (library, hash) = file_path
        .file_name()?
        .to_str()?
        .strip_prefix("lib")?
        .strip_suffix(".rlib")?
        .rsplit_once('-')?;

    if hash.len() != 16 || hash.chars().any(|c| !c.is_ascii_hexdigit()) || library.is_empty() {
        None
    } else {
        Some(library)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linkers::Linker::BundledLld;
    use crate::test_utils::TestUtils;

    #[test]
    fn test_check_target_std() {
        let target =
            TargetSpec { triple: "x86_64-unknown-linux-gnu", std: true, linker: BundledLld };

        let utils = TestUtils::new();
        utils
            .target("x86_64-unknown-linux-gnu")
            .lib("core", "0123456789abcdef")
            .lib("alloc", "0123456789abcdef")
            .lib("std", "0123456789abcdef")
            .lib("test", "0123456789abcdef")
            .lib("proc_macro", "0123456789abcdef")
            .lib("other", "0123456789abcdef") // Unknown libraries are ignored
            .create();

        assert_eq!(
            CheckTargetOutcome::Found,
            check_target(utils.reporter(), utils.sysroot(), &target).unwrap()
        );
        utils.assert_report_success("target installed correctly: x86_64-unknown-linux-gnu");
    }

    #[test]
    fn test_check_target_no_std() {
        let target = TargetSpec { triple: "x86_64-unknown-none", std: false, linker: BundledLld };

        let utils = TestUtils::new();
        utils
            .target("x86_64-unknown-none")
            .lib("core", "0123456789abcdef")
            .lib("alloc", "0123456789abcdef")
            .lib("other", "0123456789abcdef") // Unknown libraries are ignored
            .create();

        assert_eq!(
            CheckTargetOutcome::Found,
            check_target(utils.reporter(), utils.sysroot(), &target).unwrap()
        );
        utils.assert_report_success("target installed correctly: x86_64-unknown-none");
    }

    #[test]
    fn test_check_target_missing_library() {
        let target = TargetSpec { triple: "x86_64-unknown-none", std: false, linker: BundledLld };

        let utils = TestUtils::new();
        utils.target("x86_64-unknown-none").lib("core", "0123456789abcdef").create();

        match check_target(utils.reporter(), utils.sysroot(), &target) {
            Err(Error::TargetLibraryMissing { target, library }) => {
                assert_eq!(target, "x86_64-unknown-none");
                assert_eq!(library, "alloc");
            }
            other => panic!("unexpected result: {other:?}"),
        }
        utils.assert_no_reports();
    }

    #[test]
    fn test_check_target_duplicate_required_library() {
        let target = TargetSpec { triple: "x86_64-unknown-none", std: false, linker: BundledLld };

        let utils = TestUtils::new();
        utils
            .target("x86_64-unknown-none")
            .lib("core", "0123456789abcdef")
            .lib("core", "abcdef0123456789")
            .lib("alloc", "0123456789abcdef")
            .create();

        match check_target(utils.reporter(), utils.sysroot(), &target) {
            Err(Error::DuplicateTargetLibrary { target, library }) => {
                assert_eq!(target, "x86_64-unknown-none");
                assert_eq!(library, "core");
            }
            other => panic!("unexpected result: {other:?}"),
        }
        utils.assert_no_reports();
    }

    #[test]
    fn test_check_target_duplicate_other_library() {
        let target = TargetSpec { triple: "x86_64-unknown-none", std: false, linker: BundledLld };

        let utils = TestUtils::new();
        utils
            .target("x86_64-unknown-none")
            .lib("core", "0123456789abcdef")
            .lib("other", "0123456789abcdef")
            .lib("other", "abcdef0123456789")
            .lib("alloc", "0123456789abcdef")
            .create();

        match check_target(utils.reporter(), utils.sysroot(), &target) {
            Err(Error::DuplicateTargetLibrary { target, library }) => {
                assert_eq!(target, "x86_64-unknown-none");
                assert_eq!(library, "other");
            }
            other => panic!("unexpected result: {other:?}"),
        }
        utils.assert_no_reports();
    }

    #[test]
    fn test_find_libraries_in() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();

        let create_lib = |name| std::fs::write(dir.join(name), b"").unwrap();
        create_lib("libcore-0123456789abcdef.rlib");
        create_lib("libcore-abcdef0123456789.rlib");
        create_lib("liballoc-0123456789abcdef.rlib");
        create_lib("libproc_macro-0123456789abcdef.rlib");
        create_lib("foo-0123456789abcdef.so"); // Invalid files are not counted.

        let output = find_libraries_in(dir).unwrap();
        assert_eq!(3, output.len());
        assert_eq!(2, output["core"]);
        assert_eq!(1, output["alloc"]);
        assert_eq!(1, output["proc_macro"]);
    }

    #[test]
    fn test_find_libraries_in_missing_directory() {
        let temp = tempfile::tempdir().unwrap();
        let missing = temp.path().join("missing");

        let err = find_libraries_in(&missing).unwrap_err();
        if let Error::TargetLibraryDiscoveryFailed { path, error } = err {
            assert_eq!(missing, path);
            assert_eq!(error.kind(), std::io::ErrorKind::NotFound);
        } else {
            panic!("wrong error type");
        }
    }

    #[test]
    fn test_extract_library_name() {
        assert_eq!(Some("core"), extract_library_name(&Path::new("libcore-0123456789abcdef.rlib")));
        assert_eq!(
            Some("proc_macro"),
            extract_library_name(&Path::new("libproc_macro-0123456789abcdef.rlib"))
        );

        let assert_fail = |name: &str| {
            assert!(
                extract_library_name(&Path::new(name)).is_none(),
                "{name} is treated as valid but should be wrong"
            )
        };
        assert_fail("libcore-0123456789abcde.rlib"); // Hash too short
        assert_fail("libcore-0123456789abcdef0.rlib"); // Hash too long
        assert_fail("libcore-0123456789abcdeg.rlib"); // Non-hexdigit in hash
        assert_fail("core-0123456789abcdef.rlib"); // No "lib" prefix
        assert_fail("lib-0123456789abcdef.rlib"); // No library name
        assert_fail("libcore-0123456789abcdef.so"); // Different extension
        assert_fail("libcore-0123456789abcdef"); // No extension
    }
}
