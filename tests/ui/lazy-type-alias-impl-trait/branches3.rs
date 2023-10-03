#![feature(type_alias_impl_trait)]

type Foo = impl for<'a> FnOnce(&'a str) -> usize;
type Bar = impl FnOnce(&'static str) -> usize;

fn foo() -> Foo {
    if true {
        |s| s.len() //~ ERROR type annotations needed
    } else {
        panic!()
    }
}
fn bar() -> Bar {
    if true {
        |s| s.len() //~ ERROR type annotations needed
    } else {
        panic!()
    }
}

fn foo2() -> impl for<'a> FnOnce(&'a str) -> usize {
    if true {
        |s| s.len() //~ ERROR type annotations needed
    } else {
        panic!()
    }
}
fn bar2() -> impl FnOnce(&'static str) -> usize {
    if true {
        |s| s.len() //~ ERROR type annotations needed
    } else {
        panic!()
    }
}

fn main() {}

// ferrocene-annotations: fls_tjyexqrx0fx5
// Closure Expressions
//
// ferrocene-annotations: fls_3xqobbu7wfsf
// Impl Trait Type
//
// ferrocene-annotations: fls_kgvleup5mdhq
// Type Aliasing
