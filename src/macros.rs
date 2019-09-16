// Copyright 2019 Lyndon Brown
//
// Licensed under the MIT license or the Apache license (version 2.0), at your option. You may not
// copy, modify, or distribute this file except in compliance with said license. You can find copies
// of these licenses either in the LICENSE-MIT and LICENSE-APACHE files, or alternatively at
// <http://opensource.org/licenses/MIT> and <http://www.apache.org/licenses/LICENSE-2.0>
// respectively.

//! Macros

/// Constructs a code set (to be used in a control sequence)
///
/// This simply turns a sequence of numbers into a string with a semi-colon separator.
///
/// Note that this will actually happily accept strings also, not just numeric literals.
///
/// # Examples:
///
/// ```rust
/// # use term_ctrl::codes;
/// assert_eq!("1", codes!(1));
/// assert_eq!("1;2;3", codes!(1, 2, 3));
/// assert_eq!("1;2;3;4;5", codes!(1, "2;3;4", 5));
/// ```
#[macro_export]
macro_rules! codes {
    ($n1:expr, $($n2:expr),*) => { concat!($n1, $(concat!(";", $n2)),*) };
    ($n1:expr, $($n2:expr,)*) => { concat!($n1, $(concat!(";", $n2)),*) };
    ($n:expr) => { concat!($n) };
    () => { "" };
}

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
    ($($n:expr),*) => { concat!("\u{1B}[", $crate::codes!($($n),*), "m") };
    ($($n:expr,)*) => { concat!("\u{1B}[", $crate::codes!($($n),*), "m") };
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
    ($col:expr) => { $crate::codes!(38, 5, $col) };
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
    ($col:expr) => { $crate::codes!(48, 5, $col) };
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
    ($red:expr, $green:expr, $blue:expr) => { $crate::codes!(38, 2, $red, $green, $blue) };
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
    ($red:expr, $green:expr, $blue:expr) => { $crate::codes!(48, 2, $red, $green, $blue) };
}
