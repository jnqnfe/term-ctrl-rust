// Copyright 2017 Lyndon Brown
//
// Licensed under the MIT license or the Apache license (version 2.0), at your option. You may not
// copy, modify, or distribute this file except in compliance with said license. You can find copies
// of these licenses either in the LICENSE-MIT and LICENSE-APACHE files, or alternatively at
// <http://opensource.org/licenses/MIT> and <http://www.apache.org/licenses/LICENSE-2.0>
// respectively.

//! Predefined sequence strings

use crate::seq;

pub const RESET: &str = seq!(0);

/// Effects
pub mod effects {
    use super::*;

    pub const BOLD:      &str = seq!(1);
    pub const DIM:       &str = seq!(2);
    pub const ITALIC:    &str = seq!(3);
    pub const UNDERLINE: &str = seq!(4);
    pub const BLINK:     &str = seq!(5);
    pub const REVERSE:   &str = seq!(7);
    pub const INVISIBLE: &str = seq!(8);
    /// Strike-through
    pub const STRIKE:    &str = seq!(9);
}

/// Color set 1
pub mod color1 {
    use super::*;

    pub const BLACK:   &str = seq!(30);
    pub const RED:     &str = seq!(31);
    pub const GREEN:   &str = seq!(32);
    pub const YELLOW:  &str = seq!(33);
    pub const BLUE:    &str = seq!(34);
    pub const MAGENTA: &str = seq!(35);
    pub const CYAN:    &str = seq!(36);
    pub const WHITE:   &str = seq!(37);
}

/// Color set 2
pub mod color2 {
    use super::*;

    pub const BLACK:   &str = seq!(90);
    pub const RED:     &str = seq!(91);
    pub const GREEN:   &str = seq!(92);
    pub const YELLOW:  &str = seq!(93);
    pub const BLUE:    &str = seq!(94);
    pub const MAGENTA: &str = seq!(95);
    pub const CYAN:    &str = seq!(96);
    pub const WHITE:   &str = seq!(97);
}

/// Combined bold + color1
pub mod color1_bold {
    use super::*;

    pub const BLACK:   &str = seq!(30,1);
    pub const RED:     &str = seq!(31,1);
    pub const GREEN:   &str = seq!(32,1);
    pub const YELLOW:  &str = seq!(33,1);
    pub const BLUE:    &str = seq!(34,1);
    pub const MAGENTA: &str = seq!(35,1);
    pub const CYAN:    &str = seq!(36,1);
    pub const WHITE:   &str = seq!(37,1);
}

/// Text background color highlighting, set 1
pub mod highlight1 {
    use super::*;

    pub const BLACK:   &str = seq!(40);
    pub const RED:     &str = seq!(41);
    pub const GREEN:   &str = seq!(42);
    pub const YELLOW:  &str = seq!(43);
    pub const BLUE:    &str = seq!(44);
    pub const MAGENTA: &str = seq!(45);
    pub const CYAN:    &str = seq!(46);
    pub const WHITE:   &str = seq!(47);
}

/// Text background color highlighting, set 2
pub mod highlight2 {
    use super::*;

    pub const BLACK:   &str = seq!(100);
    pub const RED:     &str = seq!(101);
    pub const GREEN:   &str = seq!(102);
    pub const YELLOW:  &str = seq!(103);
    pub const BLUE:    &str = seq!(104);
    pub const MAGENTA: &str = seq!(105);
    pub const CYAN:    &str = seq!(106);
    pub const WHITE:   &str = seq!(107);
}
