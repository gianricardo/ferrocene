// FIXME: This file is tracking old lint behavior that's still unchanged in the
// unstable -Zthir-unsafeck implementation. See lint-unused-unsafe.rs for more details.
//
// Exercise the unused_unsafe attribute in some positive and negative cases

// compile-flags: -Zthir-unsafeck

#![allow(dead_code)]
#![deny(unused_unsafe)]


mod foo {
    extern "C" {
        pub fn bar();
    }
}

fn callback<T, F>(_f: F) -> T where F: FnOnce() -> T { panic!() }
unsafe fn unsf() {}

fn bad1() { unsafe {} }                  //~ ERROR: unnecessary `unsafe` block
fn bad2() { unsafe { bad1() } }          //~ ERROR: unnecessary `unsafe` block
unsafe fn bad3() { unsafe {} }           //~ ERROR: unnecessary `unsafe` block
fn bad4() { unsafe { callback(||{}) } }  //~ ERROR: unnecessary `unsafe` block
unsafe fn bad5() { unsafe { unsf() } }
fn bad6() {
    unsafe {                             // don't put the warning here
        unsafe {                         //~ ERROR: unnecessary `unsafe` block
            unsf()
        }
    }
}
unsafe fn bad7() {
    unsafe {
        unsafe {                         //~ ERROR: unnecessary `unsafe` block
            unsf()
        }
    }
}

unsafe fn good0() { unsf() }
fn good1() { unsafe { unsf() } }
fn good2() {
    /* bug uncovered when implementing warning about unused unsafe blocks. Be
       sure that when purity is inherited that the source of the unsafe-ness
       is tracked correctly */
    unsafe {
        unsafe fn what() -> Vec<String> { panic!() }

        callback(|| {
            what();
        });
    }
}

unsafe fn good3() { foo::bar() }
fn good4() { unsafe { foo::bar() } }

#[allow(unused_unsafe)] fn allowed() { unsafe {} }

fn main() {}

// ferrocene-annotations: fls_jep7p27kaqlp
// Unsafety
//
// ferrocene-annotations: fls_8wnyln2nmg4y
// Unsafe Blocks
//
// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
