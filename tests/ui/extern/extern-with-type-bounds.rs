#![feature(intrinsics, rustc_attrs)]

extern "rust-intrinsic" {
    // Real example from libcore
    #[rustc_safe_intrinsic]
    fn type_id<T: ?Sized + 'static>() -> u64;

    // Silent bounds made explicit to make sure they are actually
    // resolved.
    fn transmute<T: Sized, U: Sized>(val: T) -> U;

    // Bounds aren't checked right now, so this should work
    // even though it's incorrect.
    #[rustc_safe_intrinsic]
    fn size_of<T: Clone>() -> usize;

    // Unresolved bounds should still error.
    fn align_of<T: NoSuchTrait>() -> usize;
    //~^ ERROR cannot find trait `NoSuchTrait` in this scope
}

fn main() {}

// ferrocene-annotations: fls_yztwtek0y34v
// External Functions
