// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.



  macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }

mod macros {

}


fn main() {
    my_macro!();
}
