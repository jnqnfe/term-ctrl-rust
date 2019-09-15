// Copyright 2017 Lyndon Brown
//
// Licensed under the MIT license or the Apache license (version 2.0), at your option. You may not
// copy, modify, or distribute this file except in compliance with said license. You can find copies
// of these licenses either in the LICENSE-MIT and LICENSE-APACHE files, or alternatively at
// <http://opensource.org/licenses/MIT> and <http://www.apache.org/licenses/LICENSE-2.0>
// respectively.

//! Console/terminal text control
//!
//! Text output to a console/terminal can (sometimes) be modified with certain (limited) effects
//! (colors, boldness, etc). This is achieved by injecting a special sequence of characters into the
//! output sent to the console/terminal. This special sequence will be interpreted by the
//! console/terminal when it displays the output.
//!
//! This is not available on Windows (apparently), and should only be used otherwise when stdout is
//! actually a tty (you wouldn't want these control sequences to exist in output that the user is
//! piping into a text file or another program).
//!
//! This crate provides tools and pre-defined definitions to assist in using these effects.
//!
//! # The sequence pattern
//!
//! The special sequence to be injected into the output sent to the console/terminal is comprised of
//! four parts:
//!
//!  1. `\u{1B}`, the escape char
//!  2. `[`
//!  3. One or more numbers using `;` as a separator
//!  4. `m`
//!
//! The numbers used in the third part each correspond to a particular effect. The effects specified
//! are applied in sequence and are applied cumulatively both with respect to previous effects in
//! the same sequence and also to any existing effects previously applied by a past sequence. There
//! are no numbers assigned for turning off individual effects, just one 'reset' effect (`0`) which
//! removes all effects currently applied.
//!
//! An example: `"\u{1B}[31;1m"` specifies two effects, red text (`31`) and bold text (`1`).
//!
//! A list/description of number mapping is given in the [`numbers`](numbers/index.html) mod
//! documentation.
//!
//! ## Macro
//!
//! The `seq` macro has been provided for constructing a sequence `str`. Import it via:
//!
//! ```rust
//! #[macro_use]
//! extern crate term_ctrl;
//! # fn main() {}
//! ```
//!
//! # Resetting
//!
//! When resetting text back to "normal", prefer `RESET` (`"\u{1B}[0m"`) rather than setting
//! color to black (or white).
//!
//! # Examples
//!
//! Injecting sequences (irregardless of whether or not it is a good idea to do so)
//!
//! ```rust
//! # #[macro_use]
//! # extern crate term_ctrl;
//! # fn main() {
//! const RESET: &str = seq!(0); // "\u{1B}[0m"
//! const RED_BOLD: &str = seq!(31,1); // "\u{1B}[31;1m"
//! println!("normal-text {}red-and-bold-text{} normal-text", RED_BOLD, RESET);
//! # }
//! ```
//!
//! Same, but using predefined sequences
//!
//! ```rust
//! use term_ctrl::predefined::*;
//! println!("normal-text {}red-and-bold-text{} normal-text", color1_bold::RED, RESET);
//! ```
//!
//! Injecting sequences while being careful of whether or not to do so (of which there are many
//! potential solutions, of varying efficiency)
//!
//! ```rust
//! use term_ctrl::predefined::*;
//! let format = term_ctrl::support::fmt_supported_stdout();
//! let filter = |seq| { match format { true => seq, false => "" } };
//! println!("normal-text {}possibly-bold-text{} normal-text",
//!          filter(effects::BOLD), filter(RESET));
//! ```

#![no_std]

#[cfg(not(windows))]
extern crate libc;
#[cfg(windows)]
extern crate winapi;

pub mod predefined;
pub mod support;

/// Sequence number definitions (documentation)
pub mod numbers {
    //! # Known number definitions
    //!
    //! - 0: Reset all to defaults
    //!
    //! ## Effects (1-9)
    //!
    //!  - 1: Bold
    //!  - 2: Dim
    //!  - 3: Italic
    //!  - 4: Underline
    //!  - 5: Blink
    //!  - 6: ?
    //!  - 7: "Reverse"
    //!  - 8: Invisible
    //!  - 9: Strike-through
    //!
    //! ## Colors
    //!
    //! There are four sets of seven colors
    //!
    //!  - Text color set 1 (30-37)
    //!  - Background-highlight set 1 (40-47)
    //!  - Text color set 2 (90-97)
    //!  - Background-highlight set 2 (100-107)
    //!
    //! All four sets consist of seven colors, in the following sequence:
    //!
    //!  - x0: Black
    //!  - x1: Red
    //!  - x2: Green
    //!  - x3: Yellow
    //!  - x4: Blue
    //!  - x5: Magenta
    //!  - x6: Cyan
    //!  - x7: White
}

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
