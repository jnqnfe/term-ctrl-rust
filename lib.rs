//! Console/terminal text control

// Copyright (c) 2017 Lyndon Brown
//
// Licensed under the MIT license or the Apache license (Version 2.0), at your option. You may not
// copy, modify, or distribute this file except in compliance with said license. You can find copies
// of these licenses either in the LICENSE-APACHE and LICENSE-MIT files, or alternatively at
// <http://opensource.org/licenses/MIT> and <http://www.apache.org/licenses/LICENSE-2.0>
// respectively.

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
//! This crate provides tools and definitions to assist in using these effects.
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
//! No list of effect ↔ number mapping is given here, just common sequences.
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
//! let format = term_ctrl::color_supported();
//! let filter = |seq| { match format { true => seq, false => "" } };
//! println!("normal-text {}possibly-bold-text{} normal-text", filter(BOLD), filter(RESET));
//! ```

#[cfg(not(windows))]
extern crate libc;

// Control
pub static RESET:     &str = "\u{1B}[0m";
pub static BOLD:      &str = "\u{1B}[1m";
pub static DIM:       &str = "\u{1B}[2m";
pub static ITALIC:    &str = "\u{1B}[3m";
pub static UNDERLINE: &str = "\u{1B}[4m";
pub static REVERSE:   &str = "\u{1B}[7m";
pub static INVISIBLE: &str = "\u{1B}[8m";
pub static STRIKE:    &str = "\u{1B}[9m"; // strike-through

// Color
pub static BLACK:   &str = "\u{1B}[30m";
pub static RED:     &str = "\u{1B}[31m";
pub static GREEN:   &str = "\u{1B}[32m";
pub static YELLOW:  &str = "\u{1B}[33m";
pub static BLUE:    &str = "\u{1B}[34m";
pub static MAGENTA: &str = "\u{1B}[35m";
pub static CYAN:    &str = "\u{1B}[36m";
pub static WHITE:   &str = "\u{1B}[37m";

pub static BLACK_2:   &str = "\u{1B}[90m";
pub static RED_2:     &str = "\u{1B}[91m";
pub static GREEN_2:   &str = "\u{1B}[92m";
pub static YELLOW_2:  &str = "\u{1B}[93m";
pub static BLUE_2:    &str = "\u{1B}[94m";
pub static MAGENTA_2: &str = "\u{1B}[95m";
pub static CYAN_2:    &str = "\u{1B}[96m";
pub static WHITE_2:   &str = "\u{1B}[97m";

// Bold + color
pub static BLACK_B:   &str = "\u{1B}[30;1m";
pub static RED_B:     &str = "\u{1B}[31;1m";
pub static GREEN_B:   &str = "\u{1B}[32;1m";
pub static YELLOW_B:  &str = "\u{1B}[33;1m";
pub static BLUE_B:    &str = "\u{1B}[34;1m";
pub static MAGENTA_B: &str = "\u{1B}[35;1m";
pub static CYAN_B:    &str = "\u{1B}[36;1m";
pub static WHITE_B:   &str = "\u{1B}[37;1m";

// Text background color highlighting
pub static H_BLACK:   &str = "\u{1B}[40m";
pub static H_RED:     &str = "\u{1B}[41m";
pub static H_GREEN:   &str = "\u{1B}[42m";
pub static H_YELLOW:  &str = "\u{1B}[43m";
pub static H_BLUE:    &str = "\u{1B}[44m";
pub static H_MAGENTA: &str = "\u{1B}[45m";
pub static H_CYAN:    &str = "\u{1B}[46m";
pub static H_WHITE:   &str = "\u{1B}[47m";

pub static H_BLACK_2:   &str = "\u{1B}[100m";
pub static H_RED_2:     &str = "\u{1B}[101m";
pub static H_GREEN_2:   &str = "\u{1B}[102m";
pub static H_YELLOW_2:  &str = "\u{1B}[103m";
pub static H_BLUE_2:    &str = "\u{1B}[104m";
pub static H_MAGENTA_2: &str = "\u{1B}[105m";
pub static H_CYAN_2:    &str = "\u{1B}[106m";
pub static H_WHITE_2:   &str = "\u{1B}[107m";

#[cfg(windows)]
pub fn color_supported() -> bool {
    false
}
#[cfg(not(windows))]
pub fn color_supported() -> bool {
    if unsafe { libc::isatty(libc::STDOUT_FILENO) } == 0 {
        return false;
    }
    true
}
