// macros1.rs
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a hint.

// #[macro_export]  这个不需要，在同一个包中。应该是。
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
