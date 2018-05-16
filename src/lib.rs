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

//! Tokesies is a Rust crate for tokenizing strings using filters that can
//! select characters as delimiters to be either removed or extracted as a
//! token themselves.
//!
//! Since tokesies filter implementations can carry state it is possible that
//! smart tokenizers can be created.
//!
//! # Examples
//! ```
//! use tokesies::*;
//!
//! // Using the default filter
//! let line = "hello!world, this is some_text";
//! let tokens = FilteredTokenizer::new(filters::DefaultFilter{}, line).collect::<Vec<Token>>();
//!
//! // tokens: ["hello", "!", "world", ",", "this", "is", "some", "_", "text"]
//!
//! assert_eq!(tokens.get(0).unwrap().term(), "hello");
//! ```
//!
//! # Using custom filter
//! ```
//! use tokesies::*;
//!
//! pub struct MyFilter;
//!
//! impl filters::Filter for MyFilter {
//!     fn on_char(&self, c: &char) -> (bool, bool) {
//!         match *c {
//!             ' ' => (true, false),
//!             ',' => (true, true),
//!             _ => (false, false),
//!         }
//!     }
//! }
//!
//! let line = "hello!world, this is some_text";
//! let tokens = FilteredTokenizer::new(MyFilter{}, line).collect::<Vec<Token>>();
//!
//! // tokens: ["hello!world", ",", "this", "is", "some_text"]
//!
//! assert_eq!(tokens.get(0).unwrap().term(), "hello!world");
//! ```

mod test;

pub mod filters;

use std::fmt;
use std::borrow::Cow;

/// Contains context for a token extracted from an input.
pub struct Token<'a> {
    /// The content of the extracted token.
    pub term: Cow<'a, str>,

    /// The absolute offset of the token in chars.
    pub start_offset: usize,

    /// The token position.
    pub position: usize,
}

impl<'a> Token<'a> {
    pub fn from_str(term: &'a str, start_offset: usize, position: usize) -> Self {
        Token {
            term: Cow::Borrowed(term),
            start_offset: start_offset,
            position: position,
        }
    }

    pub fn term(&self) -> &str {
        self.term.as_ref()
    }
}

impl<'a> fmt::Debug for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.term())
    }
}

/// Implementation of Tokenizer that extracts based on a provided Filter
/// implementation.
pub struct FilteredTokenizer<'a, T: filters::Filter> {
    filter: T,
    input: &'a str,
    byte_offset: usize,
    char_offset: usize,
    position: usize,
}

impl<'a, T: filters::Filter> FilteredTokenizer<'a, T> {
    pub fn new(filter: T, input: &'a str) -> Self {
        FilteredTokenizer {
            filter: filter,
            input: input,
            byte_offset: 0,
            char_offset: 0,
            position: 0,
        }
    }
}

impl<'a, T: filters::Filter> Iterator for FilteredTokenizer<'a, T> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        let mut skipped_bytes = 0;
        let mut skipped_chars = 0;

        let filter = &self.filter;

        // cidx, bidx is the char and byte index from the last found separator
        for (cidx, bidx, c, is_keep) in
            self.input[self.byte_offset..]
                .char_indices()
                .enumerate()
                // Remove any drop codes entirely
                .filter_map(|(ci, (bi, c))| {
                    let (is_filtered, is_keep) = filter.on_char(&c);
                    if is_filtered {
                        Some((ci, bi, c, is_keep))
                    } else {
                        None
                    }
                })
        {
            let char_len = c.len_utf8();

            // If we found a separator but had no text beforehand simply move
            // our counters to the new position.
            if cidx == skipped_chars {
                self.char_offset += 1;
                self.byte_offset += char_len;
                skipped_bytes += char_len;
                skipped_chars += 1;
                if is_keep {
                    let slice = &self.input[self.byte_offset - char_len..
                                                self.byte_offset + bidx + char_len - skipped_bytes];
                    let token = Token::from_str(slice, self.char_offset - 1, self.position);
                    self.position += 1;
                    return Some(token);
                }
                continue;
            }

            let slice = &self.input[self.byte_offset..self.byte_offset + bidx - skipped_bytes];
            let token = Token::from_str(slice, self.char_offset, self.position);

            self.char_offset += slice.chars().count();
            self.position += 1;
            self.byte_offset += bidx - skipped_bytes;
            if !is_keep {
                self.char_offset += 1;
                self.byte_offset += char_len;
            }
            return Some(token);
        }

        if self.byte_offset < self.input.len() {
            let slice = &self.input[self.byte_offset..];
            let token = Token::from_str(slice, self.char_offset, self.position);
            self.byte_offset = self.input.len();
            Some(token)
        } else {
            None
        }
    }
}
