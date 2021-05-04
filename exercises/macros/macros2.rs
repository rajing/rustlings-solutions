// macros2.rs
// Make me compile! Execute `rustlings hint macros2` for hints :)

// Option 1
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}

// Option 2
//#[macro_export]
//macro_rules! my_macro {
//    () => {
//        println!("Check out my macro!");
//    };
//}
