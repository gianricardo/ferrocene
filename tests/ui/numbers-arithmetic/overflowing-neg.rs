// run-fail
// error-pattern:attempt to negate with overflow
// ignore-emscripten no processes
// compile-flags: -C debug-assertions

#![allow(arithmetic_overflow)]

fn main() {
    let _x = -i8::MIN;
}

// ferrocene-annotations: um_rustc_C_debug_assertions
