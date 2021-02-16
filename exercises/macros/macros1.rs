// macros1.rs
// Make me compile! Execute `rustlings hint macros1` for hints :)

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($x:expr) => {
        println!("{}", $x);
    };
}

fn main() {
    my_macro!();
    my_macro!("Hello!");
}
