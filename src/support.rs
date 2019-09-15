// Copyright 2017 Lyndon Brown
//
// Licensed under the MIT license or the Apache license (version 2.0), at your option. You may not
// copy, modify, or distribute this file except in compliance with said license. You can find copies
// of these licenses either in the LICENSE-MIT and LICENSE-APACHE files, or alternatively at
// <http://opensource.org/licenses/MIT> and <http://www.apache.org/licenses/LICENSE-2.0>
// respectively.

//! Formatted output support helpers

#[cfg(not(windows))]
pub use self::unix::*;

#[cfg(not(windows))]
mod unix {
    use libc;

    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    enum StdPipe {
        //StdIn = libc::STDIN_FILENO as u8, // N/a
        StdOut = libc::STDOUT_FILENO as u8,
        StdErr = libc::STDERR_FILENO as u8,
    }

    impl From<StdPipe> for libc::c_int {
        fn from(p: StdPipe) -> Self { p as libc::c_int }
    }

    /// Are format sequences supported on stdout? Returns `false` if stdout is not a tty.
    ///
    /// Only available on non-Windows.
    #[inline(always)]
    pub fn fmt_supported_stdout() -> bool {
        fmt_supported(StdPipe::StdOut)
    }

    /// Are format sequences supported on stderr? Returns `false` if stderr is not a tty.
    ///
    /// Only available on non-Windows.
    #[inline(always)]
    pub fn fmt_supported_stderr() -> bool {
        fmt_supported(StdPipe::StdErr)
    }

    /// Should I use formatting on stdout?
    ///
    /// Convenience helper, taking user preference, and checking support. Combines them to give an
    /// answer of `true` for yes, `false` for no.
    ///
    /// Only available on non-Windows.
    #[inline(always)]
    pub fn use_fmt_stdout(user_pref: bool) -> bool {
        user_pref && fmt_supported_stdout()
    }

    /// Should I use formatting on stderr?
    ///
    /// Convenience helper, taking user preference, and checking support. Combines them to give an
    /// answer of `true` for yes, `false` for no.
    ///
    /// Only available on non-Windows.
    #[inline(always)]
    pub fn use_fmt_stderr(user_pref: bool) -> bool {
        user_pref && fmt_supported_stderr()
    }


    /// Are format sequences supported on the specified pipe?
    ///
    /// Returns `false` if not a tty.
    #[inline(always)]
    fn fmt_supported(pipe: StdPipe) -> bool {
        unsafe { libc::isatty(libc::c_int::from(pipe)) != 0 }
    }
}
