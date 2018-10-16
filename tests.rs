// Copyright 2017 Lyndon Brown
//
// Licensed under the MIT license or the Apache license (version 2.0), at your option. You may not
// copy, modify, or distribute this file except in compliance with said license. You can find copies
// of these licenses either in the LICENSE-APACHE and LICENSE-MIT files, or alternatively at
// <http://opensource.org/licenses/MIT> and <http://www.apache.org/licenses/LICENSE-2.0>
// respectively.

//! Unit tests

#[macro_use]
extern crate term_ctrl;

use term_ctrl::*;

/// Check `term_seq` macro is available and working correctly
#[test]
fn term_seq() {
    let t = term_seq!(1);
    assert_eq!(t, "\u{1B}[1m");
    let t = term_seq!(1,2);
    assert_eq!(t, "\u{1B}[1;2m");
    let t = term_seq!(4,8,15,16,23,42);
    assert_eq!(t, "\u{1B}[4;8;15;16;23;42m");
}

/// Check a few `predefined` sequences seem to be available and as expected
#[test]
fn predefines() {
    assert_eq!(predefined::RESET, "\u{1B}[0m");
    assert_eq!(predefined::effects::BOLD, "\u{1B}[1m");
    assert_eq!(predefined::color1::GREEN, "\u{1B}[32m");
    assert_eq!(predefined::color1_bold::GREEN, "\u{1B}[32;1m");
}

#[cfg(not(windows))]
mod platform {
    use super::*;

    /* On non-Windows:
     * - It should always return false for `StdPipe::StdIn`
     * - Stdout/stderr have dynamic support however, depending on whether connected to a tty
     *
     * How to handle this dynamic situation?
     *  1. We could not bother to compare the returned value, just that the functions are reachable?
     *  2. We could assume that stdout/stderr will always be a tty in running tests?
     *  3. We could try to be fancy and set up both types of pipes, if possible?
     */
    //TODO: Consider switching to option #3?

    #[test]
    fn fmt_supported_shortcuts() {
        // Dynamic, assuming test env has them connected to tty
        assert_eq!(true, fmt_supported_stdout());
        assert_eq!(true, fmt_supported_stderr());
    }

    #[test]
    fn fmt_supported() {
        // Note, stdin obviously never supports formatted output!
        assert_eq!(false, StdPipe::fmt_supported(StdPipe::StdIn));
        // Dynamic, assuming test env has them connected to tty
        assert_eq!(true, StdPipe::fmt_supported(StdPipe::StdOut));
        assert_eq!(true, StdPipe::fmt_supported(StdPipe::StdErr));
    }

    #[test]
    fn fmt_supported_withpref() {
        // Dynamic, assuming test env has them connected to tty
        assert_eq!(true, use_fmt_stdout(true));
        assert_eq!(true, use_fmt_stderr(true));
        assert_eq!(false, use_fmt_stdout(false));
        assert_eq!(false, use_fmt_stderr(false));
    }
}

#[cfg(windows)]
mod platform {
    use super::*;

    #[test]
    fn fmt_supported_shortcuts() {
        assert_eq!(false, fmt_supported_stdout());
        assert_eq!(false, fmt_supported_stderr());
    }

    #[test]
    fn fmt_supported() {
        assert_eq!(false, StdPipe::fmt_supported(StdPipe::StdIn));
        assert_eq!(false, StdPipe::fmt_supported(StdPipe::StdOut));
        assert_eq!(false, StdPipe::fmt_supported(StdPipe::StdErr));
    }

    #[test]
    fn fmt_supported_withpref() {
        assert_eq!(false, use_fmt_stdout(true));
        assert_eq!(false, use_fmt_stderr(true));
        assert_eq!(false, use_fmt_stdout(false));
        assert_eq!(false, use_fmt_stderr(false));
    }
}
