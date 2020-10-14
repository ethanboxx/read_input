use crate::{Inputtable, Prompt, Test};
use std::{
    ffi::OsString,
    io::{self, Write},
    string::ToString,
};

// Core function when running `.get()`.
pub(crate) fn read_input<T: Inputtable>(
    prompt: &Prompt,
    err: &str,
    default: Option<T>,
    tests: &[Test<T>],
    err_pass: &dyn Fn(&T::Failure) -> Option<String>,
) -> io::Result<T> {
    // Flush only when possible.
    fn try_flush() {
        io::stdout().flush().unwrap_or(())
    }

    fn input_as_osstring() -> io::Result<OsString> {
        let mut input = Vec::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.as_ref().into())
    }

    print!("{}", prompt.msg);
    try_flush();

    loop {
        let input = input_as_osstring()?;

        if input.is_empty() {
            if let Some(x) = default {
                return Ok(x);
            }
        };

        match parse_input(input, err, tests, err_pass) {
            Ok(v) => return Ok(v),
            Err(e) => println!("{}", e),
        };

        if prompt.repeat {
            print!("{}", prompt.msg);
            try_flush();
        };
    }
}

pub(crate) fn parse_input<T: Inputtable>(
    input: OsString,
    err: &str,
    tests: &[Test<T>],
    err_pass: &dyn Fn(&T::Failure) -> Option<String>,
) -> Result<T, String> {
    match T::input_attempt(input) {
        Ok(value) => {
            for test in tests {
                if !(test.func)(&value) {
                    return Err(test.err.clone().unwrap_or_else(|| err.to_string()));
                }
            }
            Ok(value)
        }
        Err(error) => Err(err_pass(&error).unwrap_or_else(|| err.to_string())),
    }
}
