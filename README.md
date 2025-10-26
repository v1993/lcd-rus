# lcd-rus

lcd-rus is a crate for converting ASCII + Russian UTF-8 strings to the encoding
commonly used by LCDs such as HD44780U. It's a `no_std` crate designed to be used
in an embedded environment. Both compile time and run time transformations are
available.

Please note that this library does not include routines for actually communicating
with the display. You need a separate crate for that.

## Compile time transformation

Two macros, [lcd_const] and [lcd_literal], are provided to allow transcoding strings
at compile time. This is usually preferred as it avoids inflating the binary, incurs
zero runtime overhead, and ensures characters are valid during the build.

You must `use` the crate itself in places where the macros are invoked.

## Run time transformation

If there's no way around runtime transformation [lcd_encode_runtime] can be used
to perform it. It's generally discouraged due not being very optimized neither
for size nor speed, including calling safe versions of operations where possible.

On Arduino Uno the code for performing transformations at runtime takes about
3 KB of flash while using compile time transformation takes exactly zero.

## Other

`const` function [lcd_length] is provided for counting characters. It's usually not needed
unless you're doing runtime transformation or writing your own macros.

## Panics

Functions may panic if provided with invalid UTF-8; this should not be surprising as
strings in Rust are expected to be UTF-8 encoded. Additionally, compile-time transformation
will panic to cause build failure if a character can't be transcoded.

## Caveats

Some LCD libraries take `&str` instead of `&[u8]` for strings; this is somewhat of a bug
since LCD screens don't expect UTF-8 encoded text. You should report this as an issue
to library maintainers and work around it manually for the time being.
