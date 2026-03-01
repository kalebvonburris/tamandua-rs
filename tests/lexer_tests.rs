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
                "\nInput: {:?}\n\nExpected:{:#?}\n\nGiven:{:#?}",
                input, expected, given
            );
        }
    }

    #[rstest]
    #[case("a", vec![(Lexeme::Identifier, "a".to_string())])]
    #[case(" a", vec![(Lexeme::Identifier, "a".to_string())])]
    #[case("_", vec![(Lexeme::Identifier, "_".to_string())])]
    #[case(" _", vec![(Lexeme::Identifier, "_".to_string())])]
    #[case("bx", vec![(Lexeme::Identifier, "bx".to_string())])]
    #[case("b3", vec![(Lexeme::Identifier, "b3".to_string())])]
    #[case("_n", vec![(Lexeme::Identifier, "_n".to_string())])]
    #[case("_4", vec![(Lexeme::Identifier, "_4".to_string())])]
    #[case("abc_39xyz", vec![(Lexeme::Identifier, "abc_39xyz".to_string())])]
    #[case("abc eol_3", vec![(Lexeme::Identifier, "abc".to_string()), (Lexeme::Identifier, "eol_3".to_string())])]
    #[case("a  ", vec![(Lexeme::Identifier, "a".to_string())])]
    #[case("a#", vec![(Lexeme::Identifier, "a".to_string())])]
    #[case("a #", vec![(Lexeme::Identifier, "a".to_string())])]
    #[case("#\na", vec![(Lexeme::Identifier, "a".to_string())])]
    #[case("#\n a", vec![(Lexeme::Identifier, "a".to_string())])]
    #[case("ab", vec![(Lexeme::Identifier, "ab".to_string())])]
    #[case("_chr", vec![(Lexeme::Identifier, "_chr".to_string())])]
    #[timeout(Duration::from_secs(1))]
    fn test_identifiers(#[case] input: &str, #[case] expected: Vec<(Lexeme, String)>) {
        let mut lexer = Lexer::new(input.to_string());
        let result = lexer.lex_input();
        assert_lex_eq(input, &expected, &result);
    }

    #[rstest]
    #[case("chr", vec![(Lexeme::Keyword, "chr".to_string())])]
    #[case("else", vec![(Lexeme::Keyword, "else".to_string())])]
    #[case("elsif", vec![(Lexeme::Keyword, "elsif".to_string())])]
    #[case("func", vec![(Lexeme::Keyword, "func".to_string())])]
    #[case("if", vec![(Lexeme::Keyword, "if".to_string())])]
    #[case("print", vec![(Lexeme::Keyword, "print".to_string())])]
    #[case("println", vec![(Lexeme::Keyword, "println".to_string())])]
    #[case("readint", vec![(Lexeme::Keyword, "readint".to_string())])]
    #[case("return", vec![(Lexeme::Keyword, "return".to_string())])]
    #[case("rnd", vec![(Lexeme::Keyword, "rnd".to_string())])]
    #[case("while", vec![(Lexeme::Keyword, "while".to_string())])]
    #[timeout(Duration::from_secs(1))]
    fn test_keywords(#[case] input: &str, #[case] expected: Vec<(Lexeme, String)>) {
        let mut lexer = Lexer::new(input.to_string());
        let result = lexer.lex_input();
        assert_lex_eq(input, &expected, &result);
    }

    #[rstest]
    #[case("funcc", vec![(Lexeme::Identifier, "funcc".to_string())])]
    #[case("chr2", vec![(Lexeme::Identifier, "chr2".to_string())])]
    #[case("Print", vec![(Lexeme::Identifier, "Print".to_string())])]
    #[case("elsE", vec![(Lexeme::Identifier, "elsE".to_string())])]
    #[case("IF", vec![(Lexeme::Identifier, "IF".to_string())])]
    #[case("elsi", vec![(Lexeme::Identifier, "elsi".to_string())])]
    #[case("printl", vec![(Lexeme::Identifier, "printl".to_string())])]
    #[case("els if", vec![(Lexeme::Identifier, "els".to_string()), (Lexeme::Keyword, "if".to_string())])]
    #[case("re#\nturn", vec![(Lexeme::Identifier, "re".to_string()), (Lexeme::Identifier, "turn".to_string())])]
    #[case("whi2le", vec![(Lexeme::Identifier, "whi2le".to_string())])]
    #[case("pri_nt", vec![(Lexeme::Identifier, "pri_nt".to_string())])]
    #[case("ifreturn", vec![(Lexeme::Identifier, "ifreturn".to_string())])]
    #[timeout(Duration::from_secs(1))]
    fn test_keyword_variations(#[case] input: &str, #[case] expected: Vec<(Lexeme, String)>) {
        let mut lexer = Lexer::new(input.to_string());
        let result = lexer.lex_input();
        assert_lex_eq(input, &expected, &result);
    }

    #[rstest]
    #[case("3", vec![(Lexeme::NumericLiteral, "3".to_string())])]
    #[case("3a", vec![(Lexeme::NumericLiteral, "3".to_string()), (Lexeme::Identifier, "a".to_string())])]
    #[case("123456", vec![(Lexeme::NumericLiteral, "123456".to_string())])]
    #[case(".123456", vec![(Lexeme::Punctuation, ".".to_string()), (Lexeme::NumericLiteral, "123456".to_string())])]
    #[case("123456.", vec![(Lexeme::NumericLiteral, "123456".to_string()), (Lexeme::Punctuation, ".".to_string())])]
    #[case("123.456", vec![(Lexeme::NumericLiteral, "123".to_string()), (Lexeme::Punctuation, ".".to_string()), (Lexeme::NumericLiteral, "456".to_string())])]
    #[case("1.2.3", vec![(Lexeme::NumericLiteral, "1".to_string()), (Lexeme::Punctuation, ".".to_string()), (Lexeme::NumericLiteral, "2".to_string()), (Lexeme::Punctuation, ".".to_string()), (Lexeme::NumericLiteral, "3".to_string())])]
    #[case("123 456", vec![(Lexeme::NumericLiteral, "123".to_string()), (Lexeme::NumericLiteral, "456".to_string())])]
    #[timeout(Duration::from_secs(1))]
    fn test_numeric_literals(#[case] input: &str, #[case] expected: Vec<(Lexeme, String)>) {
        let mut lexer = Lexer::new(input.to_string());
        let result = lexer.lex_input();
        assert_lex_eq(input, &expected, &result);
    }

    #[rstest]
    #[case("+123456", vec![(Lexeme::Operator, "+".to_string()), (Lexeme::NumericLiteral, "123456".to_string())])]
    #[case("+.123456", vec![(Lexeme::Operator, "+".to_string()), (Lexeme::Punctuation, ".".to_string()), (Lexeme::NumericLiteral, "123456".to_string())])]
    #[case("+123456.", vec![(Lexeme::Operator, "+".to_string()), (Lexeme::NumericLiteral, "123456".to_string()), (Lexeme::Punctuation, ".".to_string())])]
    #[case("+123.456", vec![(Lexeme::Operator, "+".to_string()), (Lexeme::NumericLiteral, "123".to_string()), (Lexeme::Punctuation, ".".to_string()), (Lexeme::NumericLiteral, "456".to_string())])]
    #[case("+1.2.3", vec![(Lexeme::Operator, "+".to_string()), (Lexeme::NumericLiteral, "1".to_string()), (Lexeme::Punctuation, ".".to_string()), (Lexeme::NumericLiteral, "2".to_string()), (Lexeme::Punctuation, ".".to_string()), (Lexeme::NumericLiteral, "3".to_string())])]
    #[case("-123456", vec![(Lexeme::Operator, "-".to_string()), (Lexeme::NumericLiteral, "123456".to_string())])]
    #[case("-.123456", vec![(Lexeme::Operator, "-".to_string()), (Lexeme::Punctuation, ".".to_string()), (Lexeme::NumericLiteral, "123456".to_string())])]
    #[case("-123456.", vec![(Lexeme::Operator, "-".to_string()), (Lexeme::NumericLiteral, "123456".to_string()), (Lexeme::Punctuation, ".".to_string())])]
    #[case("-123.456", vec![(Lexeme::Operator, "-".to_string()), (Lexeme::NumericLiteral, "123".to_string()), (Lexeme::Punctuation, ".".to_string()), (Lexeme::NumericLiteral, "456".to_string())])]
    #[case("-1.2.3", vec![(Lexeme::Operator, "-".to_string()), (Lexeme::NumericLiteral, "1".to_string()), (Lexeme::Punctuation, ".".to_string()), (Lexeme::NumericLiteral, "2".to_string()), (Lexeme::Punctuation, ".".to_string()), (Lexeme::NumericLiteral, "3".to_string())])]
    #[case("+-123456", vec![(Lexeme::Operator, "+".to_string()), (Lexeme::Operator, "-".to_string()), (Lexeme::NumericLiteral, "123456".to_string())])]
    #[case("-+123456", vec![(Lexeme::Operator, "-".to_string()), (Lexeme::Operator, "+".to_string()), (Lexeme::NumericLiteral, "123456".to_string())])]
    #[timeout(Duration::from_secs(1))]
    fn test_numeric_with_signs(#[case] input: &str, #[case] expected: Vec<(Lexeme, String)>) {
        let mut lexer = Lexer::new(input.to_string());
        let result = lexer.lex_input();
        assert_lex_eq(input, &expected, &result);
    }

    #[rstest]
    #[case("123e456", vec![(Lexeme::NumericLiteral, "123e456".to_string())])]
    #[case("123e+456", vec![(Lexeme::NumericLiteral, "123e+456".to_string())])]
    #[case("123e-456", vec![(Lexeme::NumericLiteral, "123".to_string()), (Lexeme::Identifier, "e".to_string()), (Lexeme::Operator, "-".to_string()), (Lexeme::NumericLiteral, "456".to_string())])]
    #[case("+123e456", vec![(Lexeme::Operator, "+".to_string()), (Lexeme::NumericLiteral, "123e456".to_string())])]
    #[case("+123e+456", vec![(Lexeme::Operator, "+".to_string()), (Lexeme::NumericLiteral, "123e+456".to_string())])]
    #[case("+123e-456", vec![(Lexeme::Operator, "+".to_string()), (Lexeme::NumericLiteral, "123".to_string()), (Lexeme::Identifier, "e".to_string()), (Lexeme::Operator, "-".to_string()), (Lexeme::NumericLiteral, "456".to_string())])]
    #[case("-123e456", vec![(Lexeme::Operator, "-".to_string()), (Lexeme::NumericLiteral, "123e456".to_string())])]
    #[case("-123e+456", vec![(Lexeme::Operator, "-".to_string()), (Lexeme::NumericLiteral, "123e+456".to_string())])]
    #[case("-123e-456", vec![(Lexeme::Operator, "-".to_string()), (Lexeme::NumericLiteral, "123".to_string()), (Lexeme::Identifier, "e".to_string()), (Lexeme::Operator, "-".to_string()), (Lexeme::NumericLiteral, "456".to_string())])]
    #[case("123E456", vec![(Lexeme::NumericLiteral, "123E456".to_string())])]
    #[case("123E+456", vec![(Lexeme::NumericLiteral, "123E+456".to_string())])]
    #[case("123E-456", vec![(Lexeme::NumericLiteral, "123".to_string()), (Lexeme::Identifier, "E".to_string()), (Lexeme::Operator, "-".to_string()), (Lexeme::NumericLiteral, "456".to_string())])]
    #[case("+123E456", vec![(Lexeme::Operator, "+".to_string()), (Lexeme::NumericLiteral, "123E456".to_string())])]
    #[case("+123E+456", vec![(Lexeme::Operator, "+".to_string()), (Lexeme::NumericLiteral, "123E+456".to_string())])]
    #[case("+123E-456", vec![(Lexeme::Operator, "+".to_string()), (Lexeme::NumericLiteral, "123".to_string()), (Lexeme::Identifier, "E".to_string()), (Lexeme::Operator, "-".to_string()), (Lexeme::NumericLiteral, "456".to_string())])]
    #[case("-123E456", vec![(Lexeme::Operator, "-".to_string()), (Lexeme::NumericLiteral, "123E456".to_string())])]
    #[case("-123E+456", vec![(Lexeme::Operator, "-".to_string()), (Lexeme::NumericLiteral, "123E+456".to_string())])]
    #[case("-123E-456", vec![(Lexeme::Operator, "-".to_string()), (Lexeme::NumericLiteral, "123".to_string()), (Lexeme::Identifier, "E".to_string()), (Lexeme::Operator, "-".to_string()), (Lexeme::NumericLiteral, "456".to_string())])]
    #[timeout(Duration::from_secs(1))]
    fn test_scientific_notation(#[case] input: &str, #[case] expected: Vec<(Lexeme, String)>) {
        let mut lexer = Lexer::new(input.to_string());
        let result = lexer.lex_input();
        assert_lex_eq(input, &expected, &result);
    }

    #[rstest]
    #[case("e", vec![(Lexeme::Identifier, "e".to_string())])]
    #[case("E", vec![(Lexeme::Identifier, "E".to_string())])]
    #[case("e3", vec![(Lexeme::Identifier, "e3".to_string())])]
    #[case("E3", vec![(Lexeme::Identifier, "E3".to_string())])]
    #[case("e+3", vec![(Lexeme::Identifier, "e".to_string()), (Lexeme::Operator, "+".to_string()), (Lexeme::NumericLiteral, "3".to_string())])]
    #[case("E+3", vec![(Lexeme::Identifier, "E".to_string()), (Lexeme::Operator, "+".to_string()), (Lexeme::NumericLiteral, "3".to_string())])]
    #[case("1e3", vec![(Lexeme::NumericLiteral, "1e3".to_string())])]
    #[case("123e", vec![(Lexeme::NumericLiteral, "123".to_string()), (Lexeme::Identifier, "e".to_string())])]
    #[case("123E", vec![(Lexeme::NumericLiteral, "123".to_string()), (Lexeme::Identifier, "E".to_string())])]
    #[case("123ee", vec![(Lexeme::NumericLiteral, "123".to_string()), (Lexeme::Identifier, "ee".to_string())])]
    #[case("123Ee", vec![(Lexeme::NumericLiteral, "123".to_string()), (Lexeme::Identifier, "Ee".to_string())])]
    #[case("123eE", vec![(Lexeme::NumericLiteral, "123".to_string()), (Lexeme::Identifier, "eE".to_string())])]
    #[case("123EE", vec![(Lexeme::NumericLiteral, "123".to_string()), (Lexeme::Identifier, "EE".to_string())])]
    #[case("1.2e34", vec![(Lexeme::NumericLiteral, "1".to_string()), (Lexeme::Punctuation, ".".to_string()), (Lexeme::NumericLiteral, "2e34".to_string())])]
    #[case("12e3.4", vec![(Lexeme::NumericLiteral, "12e3".to_string()), (Lexeme::Punctuation, ".".to_string()), (Lexeme::NumericLiteral, "4".to_string())])]
    #[case("123e+", vec![(Lexeme::NumericLiteral, "123".to_string()), (Lexeme::Identifier, "e".to_string()), (Lexeme::Operator, "+".to_string())])]
    #[case("123E+", vec![(Lexeme::NumericLiteral, "123".to_string()), (Lexeme::Identifier, "E".to_string()), (Lexeme::Operator, "+".to_string())])]
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
    #[case("''", vec![(Lexeme::StringLiteral, "''".to_string())])]
    #[case("\"\"", vec![(Lexeme::StringLiteral, "\"\"".to_string())])]
    #[case("'a'", vec![(Lexeme::StringLiteral, "'a'".to_string())])]
    #[case("\"b\"", vec![(Lexeme::StringLiteral, "\"b\"".to_string())])]
    #[case("'abc eol'", vec![(Lexeme::StringLiteral, "'abc eol'".to_string())])]
    #[case("\"The quick brown fox.\"", vec![(Lexeme::StringLiteral, "\"The quick brown fox.\"".to_string())])]
    #[case("'aa\"bb'", vec![(Lexeme::StringLiteral, "'aa\"bb'".to_string())])]
    #[case("\"cc'dd\"", vec![(Lexeme::StringLiteral, "\"cc'dd\"".to_string())])]
    #[case("\"\\a\"", vec![(Lexeme::StringLiteral, "\"\\a\"".to_string())])]
    #[timeout(Duration::from_secs(1))]
    fn test_string_literals(#[case] input: &str, #[case] expected: Vec<(Lexeme, String)>) {
        let mut lexer = Lexer::new(input.to_string());
        let result = lexer.lex_input();
        assert_lex_eq(input, &expected, &result);
    }

    #[rstest]
    #[case("'aabbcc", vec![(Lexeme::Malformed(LexingError::MalformedStringLiteral), "'aabbcc".to_string())])]
    #[case("'aabbcc\"", vec![(Lexeme::Malformed(LexingError::MalformedStringLiteral), "'aabbcc\"".to_string())])]
    #[case("'aabbcc\n", vec![(Lexeme::Malformed(LexingError::MalformedStringLiteral), "'aabbcc\n".to_string())])]
    #[case("\"aabbcc", vec![(Lexeme::Malformed(LexingError::MalformedStringLiteral), "\"aabbcc".to_string())])]
    #[case("\"aabbcc'", vec![(Lexeme::Malformed(LexingError::MalformedStringLiteral), "\"aabbcc'".to_string())])]
    #[case("\"aabbcc\n", vec![(Lexeme::Malformed(LexingError::MalformedStringLiteral), "\"aabbcc".to_string())])]
    #[case("\"aabbcc\nd", vec![(Lexeme::Malformed(LexingError::MalformedStringLiteral), "\"aabbcc".to_string()), (Lexeme::Identifier, "d".to_string())])]
    #[case("'\"'\"'\"", vec![(Lexeme::StringLiteral, "'\"'".to_string()), (Lexeme::StringLiteral, "'\"'\"'\"".to_string())])]
    #[timeout(Duration::from_secs(1))]
    fn test_malformed_strings(#[case] input: &str, #[case] expected: Vec<(Lexeme, String)>) {
        let mut lexer = Lexer::new(input.to_string());
        let result = lexer.lex_input();
        assert_lex_eq(input, &expected, &result);
    }

    #[rstest]
    #[case("!", vec![(Lexeme::Operator, "!".to_string())])]
    #[case("&&", vec![(Lexeme::Operator, "&&".to_string())])]
    #[case("||", vec![(Lexeme::Operator, "||".to_string())])]
    #[case("==", vec![(Lexeme::Operator, "==".to_string())])]
    #[case("!=", vec![(Lexeme::Operator, "!=".to_string())])]
    #[case("<", vec![(Lexeme::Operator, "<".to_string())])]
    #[case("<=", vec![(Lexeme::Operator, "<=".to_string())])]
    #[case(">", vec![(Lexeme::Operator, ">".to_string())])]
    #[case(">=", vec![(Lexeme::Operator, ">=".to_string())])]
    #[case("+", vec![(Lexeme::Operator, "+".to_string())])]
    #[case("-", vec![(Lexeme::Operator, "-".to_string())])]
    #[case("*", vec![(Lexeme::Operator, "*".to_string())])]
    #[case("/", vec![(Lexeme::Operator, "/".to_string())])]
    #[case("%", vec![(Lexeme::Operator, "%".to_string())])]
    #[case("[", vec![(Lexeme::Operator, "[".to_string())])]
    #[case("]", vec![(Lexeme::Operator, "]".to_string())])]
    #[case("=", vec![(Lexeme::Operator, "=".to_string())])]
    #[case("++", vec![(Lexeme::Operator, "++".to_string())])]
    #[case("--", vec![(Lexeme::Operator, "--".to_string())])]
    #[timeout(Duration::from_secs(1))]
    fn test_operators(#[case] input: &str, #[case] expected: Vec<(Lexeme, String)>) {
        let mut lexer = Lexer::new(input.to_string());
        let result = lexer.lex_input();
        assert_lex_eq(input, &expected, &result);
    }

    #[rstest]
    #[case("+-++--", vec![(Lexeme::Operator, "+".to_string()), (Lexeme::Operator, "-".to_string()), (Lexeme::Operator, "++".to_string()), (Lexeme::Operator, "--".to_string())])]
    #[case("+++++", vec![(Lexeme::Operator, "++".to_string()), (Lexeme::Operator, "++".to_string()), (Lexeme::Operator, "+".to_string())])]
    #[case("-----", vec![(Lexeme::Operator, "--".to_string()), (Lexeme::Operator, "--".to_string()), (Lexeme::Operator, "-".to_string())])]
    #[case("=====", vec![(Lexeme::Operator, "==".to_string()), (Lexeme::Operator, "==".to_string()), (Lexeme::Operator, "=".to_string())])]
    #[case("=<<==", vec![(Lexeme::Operator, "=".to_string()), (Lexeme::Operator, "<".to_string()), (Lexeme::Operator, "<=".to_string()), (Lexeme::Operator, "=".to_string())])]
    #[case("**/ ", vec![(Lexeme::Operator, "*".to_string()), (Lexeme::Operator, "*".to_string()), (Lexeme::Operator, "/".to_string())])]
    #[case("= =", vec![(Lexeme::Operator, "=".to_string()), (Lexeme::Operator, "=".to_string())])]
    #[case("+++1+", vec![(Lexeme::Operator, "++".to_string()), (Lexeme::Operator, "+".to_string()), (Lexeme::NumericLiteral, "1".to_string()), (Lexeme::Operator, "+".to_string())])]
    #[case("---2-", vec![(Lexeme::Operator, "--".to_string()), (Lexeme::Operator, "-".to_string()), (Lexeme::NumericLiteral, "2".to_string()), (Lexeme::Operator, "-".to_string())])]
    #[timeout(Duration::from_secs(1))]
    fn test_operator_sequences(#[case] input: &str, #[case] expected: Vec<(Lexeme, String)>) {
        let mut lexer = Lexer::new(input.to_string());
        let result = lexer.lex_input();
        assert_lex_eq(input, &expected, &result);
    }

    #[rstest]
    #[case("!1", vec![(Lexeme::Operator, "!".to_string()), (Lexeme::NumericLiteral, "1".to_string())])]
    #[case("&&1", vec![(Lexeme::Operator, "&&".to_string()), (Lexeme::NumericLiteral, "1".to_string())])]
    #[case("==1", vec![(Lexeme::Operator, "==".to_string()), (Lexeme::NumericLiteral, "1".to_string())])]
    #[case("+1", vec![(Lexeme::Operator, "+".to_string()), (Lexeme::NumericLiteral, "1".to_string())])]
    #[case("-1", vec![(Lexeme::Operator, "-".to_string()), (Lexeme::NumericLiteral, "1".to_string())])]
    #[case("!a", vec![(Lexeme::Operator, "!".to_string()), (Lexeme::Identifier, "a".to_string())])]
    #[case("&&a", vec![(Lexeme::Operator, "&&".to_string()), (Lexeme::Identifier, "a".to_string())])]
    #[case("==a", vec![(Lexeme::Operator, "==".to_string()), (Lexeme::Identifier, "a".to_string())])]
    #[case("+a", vec![(Lexeme::Operator, "+".to_string()), (Lexeme::Identifier, "a".to_string())])]
    #[case("-a", vec![(Lexeme::Operator, "-".to_string()), (Lexeme::Identifier, "a".to_string())])]
    #[timeout(Duration::from_secs(1))]
    fn test_operators_with_following(#[case] input: &str, #[case] expected: Vec<(Lexeme, String)>) {
        let mut lexer = Lexer::new(input.to_string());
        let result = lexer.lex_input();
        assert_lex_eq(input, &expected, &result);
    }

    #[rstest]
    #[case("$(),.:;?@\\^`{}~", vec![(Lexeme::Punctuation, "$".to_string()), (Lexeme::Punctuation, "(".to_string()), (Lexeme::Punctuation, ")".to_string()), (Lexeme::Punctuation, ",".to_string()), (Lexeme::Punctuation, ".".to_string()), (Lexeme::Punctuation, ":".to_string()), (Lexeme::Punctuation, ";".to_string()), (Lexeme::Punctuation, "?".to_string()), (Lexeme::Punctuation, "@".to_string()), (Lexeme::Punctuation, "\\".to_string()), (Lexeme::Punctuation, "^".to_string()), (Lexeme::Punctuation, "`".to_string()), (Lexeme::Punctuation, "{".to_string()), (Lexeme::Punctuation, "}".to_string()), (Lexeme::Punctuation, "~".to_string())])]
    #[timeout(Duration::from_secs(1))]
    fn test_punctuation(#[case] input: &str, #[case] expected: Vec<(Lexeme, String)>) {
        let mut lexer = Lexer::new(input.to_string());
        let result = lexer.lex_input();
        assert_lex_eq(input, &expected, &result);
    }

    #[rstest]
    #[case(" ", vec![])]
    #[case("\t", vec![])]
    #[case("\n", vec![])]
    #[case("ab 12", vec![(Lexeme::Identifier, "ab".to_string()), (Lexeme::NumericLiteral, "12".to_string())])]
    #[case("ab\t12", vec![(Lexeme::Identifier, "ab".to_string()), (Lexeme::NumericLiteral, "12".to_string())])]
    #[case("ab\n12", vec![(Lexeme::Identifier, "ab".to_string()), (Lexeme::NumericLiteral, "12".to_string())])]
    #[timeout(Duration::from_secs(1))]
    fn test_whitespace(#[case] input: &str, #[case] expected: Vec<(Lexeme, String)>) {
        let mut lexer = Lexer::new(input.to_string());
        let result = lexer.lex_input();
        assert_lex_eq(input, &expected, &result);
    }

    #[rstest]
    #[case("#abcd\n", vec![])]
    #[case("12#abcd\nab", vec![(Lexeme::NumericLiteral, "12".to_string()), (Lexeme::Identifier, "ab".to_string())])]
    #[case("12#abcd", vec![(Lexeme::NumericLiteral, "12".to_string())])]
    #[case("12#abcd#", vec![(Lexeme::NumericLiteral, "12".to_string())])]
    #[case("ab#\"", vec![(Lexeme::Identifier, "ab".to_string())])]
    #[case("ab#'", vec![(Lexeme::Identifier, "ab".to_string())])]
    #[case("a##\nb", vec![(Lexeme::Identifier, "a".to_string()), (Lexeme::Identifier, "b".to_string())])]
    #[case("a##b", vec![(Lexeme::Identifier, "a".to_string())])]
    #[timeout(Duration::from_secs(1))]
    fn test_comments(#[case] input: &str, #[case] expected: Vec<(Lexeme, String)>) {
        let mut lexer = Lexer::new(input.to_string());
        let result = lexer.lex_input();
        assert_lex_eq(input, &expected, &result);
    }

    #[rstest]
    #[case("", vec![])]
    #[case("a_1[0]=1;", vec![(Lexeme::Identifier, "a_1".to_string()), (Lexeme::Operator, "[".to_string()), (Lexeme::NumericLiteral, "0".to_string()), (Lexeme::Operator, "]".to_string()), (Lexeme::Operator, "=".to_string()), (Lexeme::NumericLiteral, "1".to_string()), (Lexeme::Punctuation, ";".to_string())])]
    #[case("if(x==5){print(\"hello\");}", vec![(Lexeme::Keyword, "if".to_string()), (Lexeme::Punctuation, "(".to_string()), (Lexeme::Identifier, "x".to_string()), (Lexeme::Operator, "==".to_string()), (Lexeme::NumericLiteral, "5".to_string()), (Lexeme::Punctuation, ")".to_string()), (Lexeme::Punctuation, "{".to_string()), (Lexeme::Keyword, "print".to_string()), (Lexeme::Punctuation, "(".to_string()), (Lexeme::StringLiteral, "\"hello\"".to_string()), (Lexeme::Punctuation, ")".to_string()), (Lexeme::Punctuation, ";".to_string()), (Lexeme::Punctuation, "}".to_string())])]
    #[case("func test(){return 42;}", vec![(Lexeme::Keyword, "func".to_string()), (Lexeme::Identifier, "test".to_string()), (Lexeme::Punctuation, "(".to_string()), (Lexeme::Punctuation, ")".to_string()), (Lexeme::Punctuation, "{".to_string()), (Lexeme::Keyword, "return".to_string()), (Lexeme::NumericLiteral, "42".to_string()), (Lexeme::Punctuation, ";".to_string()), (Lexeme::Punctuation, "}".to_string())])]
    #[case("while(i<n){++i;}", vec![(Lexeme::Keyword, "while".to_string()), (Lexeme::Punctuation, "(".to_string()), (Lexeme::Identifier, "i".to_string()), (Lexeme::Operator, "<".to_string()), (Lexeme::Identifier, "n".to_string()), (Lexeme::Punctuation, ")".to_string()), (Lexeme::Punctuation, "{".to_string()), (Lexeme::Operator, "++".to_string()), (Lexeme::Identifier, "i".to_string()), (Lexeme::Punctuation, ";".to_string()), (Lexeme::Punctuation, "}".to_string())])]
    #[timeout(Duration::from_secs(1))]
    fn test_complete_programs(#[case] input: &str, #[case] expected: Vec<(Lexeme, String)>) {
        let mut lexer = Lexer::new(input.to_string());
        let result = lexer.lex_input();
        assert_lex_eq(input, &expected, &result);
    }
}
