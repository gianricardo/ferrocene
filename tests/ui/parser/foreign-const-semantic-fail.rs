fn main() {}

extern "C" {
    const A: isize;
    //~^ ERROR extern items cannot be `const`
    const B: isize = 42;
    //~^ ERROR extern items cannot be `const`
    //~| ERROR incorrect `static` inside `extern` block
}

// ferrocene-annotations: fls_ixjc5jaamx84
// Constants
//
// ferrocene-annotations: fls_tmoh3y9oyqsy
// External Blocks
