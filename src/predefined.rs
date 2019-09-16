// Copyright 2017 Lyndon Brown
//
// Licensed under the MIT license or the Apache license (version 2.0), at your option. You may not
// copy, modify, or distribute this file except in compliance with said license. You can find copies
// of these licenses either in the LICENSE-MIT and LICENSE-APACHE files, or alternatively at
// <http://opensource.org/licenses/MIT> and <http://www.apache.org/licenses/LICENSE-2.0>
// respectively.

//! Predefined sequence strings

use crate::seq;

// Alias for the non-British-English speakers
pub use self::colours as colors;

/// Resets everything to defaults (removes all effects and colours specified)
pub const RESET: &str = seq!(0);

/// Effects
pub mod effects {
    use super::seq;

    /// Alias for reset-all
    pub const NORMAL:      &str = super::RESET;
    /// Bold (increase intensity)
    pub const BOLD:        &str = seq!(1);
    /// Dim (aka faint; decrease intensity)
    pub const DIM:         &str = seq!(2);
    /// Italic
    pub const ITALIC:      &str = seq!(3);
    /// Underline
    pub const UNDERLINE:   &str = seq!(4);
    /// Blink
    pub const BLINK:       &str = seq!(5);
    /// Rapid-blink
    pub const RAPID_BLINK: &str = seq!(6);
    /// Inverse (swap foreground/background colours)
    pub const INVERSE:     &str = seq!(7);
    /// Invisible (hidden)
    pub const INVISIBLE:   &str = seq!(8);
    /// Strike-through
    pub const STRIKE:      &str = seq!(9);

    /* `10`-`19` are used for font selection, covered by the font mod */

    /// Fraktur
    pub const FRAKTUR:       &str = seq!(20);
    /// Double-underline
    pub const DBL_UNDERLINE: &str = seq!(21);

    /// Alias for removing blink
    pub const STEADY:    &str = remove::BLINK;
    /// Alias for removing inverse
    pub const POSITIVE:  &str = remove::INVISIBLE;
    /// Alias for v invisible
    pub const VISIBLE:   &str = remove::INVISIBLE;

    /// Sequences that remove specific effects
    pub mod remove {
        use super::seq;

        /// Removes bold and/or dim
        pub const BOLD_DIM:  &str = seq!(22);
        /// Removes italic
        pub const ITALIC:    &str = seq!(23);
        /// Removes underline
        pub const UNDERLINE: &str = seq!(24);
        /// Removes blink
        pub const BLINK:     &str = seq!(25);
        /* `26` is unused? */
        /// Removes inverse
        pub const INVERSE:   &str = seq!(27);
        /// Removes invisible
        pub const INVISIBLE: &str = seq!(28);
        /// Removes strike-through
        pub const STRIKE:    &str = seq!(29);

        /// Alias for `BOLD_DIM` (reset intensity)
        pub const INTENSITY:  &str = BOLD_DIM;
    }
}

/// Font selection
pub mod fonts {
    use super::seq;

    /// Select the primary (default) font
    pub const DEFAULT: &str = seq!(10);
    /// Select alternate font #1
    pub const ALT1: &str = seq!(11);
    /// Select alternate font #2
    pub const ALT2: &str = seq!(12);
    /// Select alternate font #3
    pub const ALT3: &str = seq!(13);
    /// Select alternate font #4
    pub const ALT4: &str = seq!(14);
    /// Select alternate font #5
    pub const ALT5: &str = seq!(15);
    /// Select alternate font #6
    pub const ALT6: &str = seq!(16);
    /// Select alternate font #7
    pub const ALT7: &str = seq!(17);
    /// Select alternate font #8
    pub const ALT8: &str = seq!(18);
    /// Select alternate font #9
    pub const ALT9: &str = seq!(19);
}

/// Text foreground and background colours
pub mod colours {
    use super::seq;

    /// Reset both foreground and background colours
    pub const RESET:    &str = seq!(39, 49);
    /// Alias for resetting foreground colour
    pub const RESET_FG: &str = fg::RESET;
    /// Alias for resetting background colour
    pub const RESET_BG: &str = bg::RESET;

    /// Text (foreground) colour
    pub mod fg {
        use super::seq;

        pub const BLACK:   &str = seq!(30);
        pub const RED:     &str = seq!(31);
        pub const GREEN:   &str = seq!(32);
        pub const YELLOW:  &str = seq!(33);
        pub const BLUE:    &str = seq!(34);
        pub const MAGENTA: &str = seq!(35);
        pub const CYAN:    &str = seq!(36);
        pub const WHITE:   &str = seq!(37);

        /// Resets foreground colour to default
        pub const RESET:   &str = seq!(39);

        /// Bright variants
        pub mod bright {
            use super::seq;

            pub const BLACK:   &str = seq!(90);
            pub const RED:     &str = seq!(91);
            pub const GREEN:   &str = seq!(92);
            pub const YELLOW:  &str = seq!(93);
            pub const BLUE:    &str = seq!(94);
            pub const MAGENTA: &str = seq!(95);
            pub const CYAN:    &str = seq!(96);
            pub const WHITE:   &str = seq!(97);
        }
    }

    /// Text (background) highlighting colour
    pub mod bg {
        use super::seq;

        pub const BLACK:   &str = seq!(40);
        pub const RED:     &str = seq!(41);
        pub const GREEN:   &str = seq!(42);
        pub const YELLOW:  &str = seq!(43);
        pub const BLUE:    &str = seq!(44);
        pub const MAGENTA: &str = seq!(45);
        pub const CYAN:    &str = seq!(46);
        pub const WHITE:   &str = seq!(47);

        /// Resets background-highlight colour to default
        pub const RESET:   &str = seq!(49);

        /// Bright variants
        pub mod bright {
            use super::seq;

            pub const BLACK:   &str = seq!(100);
            pub const RED:     &str = seq!(101);
            pub const GREEN:   &str = seq!(102);
            pub const YELLOW:  &str = seq!(103);
            pub const BLUE:    &str = seq!(104);
            pub const MAGENTA: &str = seq!(105);
            pub const CYAN:    &str = seq!(106);
            pub const WHITE:   &str = seq!(107);
        }
    }
}

/// Miscellaneous
pub mod misc {
    use super::seq;

    /// Framed
    pub const FRAMED:    &str = seq!(51);
    /// Encircled
    pub const ENCIRCLED: &str = seq!(52);
    /// Overlined
    pub const OVERLINED: &str = seq!(53);

    /// Remove misc effects
    pub mod remove {
        use super::seq;

        /// Remove framed and encircled effects
        pub const FRAMED_ENCIRCLED: &str = seq!(54);
        /// Remove overlined effects
        pub const OVERLINED:        &str = seq!(55);
    }

    /// Ideogram stuff
    pub mod ideogram {
        use super::seq;

        /// Ideogram underline or right side line
        pub const UNDERLINE:      &str = seq!(60);
        /// Ideogram double underline or double line on the right side
        pub const DBL_UNDERLINE:  &str = seq!(61);
        /// Ideogram overline or left side line
        pub const OVERLINE:       &str = seq!(62);
        /// Ideogram double overline or double line on the left side
        pub const DBL_OVERLINE:   &str = seq!(63);
        /// Ideogram stress marking
        pub const STRESS_MARKING: &str = seq!(64);
        /// Ideogram attributes off
        pub const RESET:          &str = seq!(65);
    }
}

/// Combinations
pub mod combinations {
    use super::seq;

    /// Combined bold + text (foreground) colour
    pub mod fg_bold {
        use super::seq;

        pub const BLACK:   &str = seq!(30,1);
        pub const RED:     &str = seq!(31,1);
        pub const GREEN:   &str = seq!(32,1);
        pub const YELLOW:  &str = seq!(33,1);
        pub const BLUE:    &str = seq!(34,1);
        pub const MAGENTA: &str = seq!(35,1);
        pub const CYAN:    &str = seq!(36,1);
        pub const WHITE:   &str = seq!(37,1);
    }
}
