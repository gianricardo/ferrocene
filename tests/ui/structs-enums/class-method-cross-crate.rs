// run-pass
// aux-build:cci_class_2.rs

extern crate cci_class_2;
use cci_class_2::kitties::cat;

pub fn main() {
  let nyan : cat = cat(52, 99);
  let kitty = cat(1000, 2);
  assert_eq!(nyan.how_hungry, 99);
  assert_eq!(kitty.how_hungry, 2);
  nyan.speak();
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_maw4u1o8q37u
// Crates
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_z7q8kbjwdc7g
// Method Call Expressions
//
// ferrocene-annotations: fls_18k3uajrgq5f
// Field Access Expressions
//
// ferrocene-annotations: fls_xcwfotmq2e5d
// Field Resolution
//
// ferrocene-annotations: fls_wqazkzle0ix9
// Method Resolution
