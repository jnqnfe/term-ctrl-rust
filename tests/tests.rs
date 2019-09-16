// Copyright 2017 Lyndon Brown
//
// Licensed under the MIT license or the Apache license (version 2.0), at your option. You may not
// copy, modify, or distribute this file except in compliance with said license. You can find copies
// of these licenses either in the LICENSE-MIT and LICENSE-APACHE files, or alternatively at
// <http://opensource.org/licenses/MIT> and <http://www.apache.org/licenses/LICENSE-2.0>
// respectively.

//! Unit tests

extern crate term_ctrl;

use term_ctrl::*;

/// Check `seq` macro is available and working correctly
#[test]
fn term_seq() {
    assert_eq!(seq!(1), "\u{1B}[1m");
    assert_eq!(seq!(1,2), "\u{1B}[1;2m");
    assert_eq!(seq!(4,8,15,16,23,42), "\u{1B}[4;8;15;16;23;42m");
}

/// Check a few `predefined` sequences seem to be available and as expected
#[test]
fn predefines() {
    assert_eq!(predefined::RESET, "\u{1B}[0m");
    assert_eq!(predefined::effects::BOLD, "\u{1B}[1m");
    assert_eq!(predefined::colours::fg::GREEN, "\u{1B}[32m");
    assert_eq!(predefined::combinations::fg_bold::GREEN, "\u{1B}[32;1m");
}

#[cfg(not(windows))]
mod platform {
    use super::*;

    #[test]
    fn fmt_supported() {
        // Assuming test env has them connected to tty
        assert_eq!(true, support::fmt_supported_stdout());
        assert_eq!(true, support::fmt_supported_stderr());
    }

    #[test]
    fn fmt_supported_withpref() {
        // Assuming test env has them connected to tty
        assert_eq!(true, support::use_fmt_stdout(true));
        assert_eq!(true, support::use_fmt_stderr(true));
        assert_eq!(false, support::use_fmt_stdout(false));
        assert_eq!(false, support::use_fmt_stderr(false));
    }
}
