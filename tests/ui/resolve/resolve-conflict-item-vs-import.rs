use std::mem::transmute;

fn transmute() {}
//~^ ERROR the name `transmute` is defined multiple times
//~| `transmute` redefined here
//~| `transmute` must be defined only once in the value namespace of this module
fn main() {
}

// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
