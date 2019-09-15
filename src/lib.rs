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
//! All you have to do is provide it with a list of numbers and it will construct a string of the
//! above pattern that contains them. An example:
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
//! ## Number chart
//!
//! The available numbers consist of the following (divided up into related groups):
//!
//!  - Reset: `0`
//!  - Effects: `1`-`7`
//!  - Basic colours:
//!     - `30`-`37` for text colour (set #1)
//!     - `40`-`47` for background-highlight colour (set #1)
//!     - `90`-`97` for text colour (set #2)
//!     - `100`-`107` for background-highlight colour (set #2)
//!
//! It is also possible to express *extended* colours using a multi-number sequence, as explained
//! shortly.
//!
//! The effects, specifically, are:
//!
//!  - `1`: Bold
//!  - `2`: Dim
//!  - `3`: Italic
//!  - `4`: Underline
//!  - `5`: Blink
//!  - `6`: *<unused?>*
//!  - `7`: “Reverse”
//!  - `8`: Invisible (hidden)
//!  - `9`: Strike-through
//!
//! With respect to the basic colour set ranges, the last digit of the number corresponds to a
//! colour as follows:
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
//! So for instance `32` means green text (set #1).
//!
//! Note that there are two different sets for each of these basic text and background colour
//! choices. Some terminals offer a second alternate palette of colour mappings, which could map to
//! identical colours as the first, or could offer slight variations of the precise colours used.
//! Note that the actual colours output may possibly be adjusted by user themes, or through user
//! preferences, should the terminal program offer such user configuration.
//!
//! With respect to *extended* colours, these are expressed using multiple numbers, starting with
//! `_8` from one of the basic colour sets. There are two choices available here:
//!
//!  - The “extended” 256-colour palette: This uses three numbers. The first was just explained; the
//!    second must be `5`; and the third should be a value in the range `0`-`255`, corresponding to
//!    an *extended* colour from a set of 256 available colours.
//!  - The full 24-bit RGB colour palette: This uses five numbers. The first was just explained; the
//!    second must be `2`; and the last three are RGB (red, green, and blue) values, each in the
//!    range `0`-`255`.
//!
//! For instance:
//!
//!  - `seq!(38,5,238)` would select colour `238` from the “extended” 256-colour palette, as a text
//!    colour (since `38` was used).
//!  - `seq!(48,2,180,15,70)` would specify red=180, green=15, and blue=70 as a background colour
//!    from the 24-bit RGB colour range.
//!
//! [support mod]: support/index.html
//! [`seq`]: macro.seq.html

#![no_std]

extern crate atty;
#[cfg(windows)]
extern crate winapi;

pub mod predefined;
pub mod support;

/// Macro for constructing control sequences
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
