// Copyright 2017 Lyndon Brown
//
// Licensed under the MIT license or the Apache license (version 2.0), at your option. You may not
// copy, modify, or distribute this file except in compliance with said license. You can find copies
// of these licenses either in the LICENSE-MIT and LICENSE-APACHE files, or alternatively at
// <http://opensource.org/licenses/MIT> and <http://www.apache.org/licenses/LICENSE-2.0>
// respectively.

//! ANSI terminal text formatting assistant
//!
//! Some virtual-terminals (ANSI ones being the focus here) have a feature of allowing command-line
//! programs executed in them to request that certain limited formatting effects such as colour or
//! boldness be applied to portions of the text the program outputs for them to display. This is
//! achieved through the program embedding special “ANSI control” byte sequences into that text
//! output. This can prove very useful for instance in highlighting error notices.
//!
//! The purpose of this crate is to offer functionality to assist in generating and using such
//! control sequences.
//!
//! Note that such formatting should only be used when the `stdout` stream (or `stderr` as
//! applicable) is connected to an actual terminal that will process and use them (you wouldn't want
//! these control sequences to exist in output that the user is having the terminal redirect into a
//! text file or another program as then it would just be treated as a part of the text and lead to
//! garbled output and confusion). Some utility functions are provided within the [support mod] to
//! assist you with determining appropriateness of using formatted output.
//!
//! # Predefined sequences
//!
//! There is no strict need to actually understand how to construct such control sequences to make
//! use of the formatting they offer; many predefined sequences are available for your use in the
//! [predefined mod](predefined/index.html).
//!
//! To make use of these simply inject them into the right points of a to-be-printed string via
//! format arguments, as demonstrated below. Remember not to forget to make use of a reset after the
//! text to be formatted. For instance:
//!
//! ```rust
//! use term_ctrl::predefined::{RESET, colours::fg::RED};
//! println!("{}Error:{} You made an error!", RED, RESET);
//! ```
//!
//! In the example just given, the text “<span style="color:#c00">Error:</span>” will be highlighted
//! in red. Of course as mentioned earlier, you should only use the colour sequences if output is
//! actually connected to a terminal (and that the terminal has not redirected it, so do not assume
//! output to stdout is okay). Here the example is modified to use a helper to check suitability and
//! a filter to control use:
//!
//! ```rust
//! use term_ctrl::predefined::{RESET, colours::fg::RED};
//! let format = term_ctrl::support::fmt_supported_stdout();
//! let filter = |seq| { match format { true => seq, false => "" } };
//! println!("{}Error:{} You made an error!", filter(RED), filter(RESET));
//! ```
//!
//! Note, when resetting to normal, be sure to always use the proper reset sequence. Do not make the
//! mistake of setting text colour to black and presuming that this achieves the same thing; it does
//! not. (Consider that some people have black text on a white background in their terminal, whilst
//! others use the opposite! Black text on a black background does not work very well!).
//!
//! # Constructing sequences
//!
//! Although understanding the sequences is not required, there are however benefits to be gained
//! from it, since for instance not every possible combination can reasonably be provided in
//! predefined form, and you may be able to achieve greater efficiency by constructing a custom one
//! rather than using multiple predefines in sequence.
//!
//! Let’s take a quick look at the sequence itself, then we can move on to discuss a macro which
//! helps make custom construction very easy.
//!
//! ## The sequence pattern
//!
//! The sequence pattern consists of four component parts:
//!
//! <table style="width:auto">
//!     <tr><td>1</td><td>ESC ('\u{1B}')</td><td>The escape (␛) char, Unicode 0x1B</td></tr>
//!     <tr><td>2</td><td>'['</td><td>An opening bracket char</td></tr>
//!     <tr><td>3</td><td colspan="2">One or more numbers using a semi-colon (';') as a separator</td></tr>
//!     <tr><td>4</td><td>'m'</td><td>A lower-case letter m</td></tr>
//! </table>
//!
//! In other words a pattern of: `"\u{1B}[` <em><codes\></em> `m"`
//!
//! Every control sequence fits this template, and it is the set of numbers included in it which
//! determine the precise formatting nature of the request. As a quick example, the sequence of
//! `"\u{1B}[31;1m"` specifies two numbers, `31` which corresponds to red text, and `1` which
//! corresponds to bold text (the set of available number codes are discussed shortly).
//!
//! Typically each of the individual numbers given corresponds to a particular effect, however as we
//! will see shortly there are a couple of exceptions where multiple numbers are used.
//!
//! Before we continue, understand that the effects specified are applied in sequence and are
//! applied cumulatively both with respect to previous effects in the same sequence and also to any
//! existing effects still in effect from a previous sequence. There are specific codes for removing
//! specific effects or resetting foreground or background-highlight colours. There is also the
//! catch-all code `0` for resetting everything to normal.
//!
//! ## Macro construction
//!
//! As just mentioned, a macro (called [`seq`]) is provided to assist with constructing sequences.
//! All you have to do is provide it with a list of decimal numbers and it will construct a string
//! of the above pattern that contains them. An example:
//!
//! ```rust
//! use term_ctrl::seq;
//! assert_eq!("\u{1B}[1;2;3m", seq!(1, 2, 3));
//! ```
//!
//! Note that you are not restricted to pure numeric literals, string literals work also:
//!
//! ```rust
//! use term_ctrl::seq;
//! assert_eq!("\u{1B}[1;2;3m", seq!("1", "2", "3"));
//! assert_eq!("\u{1B}[1;2;3m", seq!("1;2", 3));
//! ```
//!
//! Convenience macros are also provided for constructing 256-colour and RGB colour code sets for
//! use in a sequence (they do not generate the full sequence, just a multi-code set of numbers to
//! use in a sequence).
//!
//! ## Number code chart
//!
//! The following is a guide to the available number codes and what they correspond to. In most
//! cases a single number code corresponds to a single effect, however there are also those that
//! require a sequence of multiple numbers (with the normal semi-colon separator).
//!
//! ### Effects
//!
//! <table>
//!     <thead>
//!         <tr><th>Code</th><th>Effect</th><th>Code</th><th>Effect</th></tr>
//!     </thead>
//!     <tbody>
//!         <tr><td>0</td><td>Normal (reset)</td><td>20</td><td><em>unused?</em></td></tr>
//!         <tr><td>1</td><td>Bold</td><td>21</td><td>Double-underline</td></tr>
//!         <tr><td>2</td><td>Dim (faint)</td><td>22</td><td>Remove bold and dim</td></tr>
//!         <tr><td>3</td><td>Italic</td><td>23</td><td>Remove italic</td></tr>
//!         <tr><td>4</td><td>Underlined</td><td>24</td><td>Remove underline</td></tr>
//!         <tr><td>5</td><td>Blink</td><td>25</td><td>Remove blink (aka “steady”)</td></tr>
//!         <tr><td>6</td><td><em>unused?</em></td><td>26</td><td><em>unused?</em></td></tr>
//!         <tr><td>7</td><td>“Inverse”</td><td>27</td><td>Remove inverse effect (aka “positive”)</td></tr>
//!         <tr><td>8</td><td>Invisible (hidden)</td><td>28</td><td>Remove invisible effect (aka “visible”)</td></tr>
//!         <tr><td>9</td><td>Strike-through</td><td>29</td><td>Remove strike-through effect</td></tr>
//!     </tbody>
//! </table>
//!
//! Code zero (`0`) resets all effects and colour selections to defaults.
//!
//! Note that while code `21` has an additional form of underline effect, codes `22`-`29` provide
//! codes for removing specific effects, with `22` removing both bold and dim effects together,
//! otherwise the last digit corresponds to the effect removed (e.g. `3` and `23` relate to italic).
//!
//! ### Basic colours
//!
//! The basic colour palettes consist of a simple set of eight colours for each of foreground and
//! background-highlight uses. There are also “bright” variants.
//!
//! <table>
//!     <thead>
//!         <tr><th>Colour</th><th>Foreground</th><th>Background</th><th>Bright foreground</th><th>Bright background</th></tr>
//!     </thead>
//!     <tbody>
//!         <tr><td>Black</td><td>30</td><td>40</td><td>90</td><td>100</td></tr>
//!         <tr><td>Red</td><td>31</td><td>41</td><td>91</td><td>101</td></tr>
//!         <tr><td>Green</td><td>32</td><td>42</td><td>92</td><td>102</td></tr>
//!         <tr><td>Yellow</td><td>33</td><td>43</td><td>93</td><td>103</td></tr>
//!         <tr><td>Blue</td><td>34</td><td>44</td><td>94</td><td>104</td></tr>
//!         <tr><td>Magenta</td><td>35</td><td>45</td><td>95</td><td>105</td></tr>
//!         <tr><td>Cyan</td><td>36</td><td>46</td><td>96</td><td>106</td></tr>
//!         <tr><td>White</td><td>37</td><td>47</td><td>97</td><td>107</td></tr>
//!         <tr><td><em>Extended</em></td><td>38</td><td>48</td><td><em>n/a</em></td><td><em>n/a</em></td></tr>
//!         <tr><td><em>Default</em> (reset)</td><td>39</td><td>49</td><td><em>n/a</em></td><td><em>n/a</em></td></tr>
//!     </tbody>
//! </table>
//!
//! The “extended” codes `38` and `48` are used to start multi-code “extended” colour palette
//! selections, as discussed shortly.
//!
//! The “default” codes `39` and `49` are used to reset foreground and background-highlight colours
//! respectively to defaults, and apply to bright and extended colouring also.
//!
//! ### Extended range colours
//!
//! Wider choice of colours than above is made available as *extended* colours through use of a
//! sequence of codes (using the normal semi-colon separator).
//!
//! There are two ranges available, an 8-bit (256 colour) range, and a 24-bit RGB range. Both begin
//! with either `38` to choose text (foreground colouring) or `48` for background-highlight
//! colouring. This is followed by `5` for specifying 256-colour or `2` for specifying RGB colour.
//! With 256-colour a final third number with a value in the range of 0-255 selects the specific
//! colour. With RGB, three numbers must be given, each corresponding to red, green and blue
//! respectively, and each also being of a value in the range 0-255.
//!
//! [See here][Xterm_256color_chart.svg] for a 256-colour chart.
//!
//! As an example, `seq!(38,5,238)` changes the colour of text (not background) since it starts with
//! `38`, it is providing a 256-colour palette selection (the `5`), and is specifically selecting
//! colour `238` from that palette.
//!
//! As another example, `seq!(48,2,180,15,70)` changes the colour of the text background since it
//! starts with `48`, it is providing an RGB colour (the `2`), and is then followed with RGB values
//! of `180` for red, `15` for green and `70` for blue.
//!
//! [support mod]: support/index.html
//! [`seq`]: macro.seq.html
//! [Xterm_256color_chart.svg]: https://upload.wikimedia.org/wikipedia/commons/1/15/Xterm_256color_chart.svg

#![no_std]

extern crate atty;
#[cfg(windows)]
extern crate winapi;

mod macros;
pub mod predefined;
pub mod support;
