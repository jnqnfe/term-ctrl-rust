// Copyright 2017 Lyndon Brown
//
// Licensed under the MIT license or the Apache license (version 2.0), at your option. You may not
// copy, modify, or distribute this file except in compliance with said license. You can find copies
// of these licenses either in the LICENSE-MIT and LICENSE-APACHE files, or alternatively at
// <http://opensource.org/licenses/MIT> and <http://www.apache.org/licenses/LICENSE-2.0>
// respectively.

//! ANSI terminal text formatting assistant
//!
//! Some virtual-terminals (ANSI ones being the focus here) have a feature of allowing command-line
//! programs executed in them to request that certain limited formatting effects such as colour or
//! boldness be applied to portions of the text the program outputs for them to display. This is
//! achieved through the program embedding special “ANSI control” byte sequences into that text
//! output. This can prove very useful for instance in highlighting error notices.
//!
//! The purpose of this crate is to offer functionality to assist in generating and using such
//! control sequences.
//!
//! Note that such formatting should only be used when the `stdout` stream (or `stderr` as
//! applicable) is connected to an actual terminal that will process and use them (you wouldn't want
//! these control sequences to exist in output that the user is having the terminal redirect into a
//! text file or another program as then it would just be treated as a part of the text and lead to
//! garbled output and confusion). Some utility functions are provided within the [support mod] to
//! assist you with determining appropriateness of using formatted output.
//!
//! # Predefined sequences
//!
//! There is no strict need to actually understand how to construct such control sequences to make
//! use of the formatting they offer; many predefined sequences are available for your use in the
//! [predefined mod](predefined/index.html).
//!
//! To make use of these simply inject them into the right points of a to-be-printed string via
//! format arguments, as demonstrated below. Remember not to forget to make use of a reset after the
//! text to be formatted. For instance:
//!
//! ```rust
//! use term_ctrl::predefined::{RESET, color1::RED};
//! println!("{}Error:{} You made an error!", RED, RESET);
//! ```
//!
//! In the example just given, the text “<span style="color:#c00">Error:</span>” will be highlighted
//! in red. Of course as mentioned earlier, you should only use the colour sequences if output is
//! actually connected to a terminal (and that the terminal has not redirected it, so do not assume
//! output to stdout is okay). Here the example is modified to use a helper to check suitability and
//! a filter to control use:
//!
//! ```rust
//! use term_ctrl::predefined::{RESET, color1::RED};
//! let format = term_ctrl::support::fmt_supported_stdout();
//! let filter = |seq| { match format { true => seq, false => "" } };
//! println!("{}Error:{} You made an error!", filter(RED), filter(RESET));
//! ```
//!
//! Note, when resetting to normal, be sure to always use the proper reset sequence. Do not make the
//! mistake of setting text colour to black and presuming that this achieves the same thing; it does
//! not. (Consider that some people have black text on a white background in their terminal, whilst
//! others use the opposite! Black text on a black background does not work very well!).
//!
//! # Constructing sequences
//!
//! Although understanding the sequences is not required, there are however benefits to be gained
//! from it, since for instance not every possible combination can reasonably be provided in
//! predefined form, and you may be able to achieve greater efficiency by constructing a custom one
//! rather than using multiple predefines in sequence.
//!
//! Let’s take a quick look at the sequence itself, then we can move on to discuss a macro which
//! helps make custom construction very easy.
//!
//! ## The sequence pattern
//!
//! The sequence pattern consists of four component parts:
//!
//!  1. `'\u{1B}'` (the escape `char`, Unicode 0x1B)
//!  2. `'['` (an opening bracket `char`)
//!  3. One or more numbers using a semi-colon (`';'`) as a separator
//!  4. `'m'` (a lower-case letter m)
//!
//! In other words a pattern of: `"\u{1B}[` <em><codes\></em> `m"`
//!
//! Every control sequence fits this template, and it is the set of numbers included in it which
//! determine the precise formatting nature of the request. As a quick example, the sequence of
//! `"\u{1B}[31;1m"` specifies two numbers, `31` which corresponds to red text, and `1` which
//! corresponds to bold text (the number to effect mapping is discussed shortly).
//!
//! Typically each of the individual numbers given corresponds to a particular effect, however as we
//! will see shortly there are a couple of exceptions where multiple numbers are used.
//!
//! Before we continue, understand that the effects specified are applied in sequence and are
//! applied cumulatively both with respect to previous effects in the same sequence and also to any
//! existing effects still in effect from a previous sequence. There are no numbers assigned for
//! turning off individual effects, just one “reset” code (`0`) which removes all effects currently
//! applied.
//!
//! ## Macro construction
//!
//! As just mentioned, a macro (called [`seq`]) is provided to assist with constructing sequences.
//! All you have to do is provide it with a list of decimal numbers and it will construct a string
//! of the above pattern that contains them. An example:
//!
//! ```rust
//! use term_ctrl::seq;
//! assert_eq!("\u{1B}[1;2;3m", seq!(1, 2, 3));
//! ```
//!
//! Note that you are not restricted to pure numeric literals, string literals work also:
//!
//! ```rust
//! use term_ctrl::seq;
//! assert_eq!("\u{1B}[1;2;3m", seq!("1", "2", "3"));
//! assert_eq!("\u{1B}[1;2;3m", seq!("1;2", 3));
//! ```
//!
//! Convenience macros are also provided for constructing 256-colour and RGB colour code sets for
//! use in a sequence (they do not generate the full sequence, just a multi-code set of numbers to
//! use in a sequence).
//!
//! ## Number chart
//!
//! The following is a guide to the available number codes and what they correspond to. In most
//! cases a single number code corresponds to a single effect, however there are also those that
//! require a sequence of multiple numbers (with the normal semi-colon separator).
//!
//! ### Effects
//!
//!  - `0`: Normal (reset)
//!  - `1`: Bold
//!  - `2`: Dim (faint)
//!  - `3`: Italic
//!  - `4`: Underlined
//!  - `5`: Blink
//!  - `6`: *<unused?>*
//!  - `7`: “Inverse”
//!  - `8`: Invisible (hidden)
//!  - `9`: Strike-through
//!
//! ### Basic colours
//!
//!  - Text (foreground): `30`-`37`
//!  - Background-highlight: `40`-`47`
//!  - Bright text (foreground): `90`-`97`
//!  - Bright background-highlight: `100`-`107`
//!
//! The last digit of the number corresponds to a colour as follows:
//!
//!  - `_0`: Black
//!  - `_1`: Red
//!  - `_2`: Green
//!  - `_3`: Yellow
//!  - `_4`: Blue
//!  - `_5`: Magenta
//!  - `_6`: Cyan
//!  - `_7`: White
//!  - `_8`: Used as part of an *extended* colour sequence, as discussed shortly
//!
//! So for instance `32` means green text.
//!
//! ### Extended range colours
//!
//! Wider choice of colours than above is made available as *extended* colours through use of a
//! sequence of codes (using the normal semi-colon separator).
//!
//! There are two ranges available, an 8-bit (256 colour) range, and a 24-bit RGB range. Both begin
//! with either `38` to choose text (foreground colouring) or `48` for background-highlight
//! colouring. This is followed by `5` for specifying 256-colour or `2` for specifying RGB colour.
//! With 256-colour a final third number with a value in the range of 0-255 selects the specific
//! colour. With RGB, three numbers must be given, each corresponding to red, green and blue
//! respectively, and each also being of a value in the range 0-255.
//!
//! As an example, `seq!(38,5,238)` changes the colour of text (not background) since it starts with
//! `38`, it is providing a 256-colour palette selection (the `5`), and is specifically selecting
//! colour `238` from that palette.
//!
//! As another example, `seq!(48,2,180,15,70)` changes the colour of the text background since it
//! starts with `48`, it is providing an RGB colour (the `2`), and is then followed with RGB values
//! of `180` for red, `15` for green and `70` for blue.
//!
//! [support mod]: support/index.html
//! [`seq`]: macro.seq.html

#![no_std]

extern crate atty;
#[cfg(windows)]
extern crate winapi;

pub mod predefined;
pub mod support;

/// Constructs a control sequence
///
/// A sequence can be created with one or more numbers separated by commas.
///
/// # Examples:
///
/// ```rust
/// # use term_ctrl::seq;
/// // Bold
/// assert_eq!("\u{1B}[1m", seq!(1));
/// // Red + bold + underline
/// assert_eq!("\u{1B}[31;1;4m", seq!(31,1,4));
/// ```
#[macro_export]
macro_rules! seq {
    ($n1:expr, $($n2:expr),*) => { concat!("\u{1B}[", $n1, $(concat!(";", $n2)),*, "m") };
    ($n1:expr, $($n2:expr,)*) => { concat!("\u{1B}[", $n1, $(concat!(";", $n2)),*, "m") };
    ($n:expr) => { concat!("\u{1B}[", $n, "m") };
    () => { "" };
}

/// Constructs a 256-colour foreground (text) colour code set (to be used in a control sequence)
///
/// # Examples:
///
/// ```rust
/// # use term_ctrl::{seq, c256_fg};
/// // Get the codes
/// assert_eq!("38;5;238", c256_fg!(238));
/// // Use in a sequence
/// assert_eq!("\u{1B}[38;5;238m", seq!(c256_fg!(238)));
/// ```
#[macro_export]
macro_rules! c256_fg {
    ($col:expr) => { concat!("38;5", $col) };
}

/// Constructs a 256-colour background colour code set (to be used in a control sequence)
///
/// # Examples:
///
/// ```rust
/// # use term_ctrl::{seq, c256_bg};
/// // Get the codes
/// assert_eq!("48;5;238", c256_bg!(238));
/// // Use in a sequence
/// assert_eq!("\u{1B}[48;5;238m", seq!(c256_bg!(238)));
/// ```
#[macro_export]
macro_rules! c256_bg {
    ($col:expr) => { concat!("48;5", $col) };
}

/// Constructs an RGB foreground (text) colour code set (to be used in a control sequence)
///
/// # Examples:
///
/// ```rust
/// # use term_ctrl::{seq, rgb_fg};
/// // Get the RGB codes
/// assert_eq!("38;2;180;15;70", rgb_fg!(180, 15, 70));
/// // Use in a sequence
/// assert_eq!("\u{1B}[38;2;180;15;70m", seq!(rgb_fg!(180, 15, 70)));
/// ```
#[macro_export]
macro_rules! rgb_fg {
    ($red:expr, $green:expr, $blue:expr) => { concat!("38;2;", $red, ";", $green, ";", $blue) };
}

/// Constructs an RGB background colour code set (to be used in a control sequence)
///
/// # Examples:
///
/// ```rust
/// # use term_ctrl::{seq, rgb_bg};
/// // Get the RGB codes
/// assert_eq!("48;2;180;15;70", rgb_bg!(180, 15, 70));
/// // Use in a sequence
/// assert_eq!("\u{1B}[48;2;180;15;70m", seq!(rgb_bg!(180, 15, 70)));
/// ```
#[macro_export]
macro_rules! rgb_bg {
    ($red:expr, $green:expr, $blue:expr) => { concat!("48;2;", $red, ";", $green, ";", $blue) };
}
