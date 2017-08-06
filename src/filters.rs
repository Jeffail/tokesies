// Copyright (c) 2017 Ashley Jeffs
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

use std::collections::HashSet;

/// A type for filtering chars during tokenization.
pub trait Filter {
    /// Returns a tuple of bool, bool indicating whether the character marks the
    /// end of a token, and whether it should also be collected as a token in
    /// itself, respectively.
    ///
    /// (false, false) - part of a token
    /// (true,  false) - not part of a token and should be discarded
    /// (true,   true) - not part of token but is one in its own right
    fn on_char(&self, c: &char) -> (bool, bool);
}

/// A filter for selecting whitespace characters only.
pub struct WhitespaceFilter;

impl Filter for WhitespaceFilter {
    fn on_char(&self, c: &char) -> (bool, bool) {
        (c.is_whitespace(), false)
    }
}

/// A filter for extracting characters based on two hashmaps, one for keep chars
/// which are collected as tokens, and one for drop chars which will be removed
/// entirely.
pub struct HashFilter {
    pub keep_chars: HashSet<char>,
    pub drop_chars: HashSet<char>,
}

impl Filter for HashFilter {
    fn on_char(&self, c: &char) -> (bool, bool) {
        let is_keep = self.keep_chars.contains(&c);
        (is_keep || self.drop_chars.contains(&c), is_keep)
    }
}

/// A filter that uses a Vec<bool> collection for storing keep and drop
/// characters by index. This means each vector will have a length equal to or
/// greater than the largest character, and lookups can be much faster than a
/// HashSet since we can prefilter on vec length.
pub struct VecFilter {
    pub keep_chars: Vec<bool>,
    pub drop_chars: Vec<bool>,
}

impl VecFilter {
    pub fn add_keep(&mut self, c: char) {
        let s = c as usize;
        let l = self.keep_chars.len();
        if s >= l {
            self.keep_chars.resize(s + 1, false);
        }
        self.keep_chars[s] = true;
    }

    pub fn add_drop(&mut self, c: char) {
        let s = c as usize;
        let l = self.drop_chars.len();
        if s >= l {
            self.drop_chars.resize(s + 1, false);
        }
        self.drop_chars[s] = true;
    }
}

impl Filter for VecFilter {
    fn on_char(&self, c: &char) -> (bool, bool) {
        let s = *c as usize;
        let is_keep = s < self.keep_chars.len() && self.keep_chars[s];
        (
            is_keep || (s < self.drop_chars.len() && self.drop_chars[s]),
            is_keep,
        )
    }
}

/// A filter that uses a prechosen set of default tokenization characters.
pub struct DefaultFilter;

impl Filter for DefaultFilter {
    fn on_char(&self, c: &char) -> (bool, bool) {
        match *c {
            ' ' | '\t' | '\n' | '\r' | '\u{C}' => (true, false),
            '#' | '!' | '\\' | '"' | '%' | '&' | '\'' | '(' | ')' | '*' | '+' | '-' | '.' |
            '/' | ':' | ';' | '<' | '=' | '>' | '?' | '@' | '[' | ']' | '^' | '_' | '`' | '{' |
            '|' | '}' | '~' | '\u{201C}' | '\u{201D}' | '\u{2033}' => (true, true),
            _ => (false, false),
        }
    }
}
