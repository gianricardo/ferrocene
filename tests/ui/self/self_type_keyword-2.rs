use self::Self as Foo; //~ ERROR unresolved import `self::Self`

pub fn main() {
    let Self = 5;
    //~^ ERROR cannot find unit struct, unit variant or constant `Self` in this scope

    match 15 {
        Self => (),
        //~^ ERROR cannot find unit struct, unit variant or constant `Self` in this scope
        Foo { x: Self } => (),
        //~^ ERROR cannot find unit struct, unit variant or constant `Self` in this scope
    }
}

// ferrocene-annotations: fls_lish33a1naw5
// Keywords
//
// ferrocene-annotations: fls_mec5cg5aptf8
// Strict Keywords
