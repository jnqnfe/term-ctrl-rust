# 0.5.0 (September 15th, 2019)

 * Reworked the format-supported helpers on top of the `atty` crate, thus bringing better cross
   platform support.
 * Added a Windows `enable_ansi_support` function, taken from `ansi_term`
 * Renamed the `term_seq` macro to more simply `seq`
 * Reorganised a bit
 * Re-wrote the documentation
 * Various other odd little tweaks really not worth documenting

# 0.4.1 (October 27th, 2018)

 * Documentation example tweaks

# 0.4.0 (October 16th, 2018)

 * Introduced unit tests
 * Used a trait to help ensure that the `fmt_supported` method on `StdPipe` always has the same
   prototype across platforms.
 * Removed a stray redundant platform conditional
 * Enabled some doc tests
 * Reorganised copyright header position

# 0.3 (October 10th, 2018)

 * Grouped predefined sequences in `predefined` mod

# 0.2 (October 9th, 2018)

 * Improved crate documentation
 * Now avoids `libc` for windows builds, where it was unused
 * Grouped codes into sub-modules
 * Changed codes from statics to constants
 * Added and used a macro for creating sequences
 * Renamed `color_supported` to `fmt_supported`
 * Expanded pipe type coverage
 * Declared `no_std` since we do not need `std`

# 0.1 (January 24th, 2018)

 * Original version, previously bundled with the `gong` crate & repo, now extracted into its own
