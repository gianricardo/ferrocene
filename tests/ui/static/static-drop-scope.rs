struct WithDtor;

impl Drop for WithDtor {
    fn drop(&mut self) {}
}

static PROMOTION_FAIL_S: Option<&'static WithDtor> = Some(&WithDtor);
//~^ ERROR destructor of
//~| ERROR temporary value dropped while borrowed

const PROMOTION_FAIL_C: Option<&'static WithDtor> = Some(&WithDtor);
//~^ ERROR destructor of
//~| ERROR temporary value dropped while borrowed

static EARLY_DROP_S: i32 = (WithDtor, 0).1;
//~^ ERROR destructor of

const EARLY_DROP_C: i32 = (WithDtor, 0).1;
//~^ ERROR destructor of

const fn const_drop<T>(_: T) {}
//~^ ERROR destructor of

const fn const_drop2<T>(x: T) {
    (x, ()).1
    //~^ ERROR destructor of
}

const EARLY_DROP_C_OPTION: i32 = (Some(WithDtor), 0).1;
//~^ ERROR destructor of

const HELPER: Option<WithDtor> = Some(WithDtor);

const EARLY_DROP_C_OPTION_CONSTANT: i32 = (HELPER, 0).1;
//~^ ERROR destructor of

fn main () {}

// ferrocene-annotations: fls_ixjc5jaamx84
// Constants
//
// ferrocene-annotations: fls_4jiw35pan7vn
// Destruction
//
// ferrocene-annotations: fls_u2mzjgiwbkz0
// Destructors
//
// ferrocene-annotations: fls_rm4ncoopcdvj
// Drop Scopes
//
// ferrocene-annotations: fls_afafmafz4hf2
// Drop Order
