macro_rules! m {
    ($($e1:expr),*; $($e2:expr),*) => {
        $( let x = $e1 )*; //~ ERROR expected one of `.`, `;`, `?`, `else`, or
        $( println!("{}", $e2) )*;
    }
}

fn main() { m!(0, 0; 0, 0); }

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
