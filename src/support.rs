// Copyright 2017 Lyndon Brown
//
// Licensed under the MIT license or the Apache license (version 2.0), at your option. You may not
// copy, modify, or distribute this file except in compliance with said license. You can find copies
// of these licenses either in the LICENSE-MIT and LICENSE-APACHE files, or alternatively at
// <http://opensource.org/licenses/MIT> and <http://www.apache.org/licenses/LICENSE-2.0>
// respectively.

//! Formatted output support helpers

/// Are ANSI format sequences supported on stdout?
///
/// - On Unix this is reliable, returning `true` only if **stdout** is connected to a tty (as
///   opposed to being redirected to a file or other program).
/// - On Windows, this gives an answer on the same principle, however even if returning `true` such
///   that we know we are connected to a terminal, this does not mean that the terminal actually
///   supports ANSI control sequences. Before Windows 10 you should assume not. On Windows 10+ you
///   must use the [`enable_ansi_support`] function to turn on support.
///
/// [`enable_ansi_support`]: fn.enable_ansi_support.html
#[inline(always)]
pub fn fmt_supported_stdout() -> bool {
    atty::is(atty::Stream::Stdout)
}

/// Are ANSI format sequences supported on stderr?
///
/// - On Unix this is reliable, returning `true` only if **stderr** is connected to a tty (as
///   opposed to being redirected to a file or other program).
/// - On Windows, this gives an answer on the same principle, however even if returning `true` such
///   that we know we are connected to a terminal, this does not mean that the terminal actually
///   supports ANSI control sequences. Before Windows 10 you should assume not. On Windows 10+ you
///   must use the [`enable_ansi_support`] function to turn on support.
///
/// [`enable_ansi_support`]: fn.enable_ansi_support.html
#[inline(always)]
pub fn fmt_supported_stderr() -> bool {
    atty::is(atty::Stream::Stderr)
}

/// Should I use formatting on stdout?
///
/// Convenience helper, taking user preference, and checking support. Combines them to give an
/// answer of `true` for yes, `false` for no.
#[inline(always)]
pub fn use_fmt_stdout(user_pref: bool) -> bool {
    user_pref && fmt_supported_stdout()
}

/// Should I use formatting on stderr?
///
/// Convenience helper, taking user preference, and checking support. Combines them to give an
/// answer of `true` for yes, `false` for no.
#[inline(always)]
pub fn use_fmt_stderr(user_pref: bool) -> bool {
    user_pref && fmt_supported_stderr()
}

/*
  Copied and slightly modified from the `ansi_term` crate (MIT licensed).
*/
/// Enables ANSI code support on Windows 10.
///
/// This uses Windows API calls to alter the properties of the console that
/// the program is running in.
///
/// https://msdn.microsoft.com/en-us/library/windows/desktop/mt638032(v=vs.85).aspx
///
/// Returns a `Result` with the Windows error code if unsuccessful.
#[cfg(windows)]
pub fn enable_ansi_support() -> Result<(), u32> {
    use winapi::um::processenv::GetStdHandle;
    use winapi::um::errhandlingapi::GetLastError;
    use winapi::um::consoleapi::{GetConsoleMode, SetConsoleMode};

    const STD_OUT_HANDLE: u32 = -11i32 as u32;
    const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;

    macro_rules! handle_error {
        ( $result:expr ) => { match GetLastError() { 0 => Ok($result), e => Err(e), } };
    }

    unsafe {
        // https://docs.microsoft.com/en-us/windows/console/getstdhandle
        let std_out_handle = handle_error!(GetStdHandle(STD_OUT_HANDLE))?;

        // https://docs.microsoft.com/en-us/windows/console/getconsolemode
        let mut console_mode: u32 = 0;
        handle_error!(GetConsoleMode(std_out_handle, &mut console_mode))?;

        // VT processing not already enabled?
        if console_mode & ENABLE_VIRTUAL_TERMINAL_PROCESSING == 0 {
            // https://docs.microsoft.com/en-us/windows/console/setconsolemode
            handle_error!(SetConsoleMode(std_out_handle,
                console_mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING))?;
        }
    }
    Ok(())
}
