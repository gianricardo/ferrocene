struct Foo {
    bar: u8
}

fn main() {
    let f = Foo { bar: 22 };
    f.baz; //~ ERROR no field
}

// ferrocene-annotations: fls_18k3uajrgq5f
// Field Access Expressions
//
// ferrocene-annotations: fls_6ydylimiv553
// Place Expressions
//
// ferrocene-annotations: fls_xcwfotmq2e5d
// Field Resolution
