//To run this example `cargo run --example debug --release`
//This example shows debug prints for types.

use read_input::prelude::*;

fn main() {
    println!(
        "{:#?}",
        input()
            .repeat_msg("Please input your guess: ")
            .inside_err(..=100, "That number is more than 100. Please try again")
            .default(10)
            .inside_err(1.., "That number is less than 1. Please try again")
            .err("That does not look like a number. Please try again")
    );
    println!(
        "{:?}",
        input()
            .repeat_msg("Please input your guess: ")
            .inside_err(..=100, "That number is more than 100. Please try again")
            .inside_err(1.., "That number is less than 1. Please try again")
            .err("That does not look like a number. Please try again")
    );
}
