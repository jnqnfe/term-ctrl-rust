// Copyright 2019 Lyndon Brown
//
// Licensed under the MIT license or the Apache license (version 2.0), at your option. You may not
// copy, modify, or distribute this file except in compliance with said license. You can find copies
// of these licenses either in the LICENSE-MIT and LICENSE-APACHE files, or alternatively at
// <http://opensource.org/licenses/MIT> and <http://www.apache.org/licenses/LICENSE-2.0>
// respectively.

//! This example prints a sample text string many times with various effects applied to demonstrate
//! the capabilities of your terminal.

extern crate term_ctrl;

use term_ctrl::predefined::*;
use term_ctrl::predefined::colours::{fg, bg, RESET_BG};

fn main() {
    const SAMPLE_TEXT: &str = "Hello world!";

    println!("Demo text:");
    println!("                    {}", SAMPLE_TEXT);

    println!("With effects:");
    println!("  Bold:             {}{}{}", effects::BOLD,          SAMPLE_TEXT, RESET);
    println!("  Dim:              {}{}{}", effects::DIM,           SAMPLE_TEXT, RESET);
    println!("  Italic:           {}{}{}", effects::ITALIC,        SAMPLE_TEXT, RESET);
    println!("  Underline:        {}{}{}", effects::UNDERLINE,     SAMPLE_TEXT, RESET);
    println!("  Blink:            {}{}{}", effects::BLINK,         SAMPLE_TEXT, RESET);
    println!("  Rapid-blink:      {}{}{}", effects::RAPID_BLINK,   SAMPLE_TEXT, RESET);
    println!("  Inverse:          {}{}{}", effects::INVERSE,       SAMPLE_TEXT, RESET);
    println!("  Invisible:        {}{}{}", effects::INVISIBLE,     SAMPLE_TEXT, RESET);
    println!("  Strike-through:   {}{}{}", effects::STRIKE,        SAMPLE_TEXT, RESET);
    println!("  Fraktur:          {}{}{}", effects::FRAKTUR,       SAMPLE_TEXT, RESET);
    println!("  Double-underline: {}{}{}", effects::DBL_UNDERLINE, SAMPLE_TEXT, RESET);

    println!("Basic foreground colours:");
    println!("                    bg normal       bg black        bg white");
    println!("  Black:            {}{}    {}{}{}    {}{}{}", fg::BLACK,   SAMPLE_TEXT, bg::BLACK, SAMPLE_TEXT, RESET_BG, bg::WHITE, SAMPLE_TEXT, RESET);
    println!("  Red:              {}{}    {}{}{}    {}{}{}", fg::RED,     SAMPLE_TEXT, bg::BLACK, SAMPLE_TEXT, RESET_BG, bg::WHITE, SAMPLE_TEXT, RESET);
    println!("  Green:            {}{}    {}{}{}    {}{}{}", fg::GREEN,   SAMPLE_TEXT, bg::BLACK, SAMPLE_TEXT, RESET_BG, bg::WHITE, SAMPLE_TEXT, RESET);
    println!("  Yellow:           {}{}    {}{}{}    {}{}{}", fg::YELLOW,  SAMPLE_TEXT, bg::BLACK, SAMPLE_TEXT, RESET_BG, bg::WHITE, SAMPLE_TEXT, RESET);
    println!("  Blue:             {}{}    {}{}{}    {}{}{}", fg::BLUE,    SAMPLE_TEXT, bg::BLACK, SAMPLE_TEXT, RESET_BG, bg::WHITE, SAMPLE_TEXT, RESET);
    println!("  Magenta:          {}{}    {}{}{}    {}{}{}", fg::MAGENTA, SAMPLE_TEXT, bg::BLACK, SAMPLE_TEXT, RESET_BG, bg::WHITE, SAMPLE_TEXT, RESET);
    println!("  Cyan:             {}{}    {}{}{}    {}{}{}", fg::CYAN,    SAMPLE_TEXT, bg::BLACK, SAMPLE_TEXT, RESET_BG, bg::WHITE, SAMPLE_TEXT, RESET);
    println!("  White:            {}{}    {}{}{}    {}{}{}", fg::WHITE,   SAMPLE_TEXT, bg::BLACK, SAMPLE_TEXT, RESET_BG, bg::WHITE, SAMPLE_TEXT, RESET);

    println!("Basic foreground colours - bright:");
    println!("                    bg normal       bg black        bg white");
    println!("  Black:            {}{}    {}{}{}    {}{}{}", fg::bright::BLACK,   SAMPLE_TEXT, bg::BLACK, SAMPLE_TEXT, RESET_BG, bg::WHITE, SAMPLE_TEXT, RESET);
    println!("  Red:              {}{}    {}{}{}    {}{}{}", fg::bright::RED,     SAMPLE_TEXT, bg::BLACK, SAMPLE_TEXT, RESET_BG, bg::WHITE, SAMPLE_TEXT, RESET);
    println!("  Green:            {}{}    {}{}{}    {}{}{}", fg::bright::GREEN,   SAMPLE_TEXT, bg::BLACK, SAMPLE_TEXT, RESET_BG, bg::WHITE, SAMPLE_TEXT, RESET);
    println!("  Yellow:           {}{}    {}{}{}    {}{}{}", fg::bright::YELLOW,  SAMPLE_TEXT, bg::BLACK, SAMPLE_TEXT, RESET_BG, bg::WHITE, SAMPLE_TEXT, RESET);
    println!("  Blue:             {}{}    {}{}{}    {}{}{}", fg::bright::BLUE,    SAMPLE_TEXT, bg::BLACK, SAMPLE_TEXT, RESET_BG, bg::WHITE, SAMPLE_TEXT, RESET);
    println!("  Magenta:          {}{}    {}{}{}    {}{}{}", fg::bright::MAGENTA, SAMPLE_TEXT, bg::BLACK, SAMPLE_TEXT, RESET_BG, bg::WHITE, SAMPLE_TEXT, RESET);
    println!("  Cyan:             {}{}    {}{}{}    {}{}{}", fg::bright::CYAN,    SAMPLE_TEXT, bg::BLACK, SAMPLE_TEXT, RESET_BG, bg::WHITE, SAMPLE_TEXT, RESET);
    println!("  White:            {}{}    {}{}{}    {}{}{}", fg::bright::WHITE,   SAMPLE_TEXT, bg::BLACK, SAMPLE_TEXT, RESET_BG, bg::WHITE, SAMPLE_TEXT, RESET);

    println!("Basic background colours:");
    println!("                    fg normal       fg black        fg white");
    println!("  Black:            {}{}    {}{}    {}{}{}", bg::BLACK,   SAMPLE_TEXT, fg::BLACK, SAMPLE_TEXT, fg::WHITE, SAMPLE_TEXT, RESET);
    println!("  Red:              {}{}    {}{}    {}{}{}", bg::RED,     SAMPLE_TEXT, fg::BLACK, SAMPLE_TEXT, fg::WHITE, SAMPLE_TEXT, RESET);
    println!("  Green:            {}{}    {}{}    {}{}{}", bg::GREEN,   SAMPLE_TEXT, fg::BLACK, SAMPLE_TEXT, fg::WHITE, SAMPLE_TEXT, RESET);
    println!("  Yellow:           {}{}    {}{}    {}{}{}", bg::YELLOW,  SAMPLE_TEXT, fg::BLACK, SAMPLE_TEXT, fg::WHITE, SAMPLE_TEXT, RESET);
    println!("  Blue:             {}{}    {}{}    {}{}{}", bg::BLUE,    SAMPLE_TEXT, fg::BLACK, SAMPLE_TEXT, fg::WHITE, SAMPLE_TEXT, RESET);
    println!("  Magenta:          {}{}    {}{}    {}{}{}", bg::MAGENTA, SAMPLE_TEXT, fg::BLACK, SAMPLE_TEXT, fg::WHITE, SAMPLE_TEXT, RESET);
    println!("  Cyan:             {}{}    {}{}    {}{}{}", bg::CYAN,    SAMPLE_TEXT, fg::BLACK, SAMPLE_TEXT, fg::WHITE, SAMPLE_TEXT, RESET);
    println!("  White:            {}{}    {}{}    {}{}{}", bg::WHITE,   SAMPLE_TEXT, fg::BLACK, SAMPLE_TEXT, fg::WHITE, SAMPLE_TEXT, RESET);

    println!("Basic background colours - bright:");
    println!("                    fg normal       fg black        fg white");
    println!("  Black:            {}{}    {}{}    {}{}{}", bg::bright::BLACK,   SAMPLE_TEXT, fg::BLACK, SAMPLE_TEXT, fg::WHITE, SAMPLE_TEXT, RESET);
    println!("  Red:              {}{}    {}{}    {}{}{}", bg::bright::RED,     SAMPLE_TEXT, fg::BLACK, SAMPLE_TEXT, fg::WHITE, SAMPLE_TEXT, RESET);
    println!("  Green:            {}{}    {}{}    {}{}{}", bg::bright::GREEN,   SAMPLE_TEXT, fg::BLACK, SAMPLE_TEXT, fg::WHITE, SAMPLE_TEXT, RESET);
    println!("  Yellow:           {}{}    {}{}    {}{}{}", bg::bright::YELLOW,  SAMPLE_TEXT, fg::BLACK, SAMPLE_TEXT, fg::WHITE, SAMPLE_TEXT, RESET);
    println!("  Blue:             {}{}    {}{}    {}{}{}", bg::bright::BLUE,    SAMPLE_TEXT, fg::BLACK, SAMPLE_TEXT, fg::WHITE, SAMPLE_TEXT, RESET);
    println!("  Magenta:          {}{}    {}{}    {}{}{}", bg::bright::MAGENTA, SAMPLE_TEXT, fg::BLACK, SAMPLE_TEXT, fg::WHITE, SAMPLE_TEXT, RESET);
    println!("  Cyan:             {}{}    {}{}    {}{}{}", bg::bright::CYAN,    SAMPLE_TEXT, fg::BLACK, SAMPLE_TEXT, fg::WHITE, SAMPLE_TEXT, RESET);
    println!("  White:            {}{}    {}{}    {}{}{}", bg::bright::WHITE,   SAMPLE_TEXT, fg::BLACK, SAMPLE_TEXT, fg::WHITE, SAMPLE_TEXT, RESET);

    println!("Misc:");
    println!("  Framed:           {}{}{}", misc::FRAMED,    SAMPLE_TEXT, RESET);
    println!("  Encircled:        {}{}{}", misc::ENCIRCLED, SAMPLE_TEXT, RESET);
    println!("  Overlined:        {}{}{}", misc::OVERLINED, SAMPLE_TEXT, RESET);

    println!("Misc - Ideogram:");
    println!("  Underline:        {}{}{}", misc::ideogram::UNDERLINE,      SAMPLE_TEXT, RESET);
    println!("  Double-underline: {}{}{}", misc::ideogram::DBL_UNDERLINE,  SAMPLE_TEXT, RESET);
    println!("  Overline:         {}{}{}", misc::ideogram::OVERLINE,       SAMPLE_TEXT, RESET);
    println!("  Double-overline:  {}{}{}", misc::ideogram::DBL_OVERLINE,   SAMPLE_TEXT, RESET);
    println!("  Stress-marking:   {}{}{}", misc::ideogram::STRESS_MARKING, SAMPLE_TEXT, RESET);

    println!("Combinations - foreground-color + bold:");
    println!("  Black:            {}{}{}", combinations::fg_bold::BLACK,   SAMPLE_TEXT, RESET);
    println!("  Red:              {}{}{}", combinations::fg_bold::RED,     SAMPLE_TEXT, RESET);
    println!("  Green:            {}{}{}", combinations::fg_bold::GREEN,   SAMPLE_TEXT, RESET);
    println!("  Yellow:           {}{}{}", combinations::fg_bold::YELLOW,  SAMPLE_TEXT, RESET);
    println!("  Blue:             {}{}{}", combinations::fg_bold::BLUE,    SAMPLE_TEXT, RESET);
    println!("  Magenta:          {}{}{}", combinations::fg_bold::MAGENTA, SAMPLE_TEXT, RESET);
    println!("  Cyan:             {}{}{}", combinations::fg_bold::CYAN,    SAMPLE_TEXT, RESET);
    println!("  White:            {}{}{}", combinations::fg_bold::WHITE,   SAMPLE_TEXT, RESET);

    println!("With font selection:");
    println!("  Default:          {}{}{}", fonts::DEFAULT, SAMPLE_TEXT, RESET);
    println!("  Alt #1:           {}{}{}", fonts::ALT1,    SAMPLE_TEXT, RESET);
    println!("  Alt #2:           {}{}{}", fonts::ALT2,    SAMPLE_TEXT, RESET);
    println!("  Alt #3:           {}{}{}", fonts::ALT3,    SAMPLE_TEXT, RESET);
    println!("  Alt #4:           {}{}{}", fonts::ALT4,    SAMPLE_TEXT, RESET);
    println!("  Alt #5:           {}{}{}", fonts::ALT5,    SAMPLE_TEXT, RESET);
    println!("  Alt #6:           {}{}{}", fonts::ALT6,    SAMPLE_TEXT, RESET);
    println!("  Alt #7:           {}{}{}", fonts::ALT7,    SAMPLE_TEXT, RESET);
    println!("  Alt #8:           {}{}{}", fonts::ALT8,    SAMPLE_TEXT, RESET);
    println!("  Alt #9:           {}{}{}", fonts::ALT9,    SAMPLE_TEXT, RESET);
}
