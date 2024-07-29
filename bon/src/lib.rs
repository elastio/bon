#![doc = include_str!("../README.md")]

pub use bon_macros::*;

/// Symbols used by macros. They are not stable and are considered an implementation detail.
#[doc(hidden)]
pub mod private;

/// Same as [`std::vec!`] but converts each element with [`Into::into()`].
///
/// **WARNING:** it's not recommended to import this macro into scope. Reference it
/// using the full path (`bon::vec![]`) to avoid confusion with the [`std::vec!`] macro.
///
/// A good example of the use case for this macro is when you want to create a
/// [`Vec<String>`] where part of the items are hardcoded string literals of type
/// `&str` and the other part is made of dynamic [`String`] values.
///
/// ```
/// fn convert_media(input_extension: &str, output_extension: &str) -> std::io::Result<()> {
///     let ffmpeg_args: Vec<String> = bon::vec![
///         "-i",
///         format!("input.{input_extension}"),
///         "-y",
///         format!("output.{output_extension}"),
///     ];
///
///     std::process::Command::new("ffmpeg").args(ffmpeg_args).output()?;
///
///     Ok(())
/// }
/// ```
///
/// This macro doesn't support `vec![expr; N]` syntax, since it's simpler to
/// just write `vec![expr.into(); N]` using [`std::vec!`] instead.
#[macro_export]
macro_rules! vec {
    () => (::std::vec::Vec::new());
    ($($item:expr),+ $(,)?) => (::std::vec![$(::core::convert::Into::into($item)),+ ]);
}

/// Creates a fixed-size array literal where each element is converted with [`Into::into()`]
/// into the target type.
///
/// You'll probably need a hint for the target type of items in the array if the
/// compiler can't infer it from its usage.
///
/// This is similar in spirit to the [`bon::vec!`] macro, but it's for arrays.
/// See [`bon::vec!`] docs for details.
///
/// Same example as in [`bon::vec!`], but using this macro. It works with array
/// as well because [`Command::args`] accepts any value that implements [`IntoIterator`]:
///
/// ```
/// fn convert_media(input_extension: &str, output_extension: &str) -> std::io::Result<()> {
///     let ffmpeg_args: [String; 4] = bon::arr![
///         "-i",
///         format!("input.{input_extension}"),
///         "-y",
///         format!("output.{output_extension}"),
///     ];
///
///     std::process::Command::new("ffmpeg").args(ffmpeg_args).output()?;
///
///     Ok(())
/// }
/// ```
///
/// This macro doesn't support `[expr; N]` syntax, since it's simpler to
/// just write `[expr.into(); N]` instead.
///
/// [`Command::args`]: std::process::Command::args
/// [`bon::vec!`]: crate::vec
#[macro_export]
macro_rules! arr {
    () => ([]);
    ($($item:expr),+ $(,)?) => ([$(::core::convert::Into::into($item)),+]);
}

#[cfg(test)]
mod tests {
    #[test]
    fn arr_smoke() {
        let actual: [String; 3] = crate::arr!["foo", "bar", "baz"];
        assert_eq!(actual, ["foo", "bar", "baz"]);

        let actual: [String; 0] = crate::arr![];
        assert!(actual.is_empty());
    }

    #[test]
    fn vec_smoke() {
        let actual: Vec<String> = crate::vec!["foo", "bar", "baz"];
        assert_eq!(actual, ["foo", "bar", "baz"]);

        let actual: Vec<String> = crate::vec![];
        assert!(actual.is_empty());
    }
}
