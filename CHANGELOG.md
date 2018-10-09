# ???? (???? ??, ????)

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
