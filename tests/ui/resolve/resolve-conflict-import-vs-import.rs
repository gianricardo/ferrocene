// run-rustfix

#[allow(unused_imports)]
use std::mem::transmute;
use std::mem::transmute;
//~^ ERROR the name `transmute` is defined multiple times

fn main() {
}

// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
