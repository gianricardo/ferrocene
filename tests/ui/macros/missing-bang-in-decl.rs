// run-rustfix

#![allow(unused_macros)]

macro_rules foo {
    //~^ ERROR expected `!` after `macro_rules`
    () => {};
}

macro_rules bar! {
    //~^ ERROR expected `!` after `macro_rules`
    //~^^ ERROR macro names aren't followed by a `!`
    () => {};
}

fn main() {}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
