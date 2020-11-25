# [unreleased]

 * Another attempt to fix the macro link

# 0.7.3 (November 24th, 2020)

 * Trying again to fix mistake with macro link in previous changes (relying upon docs.rs processing
   of new crates to test the change since my distro only has rustc v1.47 for now).

# 0.7.2 (November 24th, 2020)

 * Fix mistake with macro link in previous changes

# 0.7.1 (November 24th, 2020)

 * Previous change was incomplete

# 0.7.0 (November 24th, 2020)

 * Adjusted documentation to take advantage of the better link generation provided in the latest
   rust compiler (version 1.48). This means that the minimum rust version needed is 1.48 to have
   working links within the documentation, though the crate is still usable with older versions, so
   the MSRV is not being bumped.

# 0.6.1 (November 24th, 2020)

 * Trivial non-functional tweaks, not worth listing

# 0.6.0 (September 17th, 2019)

 * Re-organised the predefines
 * Added various additional predefines
 * Renamed the `REVERSE` predefine to the more correct term `INVERSE`
 * Extended, corrected and otherwise further improved the documentation
 * Added helper macros for generating code groupings for use in sequences, including the generic
   `codes`, along with those specifically for specifying 256-colour and RGB colour palette
   selections: `c256_fg`, `c256_bg`, `rgb_fg` and `rgb_bg`.
 * Added a `codes` mod. While the `predefines` mod offers various common items in full sequence
   form, this new mod offers the codes without being embedded in the full sequence pattern. This may
   be useful to some for constructing custom combinations via named constants instead of raw numeric
   literals. The prefix and postfix parts of the sequence pattern are given also. Note that it is
   not possible to use these names constants with `seq` since the Rust macro construction would take
   the literal name of the constant, rather than what the constant points to.
 * Added a `demo` example, which outputs some sample text with various forms of formatting applied,
   to demonstrate the possibilities and the capabilities of your terminal.

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
