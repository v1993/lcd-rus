#![no_std]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

/// Certain internals have to be exported for macros to work.
#[doc(hidden)]
pub mod internal;

/// Get length of an array required to fit the LCD encoded characters.
///
/// This is normally only needed if you're encoding strings at runtime,
/// but may also come in handy if you're writing your own macros.
///
/// It's worth noting that this function is not very optimized, both in
/// terms of size and execution speed. As such, using it at runtime
/// is discouraged.
pub const fn lcd_length(s: &str) -> usize {
	internal::utf8_len(s)
}

/// Convert a string into LCD encoding at runtime.
///
/// Takes a string to convert and a buffer to write transcoded characters to.
/// You can call [lcd_length] to know how many characters you need space for.
///
/// Returns a result indicating if all of the characters were converted
/// and how many were.
///
/// This interface is mostly provided for the sake of completeness; use the
/// macros for compile time transformation if possible instead to avoid
/// both runtime overhead and binary size increase.
///
/// Example
///
/// ```
/// use lcd_rus::lcd_encode_runtime;
///
/// let mut buf = [0u8; 16];
/// let len = lcd_encode_runtime(&mut buf, "Hello, мир!").unwrap();
///
/// // Print the message on your LCD; the function must take &[u8] rather than &str
/// lcd.write_bytes(&mut delay, &buf[..len]);
/// ```
pub fn lcd_encode_runtime(output: &mut [u8], s: &str) -> Result<usize, usize> {
	internal::str_to_lcd_runtime(output, s)
}

/// Convert a string literal into LCD encoding, storing result in a constant.
///
/// The produced constant will be of type `[u8; _]`.
///
/// [lcd_literal] is often more convenient to use.
///
/// This transformation is performed entirely at compile time;
/// with correct optimization settings no code at all should be emitted.
/// Strings containing unexpected characters will result in build failure.
///
/// The crate must be imported under its name for this macro to work.
///
/// # Example
///
/// ```
/// use lcd_rus::{self, lcd_const};
///
/// // Declare the constant
/// lcd_const!(MY_MSG, "Hello, мир!");
///
/// // Print the message on your LCD; the function must take &[u8] rather than &str
/// lcd.print(&MY_MSG);
/// ```
#[macro_export]
macro_rules! lcd_const {
	($id:ident, $s:literal) => {
		const $id: [u8; lcd_rus::lcd_length($s)] =
			lcd_rus::internal::str_to_lcd_const::<{ lcd_rus::lcd_length($s) }>($s);
	};
}

/// Convert a string literal into LCD encoding, returning the result.
///
/// Return type is a `[u8; _]`.
///
/// This transformation is performed entirely at compile time with transformed string stored in a constant;
/// with correct optimization settings no code at all should be emitted. Strings containing unexpected characters
/// will result in build failure.
///
/// The crate must be imported under its name for this macro to work.
///
/// # Example
///
/// ```
/// use lcd_rus::{self, lcd_literal};
///
/// // Print the message on your LCD; the function must take &[u8] rather than &str
/// lcd.print(&lcd_literal!("Hello, мир!"));
/// ```
#[macro_export]
macro_rules! lcd_literal {
	($s:literal) => {{
		const STRING: [u8; lcd_rus::lcd_length($s)] =
			lcd_rus::internal::str_to_lcd_const::<{ lcd_rus::lcd_length($s) }>($s);
		STRING
	}};
}
