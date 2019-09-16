// Copyright 2017 Lyndon Brown
//
// Licensed under the MIT license or the Apache license (version 2.0), at your option. You may not
// copy, modify, or distribute this file except in compliance with said license. You can find copies
// of these licenses either in the LICENSE-MIT and LICENSE-APACHE files, or alternatively at
// <http://opensource.org/licenses/MIT> and <http://www.apache.org/licenses/LICENSE-2.0>
// respectively.

//! Code constants
//!
//! In string form.
//!
//! If you want whole predefined sequences, see the `predefined` mod. This mode offers simply the
//! numeric codes (as strings), should that be useful to anyone. Note that you cannot give these to
//! the `seq` macro; you will just get an error unfortunately - a limitation of the macro system.

use crate::codes;

// Alias for the non-British-English speakers
pub use self::colours as colors;

/// The start of a control sequence
pub const SEQ_PREFIX: &str = "\u{1B}[";
/// The end of a control sequence
pub const SEQ_POSTFIX: &str = "m";

/// Resets everything to defaults (removes all effects and colours specified)
pub const RESET: &str = codes!(0);

/// Effects
pub mod effects {
    use super::codes;

    /// Alias for reset-all
    pub const NORMAL:      &str = super::RESET;
    /// Bold (increase intensity)
    pub const BOLD:        &str = codes!(1);
    /// Dim (aka faint; decrease intensity)
    pub const DIM:         &str = codes!(2);
    /// Italic
    pub const ITALIC:      &str = codes!(3);
    /// Underline
    pub const UNDERLINE:   &str = codes!(4);
    /// Blink
    pub const BLINK:       &str = codes!(5);
    /// Rapid-blink
    pub const RAPID_BLINK: &str = codes!(6);
    /// Inverse (swap foreground/background colours)
    pub const INVERSE:     &str = codes!(7);
    /// Invisible (hidden)
    pub const INVISIBLE:   &str = codes!(8);
    /// Strike-through
    pub const STRIKE:      &str = codes!(9);

    /* `10`-`19` are used for font selection, covered by the font mod */

    /// Fraktur
    pub const FRAKTUR:       &str = codes!(20);
    /// Double-underline
    pub const DBL_UNDERLINE: &str = codes!(21);

    /// Alias for removing blink
    pub const STEADY:    &str = remove::BLINK;
    /// Alias for removing inverse
    pub const POSITIVE:  &str = remove::INVISIBLE;
    /// Alias for v invisible
    pub const VISIBLE:   &str = remove::INVISIBLE;

    /// Sequences that remove specific effects
    pub mod remove {
        use super::codes;

        /// Removes bold and/or dim
        pub const BOLD_DIM:  &str = codes!(22);
        /// Removes italic
        pub const ITALIC:    &str = codes!(23);
        /// Removes underline
        pub const UNDERLINE: &str = codes!(24);
        /// Removes blink
        pub const BLINK:     &str = codes!(25);
        /* `26` is unused? */
        /// Removes inverse
        pub const INVERSE:   &str = codes!(27);
        /// Removes invisible
        pub const INVISIBLE: &str = codes!(28);
        /// Removes strike-through
        pub const STRIKE:    &str = codes!(29);

        /// Alias for `BOLD_DIM` (reset intensity)
        pub const INTENSITY:  &str = BOLD_DIM;
    }
}

/// Font selection
pub mod fonts {
    use super::codes;

    /// Select the primary (default) font
    pub const DEFAULT: &str = codes!(10);
    /// Select alternate font #1
    pub const ALT1: &str = codes!(11);
    /// Select alternate font #2
    pub const ALT2: &str = codes!(12);
    /// Select alternate font #3
    pub const ALT3: &str = codes!(13);
    /// Select alternate font #4
    pub const ALT4: &str = codes!(14);
    /// Select alternate font #5
    pub const ALT5: &str = codes!(15);
    /// Select alternate font #6
    pub const ALT6: &str = codes!(16);
    /// Select alternate font #7
    pub const ALT7: &str = codes!(17);
    /// Select alternate font #8
    pub const ALT8: &str = codes!(18);
    /// Select alternate font #9
    pub const ALT9: &str = codes!(19);
}

/// Text foreground and background colours
pub mod colours {
    use super::codes;

    /// Reset both foreground and background colours
    pub const RESET:    &str = codes!(39, 49);
    /// Alias for resetting foreground colour
    pub const RESET_FG: &str = fg::RESET;
    /// Alias for resetting background colour
    pub const RESET_BG: &str = bg::RESET;

    /// Text (foreground) colour
    pub mod fg {
        use super::codes;

        pub const BLACK:   &str = codes!(30);
        pub const RED:     &str = codes!(31);
        pub const GREEN:   &str = codes!(32);
        pub const YELLOW:  &str = codes!(33);
        pub const BLUE:    &str = codes!(34);
        pub const MAGENTA: &str = codes!(35);
        pub const CYAN:    &str = codes!(36);
        pub const WHITE:   &str = codes!(37);

        /// Resets foreground colour to default
        pub const RESET:   &str = codes!(39);

        /// Bright variants
        pub mod bright {
            use super::codes;

            pub const BLACK:   &str = codes!(90);
            pub const RED:     &str = codes!(91);
            pub const GREEN:   &str = codes!(92);
            pub const YELLOW:  &str = codes!(93);
            pub const BLUE:    &str = codes!(94);
            pub const MAGENTA: &str = codes!(95);
            pub const CYAN:    &str = codes!(96);
            pub const WHITE:   &str = codes!(97);
        }
    }

    /// Text (background) highlighting colour
    pub mod bg {
        use super::codes;

        pub const BLACK:   &str = codes!(40);
        pub const RED:     &str = codes!(41);
        pub const GREEN:   &str = codes!(42);
        pub const YELLOW:  &str = codes!(43);
        pub const BLUE:    &str = codes!(44);
        pub const MAGENTA: &str = codes!(45);
        pub const CYAN:    &str = codes!(46);
        pub const WHITE:   &str = codes!(47);

        /// Resets background-highlight colour to default
        pub const RESET:   &str = codes!(49);

        /// Bright variants
        pub mod bright {
            use super::codes;

            pub const BLACK:   &str = codes!(100);
            pub const RED:     &str = codes!(101);
            pub const GREEN:   &str = codes!(102);
            pub const YELLOW:  &str = codes!(103);
            pub const BLUE:    &str = codes!(104);
            pub const MAGENTA: &str = codes!(105);
            pub const CYAN:    &str = codes!(106);
            pub const WHITE:   &str = codes!(107);
        }
    }
}

/// Miscellaneous
pub mod misc {
    use super::codes;

    /// Framed
    pub const FRAMED:    &str = codes!(51);
    /// Encircled
    pub const ENCIRCLED: &str = codes!(52);
    /// Overlined
    pub const OVERLINED: &str = codes!(53);

    /// Remove misc effects
    pub mod remove {
        use super::codes;

        /// Remove framed and encircled effects
        pub const FRAMED_ENCIRCLED: &str = codes!(54);
        /// Remove overlined effects
        pub const OVERLINED:        &str = codes!(55);
    }

    /// Ideogram stuff
    pub mod ideogram {
        use super::codes;

        /// Ideogram underline or right side line
        pub const UNDERLINE:      &str = codes!(60);
        /// Ideogram double underline or double line on the right side
        pub const DBL_UNDERLINE:  &str = codes!(61);
        /// Ideogram overline or left side line
        pub const OVERLINE:       &str = codes!(62);
        /// Ideogram double overline or double line on the left side
        pub const DBL_OVERLINE:   &str = codes!(63);
        /// Ideogram stress marking
        pub const STRESS_MARKING: &str = codes!(64);
        /// Ideogram attributes off
        pub const RESET:          &str = codes!(65);
    }
}

/// Combinations
pub mod combinations {
    use super::codes;

    /// Combined bold + text (foreground) colour
    pub mod fg_bold {
        use super::*;

        pub const BLACK:   &str = codes!(30,1);
        pub const RED:     &str = codes!(31,1);
        pub const GREEN:   &str = codes!(32,1);
        pub const YELLOW:  &str = codes!(33,1);
        pub const BLUE:    &str = codes!(34,1);
        pub const MAGENTA: &str = codes!(35,1);
        pub const CYAN:    &str = codes!(36,1);
        pub const WHITE:   &str = codes!(37,1);
    }
}
