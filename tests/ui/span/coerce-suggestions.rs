fn test(_x: &mut String) {}

fn test2(_x: &mut i32) {}


fn main() {
    let x: usize = String::new();
    //~^ ERROR E0308
    let x: &str = String::new();
    //~^ ERROR E0308
    let y = String::new();
    test(&y);
    //~^ ERROR E0308
    test2(&y);
    //~^ ERROR E0308
    let f;
    f = Box::new(f);
    //~^ ERROR E0308

    let s = &mut String::new();
    s = format!("foo");
    //~^ ERROR E0308
}

// ferrocene-annotations: fls_dw33yt5g6m0k
// Type Coercion
