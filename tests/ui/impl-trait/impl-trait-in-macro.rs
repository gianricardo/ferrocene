use std::fmt::Debug;

macro_rules! i {
    ($($tr:tt)*) => { impl $($tr)* };
}

fn foo(x: i!(Debug), y: i!(Debug)) -> String {
    let mut a = x;
    a = y; //~ ERROR mismatched
    format!("{:?}", a)
}

trait S<T> {}

fn much_universe<T: S<i!(Debug)>, U: IntoIterator<Item = i!(Iterator<Item = i!(Clone)>)>>(
    _: i!(Debug + Clone),
) {
}

fn main() {}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_wjldgtio5o75
// Macro Expansion
//
// ferrocene-annotations: fls_vnvt40pa48n8
// Macro Invocation
//
// ferrocene-annotations: fls_4apk1exafxii
// Macro Matching
//
// ferrocene-annotations: fls_ym00b6ewf4n3
// Macro Transcription
//
// ferrocene-annotations: fls_8nzypdu9j3ge
// Metavariables
//
// ferrocene-annotations: fls_k01lsksqtq1r
// Repetition
//
// ferrocene-annotations: fls_n3ktmjqf87qb
// Rule Matching
//
// ferrocene-annotations: fls_qpx6lgapce57
// Token Matching
