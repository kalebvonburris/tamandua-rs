//! Generated tests for the Lexer module using rstest
//! Auto-generated from the Lua test suite

#[cfg(test)]
mod lexer_tests {
    use rstest::rstest;
    use std::time::Duration;
    use tamandua_rs::lexer::{Lexeme, Lexer, LexingError};

    fn assert_lex_eq(input: &str, expected: &[(Lexeme, String)], given: &[(Lexeme, String)]) {
        if expected != given {
            panic!(
                "\nInput: {:?}\n\nExpected: {:#?}\n\nGiven: {:#?}",
                input, expected, given
            );
        }
    }

    #[rstest]
    #[case(r#"a"#, vec![(Lexeme::Identifier, r#"a"#.to_string())])]
    #[case(r#" a"#, vec![(Lexeme::Identifier, r#"a"#.to_string())])]
    #[case(r#"_"#, vec![(Lexeme::Identifier, r#"_"#.to_string())])]
    #[case(r#" _"#, vec![(Lexeme::Identifier, r#"_"#.to_string())])]
    #[case(r#"bx"#, vec![(Lexeme::Identifier, r#"bx"#.to_string())])]
    #[case(r#"b3"#, vec![(Lexeme::Identifier, r#"b3"#.to_string())])]
    #[case(r#"_n"#, vec![(Lexeme::Identifier, r#"_n"#.to_string())])]
    #[case(r#"_4"#, vec![(Lexeme::Identifier, r#"_4"#.to_string())])]
    #[case(r#"abc_39xyz"#, vec![(Lexeme::Identifier, r#"abc_39xyz"#.to_string())])]
    #[case(r#"abc eol_3"#, vec![(Lexeme::Identifier, r#"abc"#.to_string()), (Lexeme::Identifier, r#"eol_3"#.to_string())])]
    #[case(r#"a  "#, vec![(Lexeme::Identifier, r#"a"#.to_string())])]
    #[case(r#"a#"#, vec![(Lexeme::Identifier, r#"a"#.to_string())])]
    #[case(r#"a #"#, vec![(Lexeme::Identifier, r#"a"#.to_string())])]
    #[case(r#"#
a"#, vec![(Lexeme::Identifier, r#"a"#.to_string())])]
    #[case(r#"#
 a"#, vec![(Lexeme::Identifier, r#"a"#.to_string())])]
    #[case(r#"ab"#, vec![(Lexeme::Identifier, r#"ab"#.to_string())])]
    #[case(r#"_chr"#, vec![(Lexeme::Identifier, r#"_chr"#.to_string())])]
    #[timeout(Duration::from_secs(1))]
    fn test_identifiers(#[case] input: &str, #[case] expected: Vec<(Lexeme, String)>) {
        let mut lexer = Lexer::new(input.to_string());
        let result = lexer.lex_input();
        assert_lex_eq(input, &expected, &result);
    }

    #[rstest]
    #[case(r#"chr"#, vec![(Lexeme::Keyword, r#"chr"#.to_string())])]
    #[case(r#"else"#, vec![(Lexeme::Keyword, r#"else"#.to_string())])]
    #[case(r#"elsif"#, vec![(Lexeme::Keyword, r#"elsif"#.to_string())])]
    #[case(r#"func"#, vec![(Lexeme::Keyword, r#"func"#.to_string())])]
    #[case(r#"if"#, vec![(Lexeme::Keyword, r#"if"#.to_string())])]
    #[case(r#"print"#, vec![(Lexeme::Keyword, r#"print"#.to_string())])]
    #[case(r#"println"#, vec![(Lexeme::Keyword, r#"println"#.to_string())])]
    #[case(r#"readint"#, vec![(Lexeme::Keyword, r#"readint"#.to_string())])]
    #[case(r#"return"#, vec![(Lexeme::Keyword, r#"return"#.to_string())])]
    #[case(r#"rnd"#, vec![(Lexeme::Keyword, r#"rnd"#.to_string())])]
    #[case(r#"while"#, vec![(Lexeme::Keyword, r#"while"#.to_string())])]
    #[timeout(Duration::from_secs(1))]
    fn test_keywords(#[case] input: &str, #[case] expected: Vec<(Lexeme, String)>) {
        let mut lexer = Lexer::new(input.to_string());
        let result = lexer.lex_input();
        assert_lex_eq(input, &expected, &result);
    }

    #[rstest]
    #[case(r#"funcc"#, vec![(Lexeme::Identifier, r#"funcc"#.to_string())])]
    #[case(r#"chr2"#, vec![(Lexeme::Identifier, r#"chr2"#.to_string())])]
    #[case(r#"Print"#, vec![(Lexeme::Identifier, r#"Print"#.to_string())])]
    #[case(r#"elsE"#, vec![(Lexeme::Identifier, r#"elsE"#.to_string())])]
    #[case(r#"IF"#, vec![(Lexeme::Identifier, r#"IF"#.to_string())])]
    #[case(r#"elsi"#, vec![(Lexeme::Identifier, r#"elsi"#.to_string())])]
    #[case(r#"printl"#, vec![(Lexeme::Identifier, r#"printl"#.to_string())])]
    #[case(r#"els if"#, vec![(Lexeme::Identifier, r#"els"#.to_string()), (Lexeme::Keyword, r#"if"#.to_string())])]
    #[case(r#"re#
turn"#, vec![(Lexeme::Identifier, r#"re"#.to_string()), (Lexeme::Identifier, r#"turn"#.to_string())])]
    #[case(r#"whi2le"#, vec![(Lexeme::Identifier, r#"whi2le"#.to_string())])]
    #[case(r#"pri_nt"#, vec![(Lexeme::Identifier, r#"pri_nt"#.to_string())])]
    #[case(r#"ifreturn"#, vec![(Lexeme::Identifier, r#"ifreturn"#.to_string())])]
    #[timeout(Duration::from_secs(1))]
    fn test_keyword_variations(#[case] input: &str, #[case] expected: Vec<(Lexeme, String)>) {
        let mut lexer = Lexer::new(input.to_string());
        let result = lexer.lex_input();
        assert_lex_eq(input, &expected, &result);
    }

    #[rstest]
    #[case(r#"3"#, vec![(Lexeme::NumericLiteral, r#"3"#.to_string())])]
    #[case(r#"3a"#, vec![(Lexeme::NumericLiteral, r#"3"#.to_string()), (Lexeme::Identifier, r#"a"#.to_string())])]
    #[case(r#"123456"#, vec![(Lexeme::NumericLiteral, r#"123456"#.to_string())])]
    #[case(r#".123456"#, vec![(Lexeme::Punctuation, r#"."#.to_string()), (Lexeme::NumericLiteral, r#"123456"#.to_string())])]
    #[case(r#"123456."#, vec![(Lexeme::NumericLiteral, r#"123456"#.to_string()), (Lexeme::Punctuation, r#"."#.to_string())])]
    #[case(r#"123.456"#, vec![(Lexeme::NumericLiteral, r#"123"#.to_string()), (Lexeme::Punctuation, r#"."#.to_string()), (Lexeme::NumericLiteral, r#"456"#.to_string())])]
    #[case(r#"1.2.3"#, vec![(Lexeme::NumericLiteral, r#"1"#.to_string()), (Lexeme::Punctuation, r#"."#.to_string()), (Lexeme::NumericLiteral, r#"2"#.to_string()), (Lexeme::Punctuation, r#"."#.to_string()), (Lexeme::NumericLiteral, r#"3"#.to_string())])]
    #[case(r#"123 456"#, vec![(Lexeme::NumericLiteral, r#"123"#.to_string()), (Lexeme::NumericLiteral, r#"456"#.to_string())])]
    #[timeout(Duration::from_secs(1))]
    fn test_numeric_literals(#[case] input: &str, #[case] expected: Vec<(Lexeme, String)>) {
        let mut lexer = Lexer::new(input.to_string());
        let result = lexer.lex_input();
        assert_lex_eq(input, &expected, &result);
    }

    #[rstest]
    #[case(r#"+123456"#, vec![(Lexeme::Operator, r#"+"#.to_string()), (Lexeme::NumericLiteral, r#"123456"#.to_string())])]
    #[case(r#"+.123456"#, vec![(Lexeme::Operator, r#"+"#.to_string()), (Lexeme::Punctuation, r#"."#.to_string()), (Lexeme::NumericLiteral, r#"123456"#.to_string())])]
    #[case(r#"+123456."#, vec![(Lexeme::Operator, r#"+"#.to_string()), (Lexeme::NumericLiteral, r#"123456"#.to_string()), (Lexeme::Punctuation, r#"."#.to_string())])]
    #[case(r#"+123.456"#, vec![(Lexeme::Operator, r#"+"#.to_string()), (Lexeme::NumericLiteral, r#"123"#.to_string()), (Lexeme::Punctuation, r#"."#.to_string()), (Lexeme::NumericLiteral, r#"456"#.to_string())])]
    #[case(r#"+1.2.3"#, vec![(Lexeme::Operator, r#"+"#.to_string()), (Lexeme::NumericLiteral, r#"1"#.to_string()), (Lexeme::Punctuation, r#"."#.to_string()), (Lexeme::NumericLiteral, r#"2"#.to_string()), (Lexeme::Punctuation, r#"."#.to_string()), (Lexeme::NumericLiteral, r#"3"#.to_string())])]
    #[case(r#"-123456"#, vec![(Lexeme::Operator, r#"-"#.to_string()), (Lexeme::NumericLiteral, r#"123456"#.to_string())])]
    #[case(r#"-.123456"#, vec![(Lexeme::Operator, r#"-"#.to_string()), (Lexeme::Punctuation, r#"."#.to_string()), (Lexeme::NumericLiteral, r#"123456"#.to_string())])]
    #[case(r#"-123456."#, vec![(Lexeme::Operator, r#"-"#.to_string()), (Lexeme::NumericLiteral, r#"123456"#.to_string()), (Lexeme::Punctuation, r#"."#.to_string())])]
    #[case(r#"-123.456"#, vec![(Lexeme::Operator, r#"-"#.to_string()), (Lexeme::NumericLiteral, r#"123"#.to_string()), (Lexeme::Punctuation, r#"."#.to_string()), (Lexeme::NumericLiteral, r#"456"#.to_string())])]
    #[case(r#"-1.2.3"#, vec![(Lexeme::Operator, r#"-"#.to_string()), (Lexeme::NumericLiteral, r#"1"#.to_string()), (Lexeme::Punctuation, r#"."#.to_string()), (Lexeme::NumericLiteral, r#"2"#.to_string()), (Lexeme::Punctuation, r#"."#.to_string()), (Lexeme::NumericLiteral, r#"3"#.to_string())])]
    #[case(r#"+-123456"#, vec![(Lexeme::Operator, r#"+"#.to_string()), (Lexeme::Operator, r#"-"#.to_string()), (Lexeme::NumericLiteral, r#"123456"#.to_string())])]
    #[case(r#"-+123456"#, vec![(Lexeme::Operator, r#"-"#.to_string()), (Lexeme::Operator, r#"+"#.to_string()), (Lexeme::NumericLiteral, r#"123456"#.to_string())])]
    #[timeout(Duration::from_secs(1))]
    fn test_numeric_with_signs(#[case] input: &str, #[case] expected: Vec<(Lexeme, String)>) {
        let mut lexer = Lexer::new(input.to_string());
        let result = lexer.lex_input();
        assert_lex_eq(input, &expected, &result);
    }

    #[rstest]
    #[case(r#"123e456"#, vec![(Lexeme::NumericLiteral, r#"123e456"#.to_string())])]
    #[case(r#"123e+456"#, vec![(Lexeme::NumericLiteral, r#"123e+456"#.to_string())])]
    #[case(r#"123e-456"#, vec![(Lexeme::NumericLiteral, r#"123"#.to_string()), (Lexeme::Identifier, r#"e"#.to_string()), (Lexeme::Operator, r#"-"#.to_string()), (Lexeme::NumericLiteral, r#"456"#.to_string())])]
    #[case(r#"+123e456"#, vec![(Lexeme::Operator, r#"+"#.to_string()), (Lexeme::NumericLiteral, r#"123e456"#.to_string())])]
    #[case(r#"+123e+456"#, vec![(Lexeme::Operator, r#"+"#.to_string()), (Lexeme::NumericLiteral, r#"123e+456"#.to_string())])]
    #[case(r#"+123e-456"#, vec![(Lexeme::Operator, r#"+"#.to_string()), (Lexeme::NumericLiteral, r#"123"#.to_string()), (Lexeme::Identifier, r#"e"#.to_string()), (Lexeme::Operator, r#"-"#.to_string()), (Lexeme::NumericLiteral, r#"456"#.to_string())])]
    #[case(r#"-123e456"#, vec![(Lexeme::Operator, r#"-"#.to_string()), (Lexeme::NumericLiteral, r#"123e456"#.to_string())])]
    #[case(r#"-123e+456"#, vec![(Lexeme::Operator, r#"-"#.to_string()), (Lexeme::NumericLiteral, r#"123e+456"#.to_string())])]
    #[case(r#"-123e-456"#, vec![(Lexeme::Operator, r#"-"#.to_string()), (Lexeme::NumericLiteral, r#"123"#.to_string()), (Lexeme::Identifier, r#"e"#.to_string()), (Lexeme::Operator, r#"-"#.to_string()), (Lexeme::NumericLiteral, r#"456"#.to_string())])]
    #[case(r#"123E456"#, vec![(Lexeme::NumericLiteral, r#"123E456"#.to_string())])]
    #[case(r#"123E+456"#, vec![(Lexeme::NumericLiteral, r#"123E+456"#.to_string())])]
    #[case(r#"123E-456"#, vec![(Lexeme::NumericLiteral, r#"123"#.to_string()), (Lexeme::Identifier, r#"E"#.to_string()), (Lexeme::Operator, r#"-"#.to_string()), (Lexeme::NumericLiteral, r#"456"#.to_string())])]
    #[case(r#"+123E456"#, vec![(Lexeme::Operator, r#"+"#.to_string()), (Lexeme::NumericLiteral, r#"123E456"#.to_string())])]
    #[case(r#"+123E+456"#, vec![(Lexeme::Operator, r#"+"#.to_string()), (Lexeme::NumericLiteral, r#"123E+456"#.to_string())])]
    #[case(r#"+123E-456"#, vec![(Lexeme::Operator, r#"+"#.to_string()), (Lexeme::NumericLiteral, r#"123"#.to_string()), (Lexeme::Identifier, r#"E"#.to_string()), (Lexeme::Operator, r#"-"#.to_string()), (Lexeme::NumericLiteral, r#"456"#.to_string())])]
    #[case(r#"-123E456"#, vec![(Lexeme::Operator, r#"-"#.to_string()), (Lexeme::NumericLiteral, r#"123E456"#.to_string())])]
    #[case(r#"-123E+456"#, vec![(Lexeme::Operator, r#"-"#.to_string()), (Lexeme::NumericLiteral, r#"123E+456"#.to_string())])]
    #[case(r#"-123E-456"#, vec![(Lexeme::Operator, r#"-"#.to_string()), (Lexeme::NumericLiteral, r#"123"#.to_string()), (Lexeme::Identifier, r#"E"#.to_string()), (Lexeme::Operator, r#"-"#.to_string()), (Lexeme::NumericLiteral, r#"456"#.to_string())])]
    #[timeout(Duration::from_secs(1))]
    fn test_scientific_notation(#[case] input: &str, #[case] expected: Vec<(Lexeme, String)>) {
        let mut lexer = Lexer::new(input.to_string());
        let result = lexer.lex_input();
        assert_lex_eq(input, &expected, &result);
    }

    #[rstest]
    #[case(r#"e"#, vec![(Lexeme::Identifier, r#"e"#.to_string())])]
    #[case(r#"E"#, vec![(Lexeme::Identifier, r#"E"#.to_string())])]
    #[case(r#"e3"#, vec![(Lexeme::Identifier, r#"e3"#.to_string())])]
    #[case(r#"E3"#, vec![(Lexeme::Identifier, r#"E3"#.to_string())])]
    #[case(r#"e+3"#, vec![(Lexeme::Identifier, r#"e"#.to_string()), (Lexeme::Operator, r#"+"#.to_string()), (Lexeme::NumericLiteral, r#"3"#.to_string())])]
    #[case(r#"E+3"#, vec![(Lexeme::Identifier, r#"E"#.to_string()), (Lexeme::Operator, r#"+"#.to_string()), (Lexeme::NumericLiteral, r#"3"#.to_string())])]
    #[case(r#"1e3"#, vec![(Lexeme::NumericLiteral, r#"1e3"#.to_string())])]
    #[case(r#"123e"#, vec![(Lexeme::NumericLiteral, r#"123"#.to_string()), (Lexeme::Identifier, r#"e"#.to_string())])]
    #[case(r#"123E"#, vec![(Lexeme::NumericLiteral, r#"123"#.to_string()), (Lexeme::Identifier, r#"E"#.to_string())])]
    #[case(r#"123ee"#, vec![(Lexeme::NumericLiteral, r#"123"#.to_string()), (Lexeme::Identifier, r#"ee"#.to_string())])]
    #[case(r#"123Ee"#, vec![(Lexeme::NumericLiteral, r#"123"#.to_string()), (Lexeme::Identifier, r#"Ee"#.to_string())])]
    #[case(r#"123eE"#, vec![(Lexeme::NumericLiteral, r#"123"#.to_string()), (Lexeme::Identifier, r#"eE"#.to_string())])]
    #[case(r#"123EE"#, vec![(Lexeme::NumericLiteral, r#"123"#.to_string()), (Lexeme::Identifier, r#"EE"#.to_string())])]
    #[case(r#"1.2e34"#, vec![(Lexeme::NumericLiteral, r#"1"#.to_string()), (Lexeme::Punctuation, r#"."#.to_string()), (Lexeme::NumericLiteral, r#"2e34"#.to_string())])]
    #[case(r#"12e3.4"#, vec![(Lexeme::NumericLiteral, r#"12e3"#.to_string()), (Lexeme::Punctuation, r#"."#.to_string()), (Lexeme::NumericLiteral, r#"4"#.to_string())])]
    #[case(r#"123e+"#, vec![(Lexeme::NumericLiteral, r#"123"#.to_string()), (Lexeme::Identifier, r#"e"#.to_string()), (Lexeme::Operator, r#"+"#.to_string())])]
    #[case(r#"123E+"#, vec![(Lexeme::NumericLiteral, r#"123"#.to_string()), (Lexeme::Identifier, r#"E"#.to_string()), (Lexeme::Operator, r#"+"#.to_string())])]
    #[timeout(Duration::from_secs(1))]
    fn test_scientific_notation_edge_cases(
        #[case] input: &str,
        #[case] expected: Vec<(Lexeme, String)>,
    ) {
        let mut lexer = Lexer::new(input.to_string());
        let result = lexer.lex_input();
        assert_lex_eq(input, &expected, &result);
    }

    #[rstest]
    #[case(r#"''"#, vec![(Lexeme::StringLiteral, r#"''"#.to_string())])]
    #[case(r#""""#, vec![(Lexeme::StringLiteral, r#""""#.to_string())])]
    #[case(r#"'a'"#, vec![(Lexeme::StringLiteral, r#"'a'"#.to_string())])]
    #[case(r#""b""#, vec![(Lexeme::StringLiteral, r#""b""#.to_string())])]
    #[case(r#"'abc eol'"#, vec![(Lexeme::StringLiteral, r#"'abc eol'"#.to_string())])]
    #[case(r#""The quick brown fox.""#, vec![(Lexeme::StringLiteral, r#""The quick brown fox.""#.to_string())])]
    #[case(r#"'aa"bb'"#, vec![(Lexeme::StringLiteral, r#"'aa"bb'"#.to_string())])]
    #[case(r#""cc'dd""#, vec![(Lexeme::StringLiteral, r#""cc'dd""#.to_string())])]
    #[case(r#""\a""#, vec![(Lexeme::StringLiteral, r#""\a""#.to_string())])]
    #[timeout(Duration::from_secs(1))]
    fn test_string_literals(#[case] input: &str, #[case] expected: Vec<(Lexeme, String)>) {
        let mut lexer = Lexer::new(input.to_string());
        let result = lexer.lex_input();
        assert_lex_eq(input, &expected, &result);
    }

    #[rstest]
    #[case(r#"'aabbcc"#, vec![(Lexeme::Malformed(LexingError::MalformedStringLiteral), r#"'aabbcc"#.to_string())])]
    #[case(r#"'aabbcc""#, vec![(Lexeme::Malformed(LexingError::MalformedStringLiteral), r#"'aabbcc""#.to_string())])]
    #[case(r#"'aabbcc
"#, vec![(Lexeme::Malformed(LexingError::MalformedStringLiteral), r#"'aabbcc"#.to_string())])]
    #[case(r#""aabbcc"#, vec![(Lexeme::Malformed(LexingError::MalformedStringLiteral), r#""aabbcc"#.to_string())])]
    #[case(r#""aabbcc'"#, vec![(Lexeme::Malformed(LexingError::MalformedStringLiteral), r#""aabbcc'"#.to_string())])]
    #[case(r#""aabbcc
"#, vec![(Lexeme::Malformed(LexingError::MalformedStringLiteral), r#""aabbcc"#.to_string())])]
    #[case(r#""aabbcc
d"#, vec![(Lexeme::Malformed(LexingError::MalformedStringLiteral), r#""aabbcc"#.to_string()), (Lexeme::Identifier, r#"d"#.to_string())])]
    #[case(r#"'"'"'""#, vec![(Lexeme::StringLiteral, r#"'"'"#.to_string()), (Lexeme::StringLiteral, r#""'""#.to_string())])]
    #[timeout(Duration::from_secs(1))]
    fn test_malformed_strings(#[case] input: &str, #[case] expected: Vec<(Lexeme, String)>) {
        let mut lexer = Lexer::new(input.to_string());
        let result = lexer.lex_input();
        assert_lex_eq(input, &expected, &result);
    }

    #[rstest]
    #[case(r#"!"#, vec![(Lexeme::Operator, r#"!"#.to_string())])]
    #[case(r#"&&"#, vec![(Lexeme::Operator, r#"&&"#.to_string())])]
    #[case(r#"||"#, vec![(Lexeme::Operator, r#"||"#.to_string())])]
    #[case(r#"=="#, vec![(Lexeme::Operator, r#"=="#.to_string())])]
    #[case(r#"!="#, vec![(Lexeme::Operator, r#"!="#.to_string())])]
    #[case(r#"<"#, vec![(Lexeme::Operator, r#"<"#.to_string())])]
    #[case(r#"<="#, vec![(Lexeme::Operator, r#"<="#.to_string())])]
    #[case(r#">"#, vec![(Lexeme::Operator, r#">"#.to_string())])]
    #[case(r#">="#, vec![(Lexeme::Operator, r#">="#.to_string())])]
    #[case(r#"+"#, vec![(Lexeme::Operator, r#"+"#.to_string())])]
    #[case(r#"-"#, vec![(Lexeme::Operator, r#"-"#.to_string())])]
    #[case(r#"*"#, vec![(Lexeme::Operator, r#"*"#.to_string())])]
    #[case(r#"/"#, vec![(Lexeme::Operator, r#"/"#.to_string())])]
    #[case(r#"%"#, vec![(Lexeme::Operator, r#"%"#.to_string())])]
    #[case(r#"["#, vec![(Lexeme::Operator, r#"["#.to_string())])]
    #[case(r#"]"#, vec![(Lexeme::Operator, r#"]"#.to_string())])]
    #[case(r#"="#, vec![(Lexeme::Operator, r#"="#.to_string())])]
    #[case(r#"++"#, vec![(Lexeme::Operator, r#"++"#.to_string())])]
    #[case(r#"--"#, vec![(Lexeme::Operator, r#"--"#.to_string())])]
    #[timeout(Duration::from_secs(1))]
    fn test_operators(#[case] input: &str, #[case] expected: Vec<(Lexeme, String)>) {
        let mut lexer = Lexer::new(input.to_string());
        let result = lexer.lex_input();
        assert_lex_eq(input, &expected, &result);
    }

    #[rstest]
    #[case(r#"+-++--"#, vec![(Lexeme::Operator, r#"+"#.to_string()), (Lexeme::Operator, r#"-"#.to_string()), (Lexeme::Operator, r#"++"#.to_string()), (Lexeme::Operator, r#"--"#.to_string())])]
    #[case(r#"+++++"#, vec![(Lexeme::Operator, r#"++"#.to_string()), (Lexeme::Operator, r#"++"#.to_string()), (Lexeme::Operator, r#"+"#.to_string())])]
    #[case(r#"-----"#, vec![(Lexeme::Operator, r#"--"#.to_string()), (Lexeme::Operator, r#"--"#.to_string()), (Lexeme::Operator, r#"-"#.to_string())])]
    #[case(r#"====="#, vec![(Lexeme::Operator, r#"=="#.to_string()), (Lexeme::Operator, r#"=="#.to_string()), (Lexeme::Operator, r#"="#.to_string())])]
    #[case(r#"=<<=="#, vec![(Lexeme::Operator, r#"="#.to_string()), (Lexeme::Operator, r#"<"#.to_string()), (Lexeme::Operator, r#"<="#.to_string()), (Lexeme::Operator, r#"="#.to_string())])]
    #[case(r#"**/ "#, vec![(Lexeme::Operator, r#"*"#.to_string()), (Lexeme::Operator, r#"*"#.to_string()), (Lexeme::Operator, r#"/"#.to_string())])]
    #[case(r#"= ="#, vec![(Lexeme::Operator, r#"="#.to_string()), (Lexeme::Operator, r#"="#.to_string())])]
    #[case(r#"+++1+"#, vec![(Lexeme::Operator, r#"++"#.to_string()), (Lexeme::Operator, r#"+"#.to_string()), (Lexeme::NumericLiteral, r#"1"#.to_string()), (Lexeme::Operator, r#"+"#.to_string())])]
    #[case(r#"---2-"#, vec![(Lexeme::Operator, r#"--"#.to_string()), (Lexeme::Operator, r#"-"#.to_string()), (Lexeme::NumericLiteral, r#"2"#.to_string()), (Lexeme::Operator, r#"-"#.to_string())])]
    #[timeout(Duration::from_secs(1))]
    fn test_operator_sequences(#[case] input: &str, #[case] expected: Vec<(Lexeme, String)>) {
        let mut lexer = Lexer::new(input.to_string());
        let result = lexer.lex_input();
        assert_lex_eq(input, &expected, &result);
    }

    #[rstest]
    #[case(r#"!1"#, vec![(Lexeme::Operator, r#"!"#.to_string()), (Lexeme::NumericLiteral, r#"1"#.to_string())])]
    #[case(r#"&&1"#, vec![(Lexeme::Operator, r#"&&"#.to_string()), (Lexeme::NumericLiteral, r#"1"#.to_string())])]
    #[case(r#"==1"#, vec![(Lexeme::Operator, r#"=="#.to_string()), (Lexeme::NumericLiteral, r#"1"#.to_string())])]
    #[case(r#"+1"#, vec![(Lexeme::Operator, r#"+"#.to_string()), (Lexeme::NumericLiteral, r#"1"#.to_string())])]
    #[case(r#"-1"#, vec![(Lexeme::Operator, r#"-"#.to_string()), (Lexeme::NumericLiteral, r#"1"#.to_string())])]
    #[case(r#"!a"#, vec![(Lexeme::Operator, r#"!"#.to_string()), (Lexeme::Identifier, r#"a"#.to_string())])]
    #[case(r#"&&a"#, vec![(Lexeme::Operator, r#"&&"#.to_string()), (Lexeme::Identifier, r#"a"#.to_string())])]
    #[case(r#"==a"#, vec![(Lexeme::Operator, r#"=="#.to_string()), (Lexeme::Identifier, r#"a"#.to_string())])]
    #[case(r#"+a"#, vec![(Lexeme::Operator, r#"+"#.to_string()), (Lexeme::Identifier, r#"a"#.to_string())])]
    #[case(r#"-a"#, vec![(Lexeme::Operator, r#"-"#.to_string()), (Lexeme::Identifier, r#"a"#.to_string())])]
    #[timeout(Duration::from_secs(1))]
    fn test_operators_with_following(#[case] input: &str, #[case] expected: Vec<(Lexeme, String)>) {
        let mut lexer = Lexer::new(input.to_string());
        let result = lexer.lex_input();
        assert_lex_eq(input, &expected, &result);
    }

    #[rstest]
    #[case(r#"$(),.:;?@\^`{}~"#, vec![(Lexeme::Punctuation, r#"$"#.to_string()), (Lexeme::Punctuation, r#"("#.to_string()), (Lexeme::Punctuation, r#")"#.to_string()), (Lexeme::Punctuation, r#","#.to_string()), (Lexeme::Punctuation, r#"."#.to_string()), (Lexeme::Punctuation, r#":"#.to_string()), (Lexeme::Punctuation, r#";"#.to_string()), (Lexeme::Punctuation, r#"?"#.to_string()), (Lexeme::Punctuation, r#"@"#.to_string()), (Lexeme::Punctuation, r#"\"#.to_string()), (Lexeme::Punctuation, r#"^"#.to_string()), (Lexeme::Punctuation, r#"`"#.to_string()), (Lexeme::Punctuation, r#"{"#.to_string()), (Lexeme::Punctuation, r#"}"#.to_string()), (Lexeme::Punctuation, r#"~"#.to_string())])]
    #[timeout(Duration::from_secs(1))]
    fn test_punctuation(#[case] input: &str, #[case] expected: Vec<(Lexeme, String)>) {
        let mut lexer = Lexer::new(input.to_string());
        let result = lexer.lex_input();
        assert_lex_eq(input, &expected, &result);
    }

    #[rstest]
    #[case(r#" "#, vec![])]
    #[case(r#"	"#, vec![])]
    #[case(r#"
"#, vec![])]
    #[case(r#"ab 12"#, vec![(Lexeme::Identifier, r#"ab"#.to_string()), (Lexeme::NumericLiteral, r#"12"#.to_string())])]
    #[case(r#"ab	12"#, vec![(Lexeme::Identifier, r#"ab"#.to_string()), (Lexeme::NumericLiteral, r#"12"#.to_string())])]
    #[case(r#"ab
12"#, vec![(Lexeme::Identifier, r#"ab"#.to_string()), (Lexeme::NumericLiteral, r#"12"#.to_string())])]
    #[timeout(Duration::from_secs(1))]
    fn test_whitespace(#[case] input: &str, #[case] expected: Vec<(Lexeme, String)>) {
        let mut lexer = Lexer::new(input.to_string());
        let result = lexer.lex_input();
        assert_lex_eq(input, &expected, &result);
    }

    #[rstest]
    #[case(r#"#abcd
"#, vec![])]
    #[case(r#"12#abcd
ab"#, vec![(Lexeme::NumericLiteral, r#"12"#.to_string()), (Lexeme::Identifier, r#"ab"#.to_string())])]
    #[case(r#"12#abcd"#, vec![(Lexeme::NumericLiteral, r#"12"#.to_string())])]
    #[case(r#"12#abcd#"#, vec![(Lexeme::NumericLiteral, r#"12"#.to_string())])]
    #[case(r#"ab#""#, vec![(Lexeme::Identifier, r#"ab"#.to_string())])]
    #[case(r#"ab#'"#, vec![(Lexeme::Identifier, r#"ab"#.to_string())])]
    #[case(r#"a##
b"#, vec![(Lexeme::Identifier, r#"a"#.to_string()), (Lexeme::Identifier, r#"b"#.to_string())])]
    #[case(r#"a##b"#, vec![(Lexeme::Identifier, r#"a"#.to_string())])]
    #[timeout(Duration::from_secs(1))]
    fn test_comments(#[case] input: &str, #[case] expected: Vec<(Lexeme, String)>) {
        let mut lexer = Lexer::new(input.to_string());
        let result = lexer.lex_input();
        assert_lex_eq(input, &expected, &result);
    }

    #[rstest]
    #[case(r#""#, vec![])]
    #[case(r#"a_1[0]=1;"#, vec![(Lexeme::Identifier, r#"a_1"#.to_string()), (Lexeme::Operator, r#"["#.to_string()), (Lexeme::NumericLiteral, r#"0"#.to_string()), (Lexeme::Operator, r#"]"#.to_string()), (Lexeme::Operator, r#"="#.to_string()), (Lexeme::NumericLiteral, r#"1"#.to_string()), (Lexeme::Punctuation, r#";"#.to_string())])]
    #[case(r#"if(x==5){print("hello");}"#, vec![(Lexeme::Keyword, r#"if"#.to_string()), (Lexeme::Punctuation, r#"("#.to_string()), (Lexeme::Identifier, r#"x"#.to_string()), (Lexeme::Operator, r#"=="#.to_string()), (Lexeme::NumericLiteral, r#"5"#.to_string()), (Lexeme::Punctuation, r#")"#.to_string()), (Lexeme::Punctuation, r#"{"#.to_string()), (Lexeme::Keyword, r#"print"#.to_string()), (Lexeme::Punctuation, r#"("#.to_string()), (Lexeme::StringLiteral, r#""hello""#.to_string()), (Lexeme::Punctuation, r#")"#.to_string()), (Lexeme::Punctuation, r#";"#.to_string()), (Lexeme::Punctuation, r#"}"#.to_string())])]
    #[case(r#"func test(){return 42;}"#, vec![(Lexeme::Keyword, r#"func"#.to_string()), (Lexeme::Identifier, r#"test"#.to_string()), (Lexeme::Punctuation, r#"("#.to_string()), (Lexeme::Punctuation, r#")"#.to_string()), (Lexeme::Punctuation, r#"{"#.to_string()), (Lexeme::Keyword, r#"return"#.to_string()), (Lexeme::NumericLiteral, r#"42"#.to_string()), (Lexeme::Punctuation, r#";"#.to_string()), (Lexeme::Punctuation, r#"}"#.to_string())])]
    #[case(r#"while(i<n){++i;}"#, vec![(Lexeme::Keyword, r#"while"#.to_string()), (Lexeme::Punctuation, r#"("#.to_string()), (Lexeme::Identifier, r#"i"#.to_string()), (Lexeme::Operator, r#"<"#.to_string()), (Lexeme::Identifier, r#"n"#.to_string()), (Lexeme::Punctuation, r#")"#.to_string()), (Lexeme::Punctuation, r#"{"#.to_string()), (Lexeme::Operator, r#"++"#.to_string()), (Lexeme::Identifier, r#"i"#.to_string()), (Lexeme::Punctuation, r#";"#.to_string()), (Lexeme::Punctuation, r#"}"#.to_string())])]
    #[timeout(Duration::from_secs(1))]
    fn test_complete_programs(#[case] input: &str, #[case] expected: Vec<(Lexeme, String)>) {
        let mut lexer = Lexer::new(input.to_string());
        let result = lexer.lex_input();
        assert_lex_eq(input, &expected, &result);
    }
}
