// Copyright 2017 Lyndon Brown
//
// Licensed under the MIT license or the Apache license (version 2.0), at your option. You may not
// copy, modify, or distribute this file except in compliance with said license. You can find copies
// of these licenses either in the LICENSE-APACHE and LICENSE-MIT files, or alternatively at
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
//! The `term_seq` macro has been provided for constructing a sequence `str`.
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
//! ```rust,ignore
//! const RESET: &str = "\u{1B}[0m";
//! const RED_BOLD: &str = "\u{1B}[31;1m";
//! println!("normal-text {}red-and-bold-text{} normal-text", RED_BOLD, RESET);
//! ```
//!
//! Injecting sequences while being careful of whether or not to do so (of which there are many
//! potential solutions, of varying efficiency)
//!
//! ```rust,ignore
//! const RESET: &str = "\u{1B}[0m";
//! const BOLD: &str = "\u{1B}[1m";
//! let format = term_ctrl::fmt_supported_stdout();
//! let filter = |seq| { match format { true => seq, false => "" } };
//! println!("normal-text {}possibly-bold-text{} normal-text", filter(BOLD), filter(RESET));
//! ```

#![no_std]

#[cfg(not(windows))]
extern crate libc;

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
    //!  - 5: ?
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

/// Macro for defining control sequences
///
/// A sequence can be created with one or more numbers separated by commas.
///
/// Examples:
///
/// ```rust,ignore
/// const BOLD: &str = term_seq!(1);
/// const RED_BOLD_UNDERLINE: &str = term_seq!(31,1,4);
/// ```
#[macro_export]
macro_rules! term_seq {
    ($n1:expr, $($n2:expr),*) => { concat!("\u{1B}[", $n1, $(concat!(";", $n2)),*, "m") };
    ($n:expr) => { concat!("\u{1B}[", $n, "m") };
}

/// Predefined sequences
pub mod predefined {
    pub const RESET: &str = term_seq!(0);

    /// Effects
    pub mod effects {
        pub const BOLD:      &str = term_seq!(1);
        pub const DIM:       &str = term_seq!(2);
        pub const ITALIC:    &str = term_seq!(3);
        pub const UNDERLINE: &str = term_seq!(4);
        pub const REVERSE:   &str = term_seq!(7);
        pub const INVISIBLE: &str = term_seq!(8);
        /// Strike-through
        pub const STRIKE:    &str = term_seq!(9);
    }

    /// Color set 1
    pub mod color1 {
        pub const BLACK:   &str = term_seq!(30);
        pub const RED:     &str = term_seq!(31);
        pub const GREEN:   &str = term_seq!(32);
        pub const YELLOW:  &str = term_seq!(33);
        pub const BLUE:    &str = term_seq!(34);
        pub const MAGENTA: &str = term_seq!(35);
        pub const CYAN:    &str = term_seq!(36);
        pub const WHITE:   &str = term_seq!(37);
    }

    /// Color set 2
    pub mod color2 {
        pub const BLACK:   &str = term_seq!(90);
        pub const RED:     &str = term_seq!(91);
        pub const GREEN:   &str = term_seq!(92);
        pub const YELLOW:  &str = term_seq!(93);
        pub const BLUE:    &str = term_seq!(94);
        pub const MAGENTA: &str = term_seq!(95);
        pub const CYAN:    &str = term_seq!(96);
        pub const WHITE:   &str = term_seq!(97);
    }

    /// Combined bold + color1
    pub mod color1_bold {
        pub const BLACK:   &str = term_seq!(30,1);
        pub const RED:     &str = term_seq!(31,1);
        pub const GREEN:   &str = term_seq!(32,1);
        pub const YELLOW:  &str = term_seq!(33,1);
        pub const BLUE:    &str = term_seq!(34,1);
        pub const MAGENTA: &str = term_seq!(35,1);
        pub const CYAN:    &str = term_seq!(36,1);
        pub const WHITE:   &str = term_seq!(37,1);
    }

    /// Text background color highlighting, set 1
    pub mod highlight1 {
        pub const BLACK:   &str = term_seq!(40);
        pub const RED:     &str = term_seq!(41);
        pub const GREEN:   &str = term_seq!(42);
        pub const YELLOW:  &str = term_seq!(43);
        pub const BLUE:    &str = term_seq!(44);
        pub const MAGENTA: &str = term_seq!(45);
        pub const CYAN:    &str = term_seq!(46);
        pub const WHITE:   &str = term_seq!(47);
    }

    /// Text background color highlighting, set 2
    pub mod highlight2 {
        pub const BLACK:   &str = term_seq!(100);
        pub const RED:     &str = term_seq!(101);
        pub const GREEN:   &str = term_seq!(102);
        pub const YELLOW:  &str = term_seq!(103);
        pub const BLUE:    &str = term_seq!(104);
        pub const MAGENTA: &str = term_seq!(105);
        pub const CYAN:    &str = term_seq!(106);
        pub const WHITE:   &str = term_seq!(107);
    }
}

#[cfg(not(windows))]
pub use unix::*;
#[cfg(windows)]
pub use windows::*;

/// Are format sequences supported on stdout? Returns `false` if stdout is not a tty.
pub fn fmt_supported_stdout() -> bool {
    StdPipe::fmt_supported(StdPipe::StdOut)
}

/// Are format sequences supported on stderr? Returns `false` if stderr is not a tty.
pub fn fmt_supported_stderr() -> bool {
    StdPipe::fmt_supported(StdPipe::StdErr)
}

/// Should I use formatting on stdout?
///
/// Convenience helper, taking user preference, and checking support. Returns `true` for yes,
/// `false` for no.
pub fn use_fmt_stdout(user_pref: bool) -> bool {
    user_pref && StdPipe::fmt_supported(StdPipe::StdOut)
}

/// Should I use formatting on stderr?
///
/// Convenience helper, taking user preference, and checking support. Returns `true` for yes,
/// `false` for no.
pub fn use_fmt_stderr(user_pref: bool) -> bool {
    user_pref && StdPipe::fmt_supported(StdPipe::StdErr)
}

#[cfg(not(windows))]
mod unix {
    use libc;

    #[cfg(not(windows))]
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum StdPipe {
        StdIn = libc::STDIN_FILENO as u8,
        StdOut = libc::STDOUT_FILENO as u8,
        StdErr = libc::STDERR_FILENO as u8,
    }

    impl From<StdPipe> for libc::c_int {
        fn from(p: StdPipe) -> Self { p as libc::c_int }
    }

    impl StdPipe {
        /// Are format sequences supported on `self`? Returns `false` if not a tty or is
        /// `StdPipe::StdIn`.
        pub fn fmt_supported(self) -> bool {
            match self {
                StdPipe::StdIn => false,
                _ => match unsafe { libc::isatty(libc::c_int::from(self)) } {
                    0 => false,
                    _ => true,
                },
            }
        }
    }
}

#[cfg(windows)]
mod windows {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum StdPipe {
        StdIn,
        StdOut,
        StdErr,
    }

    impl StdPipe {
        /// Are format sequences supported on `self`? (Always returns `false` on Windows)
        #[inline(always)]
        pub fn fmt_supported(self) -> bool {
            false
        }
    }
}
