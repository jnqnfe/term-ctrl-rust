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
    pub const NORMAL:    &str = super::RESET;
    /// Bold
    pub const BOLD:      &str = seq!(1);
    /// Dim (aka faint)
    pub const DIM:       &str = seq!(2);
    /// Italic
    pub const ITALIC:    &str = seq!(3);
    /// Underline
    pub const UNDERLINE: &str = seq!(4);
    /// Blink
    pub const BLINK:     &str = seq!(5);
    /* `6` is unused? */
    /// Inverse
    pub const INVERSE:   &str = seq!(7);
    /// Invisible (hidden)
    pub const INVISIBLE: &str = seq!(8);
    /// Strike-through
    pub const STRIKE:    &str = seq!(9);

    /* `10`-`20` are unused? */

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
    }
}

/// Text foreground and background colours
pub mod colours {
    use super::seq;

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
