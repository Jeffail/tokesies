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

#[cfg(test)]
mod tests {
    use ::*;

    #[test]
    fn simple_case() {
        let result: Vec<Token> = FilteredTokenizer::new(filters::DefaultFilter {}, "hello world")
            .collect::<Vec<Token>>();

        let expected: Vec<Token> = vec![
            Token::from_str("hello", 0, 0),
            Token::from_str("world", 6, 1),
        ];

        assert_eq!(expected.len(), result.len());
        for (i, exp) in expected.iter().enumerate() {
            let act = result.get(i).unwrap();
            assert_eq!(exp.term(), act.term());
            assert_eq!(exp.start_offset, act.start_offset);
            assert_eq!(exp.position, act.position);
        }
    }

    #[test]
    fn hash_filter_case() {
        let result: Vec<Token> = FilteredTokenizer::new(
            filters::HashFilter {
                drop_chars: [' '].iter().cloned().collect(),
                keep_chars: ['!'].iter().cloned().collect(),
            },
            "hello! !world this!is some text",
        ).collect::<Vec<Token>>();

        let expected: Vec<Token> = vec![
            Token::from_str("hello", 0, 0),
            Token::from_str("!", 5, 1),
            Token::from_str("!", 7, 2),
            Token::from_str("world", 8, 3),
            Token::from_str("this", 14, 4),
            Token::from_str("!", 18, 5),
            Token::from_str("is", 19, 6),
            Token::from_str("some", 22, 7),
            Token::from_str("text", 27, 8),
        ];

        assert_eq!(expected.len(), result.len());
        for (i, exp) in expected.iter().enumerate() {
            let act = result.get(i).unwrap();
            assert_eq!(exp.term(), act.term());
            assert_eq!(exp.start_offset, act.start_offset);
            assert_eq!(exp.position, act.position);
        }
    }

    #[test]
    fn vec_filter_case() {
        let mut filter = filters::VecFilter {
            keep_chars: Vec::new(),
            drop_chars: Vec::new(),
        };

        filter.add_drop(' ');
        filter.add_keep('!');

        let result: Vec<Token> = FilteredTokenizer::new(filter, "hello! !world this!is some text")
            .collect::<Vec<Token>>();

        let expected: Vec<Token> = vec![
            Token::from_str("hello", 0, 0),
            Token::from_str("!", 5, 1),
            Token::from_str("!", 7, 2),
            Token::from_str("world", 8, 3),
            Token::from_str("this", 14, 4),
            Token::from_str("!", 18, 5),
            Token::from_str("is", 19, 6),
            Token::from_str("some", 22, 7),
            Token::from_str("text", 27, 8),
        ];

        assert_eq!(expected.len(), result.len());
        for (i, exp) in expected.iter().enumerate() {
            let act = result.get(i).unwrap();
            assert_eq!(exp.term(), act.term());
            assert_eq!(exp.start_offset, act.start_offset);
            assert_eq!(exp.position, act.position);
        }
    }

    #[test]
    fn keep_tokens_case() {
        let result: Vec<Token> =
            FilteredTokenizer::new(filters::DefaultFilter {}, "hello! !world this!is some text")
                .collect::<Vec<Token>>();

        let expected: Vec<Token> = vec![
            Token::from_str("hello", 0, 0),
            Token::from_str("!", 5, 1),
            Token::from_str("!", 7, 2),
            Token::from_str("world", 8, 3),
            Token::from_str("this", 14, 4),
            Token::from_str("!", 18, 5),
            Token::from_str("is", 19, 6),
            Token::from_str("some", 22, 7),
            Token::from_str("text", 27, 8),
        ];

        assert_eq!(expected.len(), result.len());
        for (i, exp) in expected.iter().enumerate() {
            let act = result.get(i).unwrap();
            assert_eq!(exp.term(), act.term());
            assert_eq!(exp.start_offset, act.start_offset);
            assert_eq!(exp.position, act.position);
        }
    }

    #[test]
    fn whitespace_case() {
        let result: Vec<Token> = FilteredTokenizer::new(
            filters::WhitespaceFilter {},
            "hello world  this    is some text",
        ).collect::<Vec<Token>>();

        let expected: Vec<Token> = vec![
            Token::from_str("hello", 0, 0),
            Token::from_str("world", 6, 1),
            Token::from_str("this", 13, 2),
            Token::from_str("is", 21, 3),
            Token::from_str("some", 24, 4),
            Token::from_str("text", 29, 5),
        ];

        assert_eq!(expected.len(), result.len());
        for (i, exp) in expected.iter().enumerate() {
            let act = result.get(i).unwrap();
            assert_eq!(exp.term(), act.term());
            assert_eq!(exp.start_offset, act.start_offset);
            assert_eq!(exp.position, act.position);
        }
    }
}
