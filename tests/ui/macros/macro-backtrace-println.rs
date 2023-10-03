// The `format_args!` syntax extension issues errors before code expansion
// has completed, but we still need a backtrace.

// This test includes stripped-down versions of `print!` and `println!`,
// because we can't otherwise verify the lines of the backtrace.

fn print(_args: std::fmt::Arguments) {}

macro_rules! myprint {
    ($($arg:tt)*) => (print(format_args!($($arg)*)));
}

macro_rules! myprintln {
    ($fmt:expr) => (myprint!(concat!($fmt, "\n"))); //~ ERROR no arguments were given
}

fn main() {
    myprintln!("{}");
}

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
