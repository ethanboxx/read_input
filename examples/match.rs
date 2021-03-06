//To run this example `cargo run --example match --release`
//This example shows how best to use `match` on a inputted value.
//The thing to note form this program is the use of `unreachable!()`

use read_input::shortcut::*;

fn main() {
    match input_inside(2..=4) {
        2 => println!("You inputted the number 2"),
        3 => println!("You inputted the number 3"),
        4 => println!("You inputted the number 4"),
        _ => unreachable!(),
    }
    dont_disappear::enter_to_continue::default();
}
