// TODO: Fix the compiler error without taking the macro definition out of this
// module.
#[macro_use]
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}
// use macros;

fn main() {
    my_macro!();
}
