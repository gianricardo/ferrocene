// run-rustfix
#[allow(unused_macros)]

macro_rules! foo! { //~ ERROR macro names aren't followed by a `!`
    () => {};
}

fn main() {}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
