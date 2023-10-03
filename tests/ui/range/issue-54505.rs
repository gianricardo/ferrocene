// run-rustfix

// Regression test for #54505 - range borrowing suggestion had
// incorrect syntax (missing parentheses).

use std::ops::RangeBounds;


// take a reference to any built-in range
fn take_range(_r: &impl RangeBounds<i8>) {}


fn main() {
    take_range(0..1);
    //~^ ERROR mismatched types [E0308]
    //~| HELP consider borrowing here
    //~| SUGGESTION &(

    take_range(1..);
    //~^ ERROR mismatched types [E0308]
    //~| HELP consider borrowing here
    //~| SUGGESTION &(

    take_range(..);
    //~^ ERROR mismatched types [E0308]
    //~| HELP consider borrowing here
    //~| SUGGESTION &(

    take_range(0..=1);
    //~^ ERROR mismatched types [E0308]
    //~| HELP consider borrowing here
    //~| SUGGESTION &(

    take_range(..5);
    //~^ ERROR mismatched types [E0308]
    //~| HELP consider borrowing here
    //~| SUGGESTION &(

    take_range(..=42);
    //~^ ERROR mismatched types [E0308]
    //~| HELP consider borrowing here
    //~| SUGGESTION &(
}

// ferrocene-annotations: fls_18swodqqzadj
// Range Expressions
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
