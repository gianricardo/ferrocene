#![allow(bare_trait_objects)]

fn main() {
    let _: &Copy + 'static; //~ ERROR expected a path
    //~^ ERROR cannot be made into an object
    let _: &'static Copy + 'static; //~ ERROR expected a path
}

// ferrocene-annotations: fls_qa98qdi42orq
// Trait Object Type
