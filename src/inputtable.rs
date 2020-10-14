use std::{
    convert::Infallible, ffi::OsString, fmt, marker::PhantomData, num::IntErrorKind, str::FromStr,
};

use num_traits::real::Real;

pub trait Inputtable: Sized {
    type Failure: InputFail;

    const INIT_MSG: &'static str = "Please input a value: ";
    const MSG: &'static str = Self::INIT_MSG;

    fn input_attempt(input: OsString) -> Result<Self, Self::Failure>;
}

pub trait InputFail {
    fn msg(self) -> String;
}

impl InputFail for Infallible {
    fn msg(self) -> String {
        match self {}
    }
}

pub enum MaybeUtf8<E> {
    NotUtf8(OsString),
    Other(E),
}

impl<E> From<E> for MaybeUtf8<E> {
    fn from(e: E) -> Self {
        Self::Other(e)
    }
}

impl<E> MaybeUtf8<E> {
    pub fn parse_input_as_string<T>(
        input: OsString,
        parser: impl Fn(String) -> Result<T, E>,
    ) -> Result<T, Self> {
        Ok(match input.into_string() {
            Ok(s) => parser(s)?,
            Err(s) => return Err(Self::NotUtf8(s)),
        })
    }
}

impl<E: InputFail> InputFail for MaybeUtf8<E> {
    fn msg(self) -> String {
        match self {
            Self::NotUtf8(_) => "Please input valid UTF8".to_owned(),
            Self::Other(e) => e.msg(),
        }
    }
}

impl Inputtable for OsString {
    type Failure = Infallible;

    const INIT_MSG: &'static str = "Please input any value: ";

    fn input_attempt(input: OsString) -> Result<Self, Self::Failure> {
        Ok(input)
    }
}

impl Inputtable for bool {
    type Failure = MaybeUtf8<NotBool>;

    const INIT_MSG: &'static str = "Please input true or false: ";

    fn input_attempt(input: OsString) -> Result<Self, Self::Failure> {
        MaybeUtf8::parse_input_as_string(input, |input| {
            Ok(match input.as_str() {
                "true" => true,
                "false" => false,
                _ => return Err(NotBool(input)),
            })
        })
    }
}

struct NotBool(pub String);

impl InputFail for NotBool {
    fn msg(self) -> String {
        todo!()
    }
}

impl Inputtable for char {
    type Failure = MaybeUtf8<NotChar>;

    const INIT_MSG: &'static str = "Please input a character: ";

    fn input_attempt(input: OsString) -> Result<Self, Self::Failure> {
        MaybeUtf8::parse_input_as_string(input, |input| {
            let mut chars = input.chars();
            match (chars.next(), chars.next()) {
                (None, _) => Err(NotChar::EmptyString),
                (Some(c), None) => Ok(c),
                _ => Err(NotChar::TooManyChars),
            }
        })
    }
}

enum NotChar {
    EmptyString,
    TooManyChars,
}

impl InputFail for NotChar {
    fn msg(self) -> String {
        match self {
            NotChar::EmptyString => "",
            NotChar::TooManyChars => "Only type a single character",
        }
        .to_owned()
    }
}

pub struct NotInt<I> {
    kind: IntErrorKind,
    min: I,
    max: I,
}

impl<I: fmt::Display> InputFail for NotInt<I> {
    fn msg(self) -> String {
        match self.kind {
            IntErrorKind::Empty => "Please Input a value".to_owned(),
            IntErrorKind::InvalidDigit => {
                "You inputted an invalid digit. Please input characters 0-9".to_owned()
            }
            IntErrorKind::Overflow => {
                format!("Please only input values below or equal to {}", self.max)
            }
            IntErrorKind::Underflow => {
                format!("Please only input values above or equal to {}", self.min)
            }
            IntErrorKind::Zero => "Please input a non-zero value".to_owned(),
        }
    }
}

macro_rules! impl_default_builder_for_int {
    ($($t:ty),*) => {$(
    impl Inputtable for $t {
        type Failure = MaybeUtf8<NotInt<Self>>;

        const INIT_MSG: &'static str = "Please input an integer: ";

        fn input_attempt(input: OsString) -> Result<Self, Self::Failure> {
            MaybeUtf8::parse_input_as_string(input, |input| {
                input.parse::<Self>().map_err(|e| NotInt{kind: e.kind().clone(), min: Self::MIN, max: Self::MAX})
            })
        }
    }
    )*}
}

impl_default_builder_for_int! { i8, i16, i32, i64, i128, isize }

// macro_rules! impl_default_builder_for_whole {
//     ($($t:ty),*) => {$(
//     impl DefaultBuilderSettings for $t {
//         fn settings() -> InputBuilder<Self> {
//             input()
//                 .repeat_msg("Please input a positive integer: ")
//                 .err("Only type positive integers.")
//         }
//     }
//     )*}
// }

// impl_default_builder_for_whole! { u8, u16, u32, u64, u128, usize }

// macro_rules! impl_default_builder_for_float {
//     ($($t:ty),*) => {$(
//     impl DefaultBuilderSettings for $t {
//         fn settings() -> InputBuilder<Self> {
//             input()
//                 .repeat_msg("Please input a number: ")
//                 .err("Only type numbers or decimal point.")
//         }
//     }
//     )*}
// }

// impl_default_builder_for_float! { f32, f64 }
