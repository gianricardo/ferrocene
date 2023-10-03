struct Foo<'a> {
    s: &'a mut String
}

fn f(x: usize, f: &Foo) {
    f.s.push('x'); //~ ERROR cannot borrow `*f.s` as mutable, as it is behind a `&` reference
}

fn main() {}

// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_18k3uajrgq5f
// Field Access Expressions
//
// ferrocene-annotations: fls_xcwfotmq2e5d
// Field Resolution
