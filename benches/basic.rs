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

#![feature(test)]
extern crate test;
extern crate tokesies;

#[cfg(test)]
mod tests {
    use tokesies::*;
    use test::Bencher;

    static INPUT: &'static str = "In addition to conventional static typing, before version 0.4, Rust also supported \
     typestates. The typestate system modeled assertions before and after program statements, \
     through use of a special check statement. Discrepancies could be discovered at compile time, \
     rather than when a program was running, as might be the case with assertions in C or C++ \
     code. The typestate concept was not unique to Rust, as it was first introduced in the \
     language NIL. Typestates were removed because in practice they found little use, though the \
     same functionality can still be achieved with branding patterns.

The style changed between \
     0.2, 0.3 and 0.4. Version 0.2 introduced classes for the first time, with version 0.3 adding \
     a number of features including destructors and polymorphism through the use of interfaces. \
     In Rust 0.4, traits were added as a means to provide inheritance; In January 2014, the \
     editor-in-chief of Dr Dobb's, Andrew Binstock, commented on Rust's chances to become a \
     competitor to C++.";

    #[bench]
    fn bench_defaults_case(b: &mut Bencher) {
        b.iter(|| {
            FilteredTokenizer::new(filters::DefaultFilter {}, INPUT).last()
        });
    }

    #[bench]
    fn bench_hash_set(b: &mut Bencher) {
        b.iter(|| {
            FilteredTokenizer::new(
                filters::HashFilter {
                    drop_chars: [' ', '\t', '\n', '\r', '\u{C}'].iter().cloned().collect(),
                    keep_chars: [
                        '#',
                        '!',
                        '\\',
                        '"',
                        '%',
                        '&',
                        '\'',
                        '(',
                        ')',
                        '*',
                        '+',
                        '-',
                        '.',
                        '/',
                        ':',
                        ';',
                        '<',
                        '=',
                        '>',
                        '?',
                        '@',
                        '[',
                        ']',
                        '^',
                        '_',
                        '`',
                        '{',
                        '|',
                        '}',
                        '~',
                        '\u{201C}',
                        '\u{201D}',
                        '\u{2033}',
                    ].iter()
                        .cloned()
                        .collect(),
                },
                INPUT,
            ).last()
        });
    }

    #[bench]
    fn bench_huge_vec(b: &mut Bencher) {
        b.iter(|| {
            let mut filter = filters::VecFilter {
                keep_chars: Vec::new(),
                drop_chars: Vec::new(),
            };

            filter.add_drop(' ');
            filter.add_drop('\t');
            filter.add_drop('\n');
            filter.add_drop('\r');
            filter.add_drop('\u{C}');

            filter.add_keep('#');
            filter.add_keep('!');
            filter.add_keep('\\');
            filter.add_keep('"');
            filter.add_keep('%');
            filter.add_keep('&');
            filter.add_keep('\'');
            filter.add_keep('(');
            filter.add_keep(')');
            filter.add_keep('*');
            filter.add_keep('+');
            filter.add_keep('-');
            filter.add_keep('.');
            filter.add_keep('/');
            filter.add_keep(':');
            filter.add_keep(';');
            filter.add_keep('<');
            filter.add_keep('=');
            filter.add_keep('>');
            filter.add_keep('?');
            filter.add_keep('@');
            filter.add_keep('[');
            filter.add_keep(']');
            filter.add_keep('^');
            filter.add_keep('_');
            filter.add_keep('`');
            filter.add_keep('{');
            filter.add_keep('|');
            filter.add_keep('}');
            filter.add_keep('~');
            filter.add_keep('\u{201C}');
            filter.add_keep('\u{201D}');
            filter.add_keep('\u{2033}');

            FilteredTokenizer::new(filter, INPUT).last()
        });
    }

    #[bench]
    fn bench_whitespace_case(b: &mut Bencher) {
        b.iter(|| {
            FilteredTokenizer::new(filters::WhitespaceFilter {}, INPUT).last()
        });
    }
}
