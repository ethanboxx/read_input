use std::io;
use std::io::Write;

const DEFAULT_ERR: &str = "That value does not pass please try again";

pub struct InputBuilder<'a, T>
where
    T: std::str::FromStr,
{
    msg: Option<&'a str>,
    err: Option<&'a str>,
    default: Option<T>,
    test: Vec<(Box<dyn Fn(&T) -> bool>, Option<&'a str>)>,
    err_match: Box<dyn Fn(&T::Err) -> Option<String>>,
}

impl<'a, T> InputBuilder<'a, T>
where
    T: std::str::FromStr,
{
    pub fn msg(self, msg: &'a str) -> Self {
        InputBuilder {
            msg: Some(msg),
            ..self
        }
    }
    pub fn err(self, err: &'a str) -> Self {
        InputBuilder {
            err: Some(err),
            ..self
        }
    }
    pub fn default(self, default: T) -> Self {
        InputBuilder {
            default: Some(default),
            ..self
        }
    }
    fn test<F: 'static + Fn(&T) -> bool>(self, test: F, err: Option<&'a str>) -> Self {
        InputBuilder {
            test: {
                let mut x = self.test;
                x.push((Box::new(test), err));
                x
            },
            ..self
        }
    }
    pub fn add_test<F: 'static + Fn(&T) -> bool>(self, test: F) -> Self {
        self.test(test, None)
    }
    pub fn add_err_test<F: 'static + Fn(&T) -> bool>(self, test: F, err: &'a str) -> Self {
        self.test(test, Some(err))
    }
    pub fn clear_tests(self) -> Self {
        InputBuilder {
            test: Vec::new(),
            ..self
        }
    }
    pub fn err_match<F: 'static + Fn(&T::Err) -> Option<String>>(self, err_match: F) -> Self {
        InputBuilder {
            err_match: Box::new(err_match),
            ..self
        }
    }
    pub fn get(self) -> T {
        read_input::<T>(self.msg, self.err, self.default, self.test, self.err_match)
    }
}

pub fn input_new<'a, T>() -> InputBuilder<'a, T>
where
    T: std::str::FromStr,
{
    InputBuilder {
        msg: None,
        err: None,
        default: None,
        test: Vec::new(),
        err_match: Box::new(|_| None),
    }
}

pub fn valid_input<T, F: 'static + Fn(&T) -> bool>(test: F) -> T
where
    T: std::str::FromStr,
{
    input_new::<T>().add_test::<F>(test).get()
}

pub fn simple_input<T>() -> T
where
    T: std::str::FromStr,
{
    input_new().get()
}

fn read_input<'a, T>(
    msg: Option<&str>,
    err: Option<&str>,
    default: Option<T>,
    test: Vec<(Box<dyn Fn(&T) -> bool>, Option<&'a str>)>,
    err_pass: Box<dyn Fn(&T::Err) -> Option<String>>,
) -> T
where
    T: std::str::FromStr,
{
    if let Some(msg) = msg {
        print!("{}", msg);
        io::stdout().flush().expect("could not flush output");
    };
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    if input.trim().is_empty() {
        if let Some(x) = default {
            return x;
        }
    };
    loop {
        match T::from_str(&input.trim()) {
            Ok(value) => {
                let mut test_err = None;
                if test.iter().all(|f| {
                    if f.0(&value) {
                        true
                    } else {
                        test_err = Some(f.1.unwrap_or(err.unwrap_or(DEFAULT_ERR)));
                        false
                    }
                }) {
                    return value;
                } else {
                    println!("{}", test_err.unwrap_or(err.unwrap_or(DEFAULT_ERR)));
                }
            }
            Err(error) => {
                println!(
                    "{}",
                    err_pass(&error).unwrap_or(err.unwrap_or(DEFAULT_ERR).to_string())
                );
            }
        }
        input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    }
}
