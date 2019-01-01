//To run this example `cargo run --example yes_no --release`

extern crate dont_disappear;
extern crate read_input;

use read_input::prelude::*;
use std::str::FromStr;

#[derive(Debug)]
enum Choice {
    Positive,
    Negative,
}

use self::Choice::*;

#[derive(Debug)]
struct ChoiceParseError {
    unknown: String,
}

impl FromStr for Choice {
    type Err = ChoiceParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "y" | "yes" | "true" | "positive" => Ok(Positive),
            "n" | "no" | "false" | "negative" => Ok(Negative),
            _ => Err(ChoiceParseError {
                unknown: s.to_string(),
            }),
        }
    }
}

fn main() {
    println!("You inputted\n{:#?}", input::<Choice>().get());
    dont_disappear::enter_to_continue::default();
}
