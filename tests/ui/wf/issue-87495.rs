// Regression test for the ICE described in #87495.

trait T {
    const CONST: (bool, dyn T);
    //~^ ERROR: the trait `T` cannot be made into an object [E0038]
}

fn main() {}

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_ixjc5jaamx84
// Constants
