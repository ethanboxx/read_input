//! Collection of functions that make things a little less verbose.

use crate::{test_generators::InsideFunc, InputBuild, InputBuilder, Inputtable};
use std::{error::Error, fmt::Display, str::FromStr};

/// Shortcut function. Fetches input that is validated with a test function.
pub fn valid_input<T, F>(test: F) -> T
where
    T: Inputtable,
    F: Fn(&T) -> bool + 'static,
{
    input().add_test(test).get()
}

/// Shortcut function. Fetches input that is within a range, array or vector.
pub fn input_inside<T, U>(constraint: U) -> T
where
    T: Inputtable,
    U: InsideFunc<T>,
{
    input().inside(constraint).get()
}

/// Shortcut function. Fetches input that is valid for whatever type needed.
pub fn simple_input<T: Inputtable>() -> T {
    input().get()
}

/// Creates a new instance of `InputBuilder` with generic, minimal settings.
pub fn input<T: Inputtable>() -> InputBuilder<T> {
    InputBuilder::new()
}

/// Creates a new instance of `InputBuilder` with settings specifically
/// tailored to the type you want.
pub fn input_d<T: DefaultBuilderSettings>() -> InputBuilder<T> {
    T::settings()
}

/// Trait for describing specifically tailored input settings for types.
pub trait DefaultBuilderSettings: Inputtable {
    /// Returns tailored `InputBuilder`.
    fn settings() -> InputBuilder<Self>;
}

/// Produces an error message from an error type. Made for use in `.err_match()`
pub fn with_display<T: Display>(x: &T) -> Option<String> {
    Some(format!("Error: \"{}\"", x))
}

#[deprecated(
    since = "0.8.4",
    note = "Deprecated due to the depreciation of `std::error::Error::description`. Please use the `with_display` function instead."
)]
#[allow(deprecated)]
/// Produces an error message from an error type. Made for use in `.err_match()`
pub fn with_description<T: Error>(x: &T) -> Option<String> {
    Some(format!("Error: \"{}\"", (*x).description()))
}
