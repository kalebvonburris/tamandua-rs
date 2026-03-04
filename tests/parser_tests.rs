//! Parser tests — auto-generated from parseit_test.lua
//! Do not edit by hand.

use rstest::rstest;
use std::time::Duration;
use tamandua_rs::parser::{Node, Parser, ParseToken};
use tamandua_rs::lexer::Lexer;

    #[rstest]
    #[case("", true, true, Node::program(vec![]))]
    #[case("return", false, true, Node::none())]
    #[case("elsif", true, false, Node::none())]
    #[case("else", true, false, Node::none())]
    #[case("ab", false, true, Node::none())]
    #[case("ab;", false, false, Node::none())]
    #[case("123", true, false, Node::none())]
    #[case("123;", true, false, Node::none())]
    #[case("\"xyz\"", true, false, Node::none())]
    #[case("<=", true, false, Node::none())]
    #[case("{", true, false, Node::none())]
    #[case("", true, false, Node::none())]
    #[case("\"", true, false, Node::none())]
    #[timeout(Duration::from_secs(1))]
    fn test_simple(
        #[case] input: &str,
        #[case] exp_good: bool,
        #[case] exp_done: bool,
        #[case] exp_ast: Node,
    ) {
        let tokens = Lexer::new(input.to_string()).lex_input();
        let (good, done, ast) = Parser::new(tokens).parse();
        assert_eq!(good, exp_good, "good flag mismatch\ninput: {input:?}");
        assert_eq!(done, exp_done, "done flag mismatch\ninput: {input:?}");
        if exp_good && exp_done {
            assert_eq!(ast, exp_ast, "ast mismatch\ninput: {input:?}");
        }
    }

    #[rstest]
    #[case("print();", true, true, Node::program(vec![Node::stmt(ParseToken::Print, vec![])]))]
    #[case("print();print();print();", true, true, Node::program(vec![Node::stmt(ParseToken::Print, vec![]), Node::stmt(ParseToken::Print, vec![]), Node::stmt(ParseToken::Print, vec![])]))]
    #[case("print(\"abc\");", true, true, Node::program(vec![Node::stmt(ParseToken::Print, vec![Node::stmt("strlit_out", vec![Node::value("\"abc\"")])])]))]
    #[case("print(\"a\",\"b\",\"c\",\"d\",\"e\");", true, true, Node::program(vec![Node::stmt(ParseToken::Print, vec![Node::stmt("strlit_out", vec![Node::value("\"a\"")]), Node::stmt("strlit_out", vec![Node::value("\"b\"")]), Node::stmt("strlit_out", vec![Node::value("\"c\"")]), Node::stmt("strlit_out", vec![Node::value("\"d\"")]), Node::stmt("strlit_out", vec![Node::value("\"e\"")])])]))]
    #[case("print()", false, true, Node::none())]
    #[case("print", false, true, Node::none())]
    #[case("print \"a\";", false, false, Node::none())]
    #[case("print\"a\");", false, false, Node::none())]
    #[case("print(\"a\";", false, false, Node::none())]
    #[case("print(if);", false, false, Node::none())]
    #[case("print(print);", false, false, Node::none())]
    #[case("print(\"a\"\"b\");", false, false, Node::none())]
    #[case("print(,\"a\");", false, false, Node::none())]
    #[case("print(\"a\",);", false, false, Node::none())]
    #[case("print(,);", false, false, Node::none())]
    #[case("print(\"a\",,\"b\");", false, false, Node::none())]
    #[case("print(\"a\")else;", false, false, Node::none())]
    #[case("print(\"a\");else", true, false, Node::none())]
    #[case("println();", true, true, Node::program(vec![Node::stmt(ParseToken::Println, vec![])]))]
    #[case("println();println();println();", true, true, Node::program(vec![Node::stmt(ParseToken::Println, vec![]), Node::stmt(ParseToken::Println, vec![]), Node::stmt(ParseToken::Println, vec![])]))]
    #[case("println(\"abc\");", true, true, Node::program(vec![Node::stmt(ParseToken::Println, vec![Node::stmt("strlit_out", vec![Node::value("\"abc\"")])])]))]
    #[case("println(\"a\",\"b\",\"c\",\"d\",\"e\");", true, true, Node::program(vec![Node::stmt(ParseToken::Println, vec![Node::stmt("strlit_out", vec![Node::value("\"a\"")]), Node::stmt("strlit_out", vec![Node::value("\"b\"")]), Node::stmt("strlit_out", vec![Node::value("\"c\"")]), Node::stmt("strlit_out", vec![Node::value("\"d\"")]), Node::stmt("strlit_out", vec![Node::value("\"e\"")])])]))]
    #[case("println()", false, true, Node::none())]
    #[case("println", false, true, Node::none())]
    #[case("println \"a\";", false, false, Node::none())]
    #[case("println\"a\");", false, false, Node::none())]
    #[case("println(\"a\";", false, false, Node::none())]
    #[case("println(if);", false, false, Node::none())]
    #[case("println(println);", false, false, Node::none())]
    #[case("println(\"a\"\"b\");", false, false, Node::none())]
    #[case("println(,\"a\");", false, false, Node::none())]
    #[case("println(\"a\",);", false, false, Node::none())]
    #[case("println(,);", false, false, Node::none())]
    #[case("println(\"a\",,\"b\");", false, false, Node::none())]
    #[case("println(\"a\")else;", false, false, Node::none())]
    #[case("println(\"a\");else", true, false, Node::none())]
    #[case("print();println();", true, true, Node::program(vec![Node::stmt(ParseToken::Print, vec![]), Node::stmt(ParseToken::Println, vec![])]))]
    #[case("println();print();", true, true, Node::program(vec![Node::stmt(ParseToken::Println, vec![]), Node::stmt(ParseToken::Print, vec![])]))]
    #[case("print('a');println('b');", true, true, Node::program(vec![Node::stmt(ParseToken::Print, vec![Node::stmt("strlit_out", vec![Node::value("'a'")])]), Node::stmt(ParseToken::Println, vec![Node::stmt("strlit_out", vec![Node::value("'b'")])])]))]
    #[case("println('x');print('y');", true, true, Node::program(vec![Node::stmt(ParseToken::Println, vec![Node::stmt("strlit_out", vec![Node::value("'x'")])]), Node::stmt(ParseToken::Print, vec![Node::stmt("strlit_out", vec![Node::value("'y'")])])]))]
    #[case("\"a\";", true, false, Node::none())]
    #[case("(\"a\");", true, false, Node::none())]
    #[timeout(Duration::from_secs(1))]
    fn test_print_stmt_no_expr(
        #[case] input: &str,
        #[case] exp_good: bool,
        #[case] exp_done: bool,
        #[case] exp_ast: Node,
    ) {
        let tokens = Lexer::new(input.to_string()).lex_input();
        let (good, done, ast) = Parser::new(tokens).parse();
        assert_eq!(good, exp_good, "good flag mismatch\ninput: {input:?}");
        assert_eq!(done, exp_done, "done flag mismatch\ninput: {input:?}");
        if exp_good && exp_done {
            assert_eq!(ast, exp_ast, "ast mismatch\ninput: {input:?}");
        }
    }

    #[rstest]
    #[case("func s(){}", true, true, Node::program(vec![Node::stmt("func_def", vec![Node::value("s"), Node::program(vec![])])]))]
    #[case("func(){}", false, false, Node::none())]
    #[case("func{}", false, false, Node::none())]
    #[case("func &s(){}", false, false, Node::none())]
    #[case("func s{}", false, false, Node::none())]
    #[case("func s()", false, true, Node::none())]
    #[case("func s()end", false, false, Node::none())]
    #[case("func s()){}", false, false, Node::none())]
    #[case("func (s)(){}", false, false, Node::none())]
    #[case("func s(){print(\"abc\");}", true, true, Node::program(vec![Node::stmt("func_def", vec![Node::value("s"), Node::program(vec![Node::stmt(ParseToken::Print, vec![Node::stmt("strlit_out", vec![Node::value("\"abc\"")])])])])]))]
    #[case("func s(){print(\"x\");}", true, true, Node::program(vec![Node::stmt("func_def", vec![Node::value("s"), Node::program(vec![Node::stmt(ParseToken::Print, vec![Node::stmt("strlit_out", vec![Node::value("\"x\"")])])])])]))]
    #[case("func s(){print();print();}", true, true, Node::program(vec![Node::stmt("func_def", vec![Node::value("s"), Node::program(vec![Node::stmt(ParseToken::Print, vec![]), Node::stmt(ParseToken::Print, vec![])])])]))]
    #[case("func sss(){print();print();print();}", true, true, Node::program(vec![Node::stmt("func_def", vec![Node::value("sss"), Node::program(vec![Node::stmt(ParseToken::Print, vec![]), Node::stmt(ParseToken::Print, vec![]), Node::stmt(ParseToken::Print, vec![])])])]))]
    #[timeout(Duration::from_secs(1))]
    fn test_func_def_no_expr(
        #[case] input: &str,
        #[case] exp_good: bool,
        #[case] exp_done: bool,
        #[case] exp_ast: Node,
    ) {
        let tokens = Lexer::new(input.to_string()).lex_input();
        let (good, done, ast) = Parser::new(tokens).parse();
        assert_eq!(good, exp_good, "good flag mismatch\ninput: {input:?}");
        assert_eq!(done, exp_done, "done flag mismatch\ninput: {input:?}");
        if exp_good && exp_done {
            assert_eq!(ast, exp_ast, "ast mismatch\ninput: {input:?}");
        }
    }

    #[rstest]
    #[case("ff();", true, true, Node::program(vec![Node::stmt("func_call", vec![Node::value("ff")])]))]
    #[case("fffffffffffffffffffffffffffffffff();", true, true, Node::program(vec![Node::stmt("func_call", vec![Node::value("fffffffffffffffffffffffffffffffff")])]))]
    #[case("ff();gg();", true, true, Node::program(vec![Node::stmt("func_call", vec![Node::value("ff")]), Node::stmt("func_call", vec![Node::value("gg")])]))]
    #[case("ff()", false, true, Node::none())]
    #[case("ff);", false, false, Node::none())]
    #[case("ff(;", false, false, Node::none())]
    #[case("ff(();", false, false, Node::none())]
    #[case("ff());", false, false, Node::none())]
    #[case("ff()();", false, false, Node::none())]
    #[case("ff gg();", false, false, Node::none())]
    #[case("(ff)();", true, false, Node::none())]
    #[case("ff(a);", false, false, Node::none())]
    #[case("ff(\"abc\");", false, false, Node::none())]
    #[case("ff(2);", false, false, Node::none())]
    #[timeout(Duration::from_secs(1))]
    fn test_function_call_stmt(
        #[case] input: &str,
        #[case] exp_good: bool,
        #[case] exp_done: bool,
        #[case] exp_ast: Node,
    ) {
        let tokens = Lexer::new(input.to_string()).lex_input();
        let (good, done, ast) = Parser::new(tokens).parse();
        assert_eq!(good, exp_good, "good flag mismatch\ninput: {input:?}");
        assert_eq!(done, exp_done, "done flag mismatch\ninput: {input:?}");
        if exp_good && exp_done {
            assert_eq!(ast, exp_ast, "ast mismatch\ninput: {input:?}");
        }
    }

    #[rstest]
    #[case("while(1){}", true, true, Node::program(vec![Node::stmt(ParseToken::While, vec![Node::value("1"), Node::program(vec![])])]))]
    #[case("while(1){print();}", true, true, Node::program(vec![Node::stmt(ParseToken::While, vec![Node::value("1"), Node::program(vec![Node::stmt(ParseToken::Print, vec![])])])]))]
    #[case("while(){}", false, false, Node::none())]
    #[case("while{}", false, false, Node::none())]
    #[case("while(1)", false, true, Node::none())]
    #[case("while while(1){}", false, false, Node::none())]
    #[case("while(1)(1){}", false, false, Node::none())]
    #[case("while(1){{}", false, false, Node::none())]
    #[case("while(1){}}", true, false, Node::none())]
    #[timeout(Duration::from_secs(1))]
    fn test_while_loop_simple_expr(
        #[case] input: &str,
        #[case] exp_good: bool,
        #[case] exp_done: bool,
        #[case] exp_ast: Node,
    ) {
        let tokens = Lexer::new(input.to_string()).lex_input();
        let (good, done, ast) = Parser::new(tokens).parse();
        assert_eq!(good, exp_good, "good flag mismatch\ninput: {input:?}");
        assert_eq!(done, exp_done, "done flag mismatch\ninput: {input:?}");
        if exp_good && exp_done {
            assert_eq!(ast, exp_ast, "ast mismatch\ninput: {input:?}");
        }
    }

    #[rstest]
    #[case("if(1){}", true, true, Node::program(vec![Node::stmt(ParseToken::If, vec![Node::value("1"), Node::program(vec![])])]))]
    #[case("if(1){print();}", true, true, Node::program(vec![Node::stmt(ParseToken::If, vec![Node::value("1"), Node::program(vec![Node::stmt(ParseToken::Print, vec![])])])]))]
    #[case("if(1){}else{}", true, true, Node::program(vec![Node::stmt(ParseToken::If, vec![Node::value("1"), Node::program(vec![]), Node::program(vec![])])]))]
    #[case("if(1){}elsif(1){}", true, true, Node::program(vec![Node::stmt(ParseToken::If, vec![Node::value("1"), Node::program(vec![]), Node::value("1"), Node::program(vec![])])]))]
    #[case("if(){print();}", false, false, Node::none())]
    #[case("if{print();}", false, false, Node::none())]
    #[case("if(1)print();}", false, false, Node::none())]
    #[case("if print();}", false, false, Node::none())]
    #[case("if(1){", false, true, Node::none())]
    #[case("if(1)(1){}", false, false, Node::none())]
    #[case("if(1){}else{}elsif(1){}", true, false, Node::none())]
    #[case("if(1){}}", true, false, Node::none())]
    #[timeout(Duration::from_secs(1))]
    fn test_if_stmt_simple_expr(
        #[case] input: &str,
        #[case] exp_good: bool,
        #[case] exp_done: bool,
        #[case] exp_ast: Node,
    ) {
        let tokens = Lexer::new(input.to_string()).lex_input();
        let (good, done, ast) = Parser::new(tokens).parse();
        assert_eq!(good, exp_good, "good flag mismatch\ninput: {input:?}");
        assert_eq!(done, exp_done, "done flag mismatch\ninput: {input:?}");
        if exp_good && exp_done {
            assert_eq!(ast, exp_ast, "ast mismatch\ninput: {input:?}");
        }
    }

    #[rstest]
    #[case("abc=123;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("abc"), Node::value("123")])]))]
    #[case("abc=xyz;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("abc"), Node::value("xyz")])]))]
    #[case("abc[1]=xyz;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::expr("array_var", vec![Node::value("abc"), Node::value("1")]), Node::value("xyz")])]))]
    #[case("=123;", true, false, Node::none())]
    #[case("123=123;", true, false, Node::none())]
    #[case("else=123;", true, false, Node::none())]
    #[case("abc 123;", false, false, Node::none())]
    #[case("abc==123;", false, false, Node::none())]
    #[case("abc=123", false, true, Node::none())]
    #[case("abc=;", false, false, Node::none())]
    #[case("abc=else;", false, false, Node::none())]
    #[case("abc=1 2;", false, false, Node::none())]
    #[case("abc=1 else;", false, false, Node::none())]
    #[case("x=foo();", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::stmt("func_call", vec![Node::value("foo")])])]))]
    #[case("x=();", false, false, Node::none())]
    #[case("x=1&&2;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("&&", vec![Node::value("1"), Node::value("2")])])]))]
    #[case("x=1 || 2;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("||", vec![Node::value("1"), Node::value("2")])])]))]
    #[case("x=1 + 2;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("+", vec![Node::value("1"), Node::value("2")])])]))]
    #[case("x=1+2;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("+", vec![Node::value("1"), Node::value("2")])])]))]
    #[case("x=a+2;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("+", vec![Node::value("a"), Node::value("2")])])]))]
    #[case("x=1+b;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("+", vec![Node::value("1"), Node::value("b")])])]))]
    #[case("x=a+b;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("+", vec![Node::value("a"), Node::value("b")])])]))]
    #[case("x=1+;", false, false, Node::none())]
    #[case("x=1 - 2;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("-", vec![Node::value("1"), Node::value("2")])])]))]
    #[case("x=1-2;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("-", vec![Node::value("1"), Node::value("2")])])]))]
    #[case("x=1-;", false, false, Node::none())]
    #[case("x=1*2;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("*", vec![Node::value("1"), Node::value("2")])])]))]
    #[case("x=a*2;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("*", vec![Node::value("a"), Node::value("2")])])]))]
    #[case("x=1*b;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("*", vec![Node::value("1"), Node::value("b")])])]))]
    #[case("x=a*b;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("*", vec![Node::value("a"), Node::value("b")])])]))]
    #[case("x=1*;", false, false, Node::none())]
    #[case("x=1/2;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("/", vec![Node::value("1"), Node::value("2")])])]))]
    #[case("x=1/;", false, false, Node::none())]
    #[case("x=1%2;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("%", vec![Node::value("1"), Node::value("2")])])]))]
    #[case("x=1%1;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("%", vec![Node::value("1"), Node::value("1")])])]))]
    #[case("x=1%0;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("%", vec![Node::value("1"), Node::value("0")])])]))]
    #[case("x=1%;", false, false, Node::none())]
    #[case("x=1==2;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("==", vec![Node::value("1"), Node::value("2")])])]))]
    #[case("x=a==2;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("==", vec![Node::value("a"), Node::value("2")])])]))]
    #[case("x=1==b;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("==", vec![Node::value("1"), Node::value("b")])])]))]
    #[case("x=a==b;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("==", vec![Node::value("a"), Node::value("b")])])]))]
    #[case("x=1==;", false, false, Node::none())]
    #[case("x=1!=2;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("!=", vec![Node::value("1"), Node::value("2")])])]))]
    #[case("x=1!=;", false, false, Node::none())]
    #[case("x=1<2;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("<", vec![Node::value("1"), Node::value("2")])])]))]
    #[case("x=1<;", false, false, Node::none())]
    #[case("x=1<=2;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("<=", vec![Node::value("1"), Node::value("2")])])]))]
    #[case("x=1<=;", false, false, Node::none())]
    #[case("x=1>2;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr(">", vec![Node::value("1"), Node::value("2")])])]))]
    #[case("x=1>;", false, false, Node::none())]
    #[case("x=1>=2;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr(">=", vec![Node::value("1"), Node::value("2")])])]))]
    #[case("x=+a;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("+", vec![Node::value("unary"), Node::value("a")])])]))]
    #[case("x=-a;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("-", vec![Node::value("unary"), Node::value("a")])])]))]
    #[case("x=1>=;", false, false, Node::none())]
    #[case("x=(1);", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::value("1")])]))]
    #[case("x=(a);", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::value("a")])]))]
    #[case("x=a[1];", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("array_var", vec![Node::value("a"), Node::value("1")])])]))]
    #[case("x=(1;", false, false, Node::none())]
    #[case("x=a[1;", false, false, Node::none())]
    #[case("x=a 1];", false, false, Node::none())]
    #[case("x=a[];", false, false, Node::none())]
    #[case("x=(x);", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::value("x")])]))]
    #[case("(x)=x;", true, false, Node::none())]
    #[case("x[1]=(x[1]);", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::expr("array_var", vec![Node::value("x"), Node::value("1")]), Node::expr("array_var", vec![Node::value("x"), Node::value("1")])])]))]
    #[case("(x[1])=x[1];", true, false, Node::none())]
    #[case("x=f()();", false, false, Node::none())]
    #[case("x=3();", false, false, Node::none())]
    #[case("x=(x)();", false, false, Node::none())]
    #[timeout(Duration::from_secs(1))]
    fn test_assn_stmt(
        #[case] input: &str,
        #[case] exp_good: bool,
        #[case] exp_done: bool,
        #[case] exp_ast: Node,
    ) {
        let tokens = Lexer::new(input.to_string()).lex_input();
        let (good, done, ast) = Parser::new(tokens).parse();
        assert_eq!(good, exp_good, "good flag mismatch\ninput: {input:?}");
        assert_eq!(done, exp_done, "done flag mismatch\ninput: {input:?}");
        if exp_good && exp_done {
            assert_eq!(ast, exp_ast, "ast mismatch\ninput: {input:?}");
        }
    }

    #[rstest]
    #[case("return x;", true, true, Node::program(vec![Node::stmt(ParseToken::Return, vec![Node::value("x")])]))]
    #[case("return x", false, true, Node::none())]
    #[case("return -34;", true, true, Node::program(vec![Node::stmt(ParseToken::Return, vec![Node::expr("-", vec![Node::value("unary"), Node::value("34")])])]))]
    #[case("return;", false, false, Node::none())]
    #[case("return(x);", true, true, Node::program(vec![Node::stmt(ParseToken::Return, vec![Node::value("x")])]))]
    #[case("return(3+1<=4*(x-y));", true, true, Node::program(vec![Node::stmt(ParseToken::Return, vec![Node::expr("<=", vec![Node::expr("+", vec![Node::value("3"), Node::value("1")]), Node::expr("*", vec![Node::value("4"), Node::expr("-", vec![Node::value("x"), Node::value("y")])])])])]))]
    #[timeout(Duration::from_secs(1))]
    fn test_return_stmt(
        #[case] input: &str,
        #[case] exp_good: bool,
        #[case] exp_done: bool,
        #[case] exp_ast: Node,
    ) {
        let tokens = Lexer::new(input.to_string()).lex_input();
        let (good, done, ast) = Parser::new(tokens).parse();
        assert_eq!(good, exp_good, "good flag mismatch\ninput: {input:?}");
        assert_eq!(done, exp_done, "done flag mismatch\ninput: {input:?}");
        if exp_good && exp_done {
            assert_eq!(ast, exp_ast, "ast mismatch\ninput: {input:?}");
        }
    }

    #[rstest]
    #[case("++abc;", true, true, Node::program(vec![Node::stmt(ParseToken::Inc, vec![Node::value("abc")])]))]
    #[case("--zz;", true, true, Node::program(vec![Node::stmt(ParseToken::Dec, vec![Node::value("zz")])]))]
    #[case("++x[1];", true, true, Node::program(vec![Node::stmt(ParseToken::Inc, vec![Node::expr("array_var", vec![Node::value("x"), Node::value("1")])])]))]
    #[case("--y[2];", true, true, Node::program(vec![Node::stmt(ParseToken::Dec, vec![Node::expr("array_var", vec![Node::value("y"), Node::value("2")])])]))]
    #[case("++aa[(n+1)*3];", true, true, Node::program(vec![Node::stmt(ParseToken::Inc, vec![Node::expr("array_var", vec![Node::value("aa"), Node::expr("*", vec![Node::expr("+", vec![Node::value("n"), Node::value("1")]), Node::value("3")])])])]))]
    #[case("--bb[4/(2-k)];", true, true, Node::program(vec![Node::stmt(ParseToken::Dec, vec![Node::expr("array_var", vec![Node::value("bb"), Node::expr("/", vec![Node::value("4"), Node::expr("-", vec![Node::value("2"), Node::value("k")])])])])]))]
    #[case("++abc", false, true, Node::none())]
    #[case("--zz", false, true, Node::none())]
    #[case("++;", false, false, Node::none())]
    #[case("--;", false, false, Node::none())]
    #[case("++a+b;", false, false, Node::none())]
    #[case("--c-d;", false, false, Node::none())]
    #[case("+++b;", false, false, Node::none())]
    #[case("---d;", false, false, Node::none())]
    #[case("++(b);", false, false, Node::none())]
    #[case("--(d);", false, false, Node::none())]
    #[case("++!bl", false, false, Node::none())]
    #[case("--!d;", false, false, Node::none())]
    #[case("print(1+(++abc));", false, false, Node::none())]
    #[case("print((++abc)-1);", false, false, Node::none())]
    #[case("qq++;", false, false, Node::none())]
    #[case("ww--;", false, false, Node::none())]
    #[timeout(Duration::from_secs(1))]
    fn test_inc_dec(
        #[case] input: &str,
        #[case] exp_good: bool,
        #[case] exp_done: bool,
        #[case] exp_ast: Node,
    ) {
        let tokens = Lexer::new(input.to_string()).lex_input();
        let (good, done, ast) = Parser::new(tokens).parse();
        assert_eq!(good, exp_good, "good flag mismatch\ninput: {input:?}");
        assert_eq!(done, exp_done, "done flag mismatch\ninput: {input:?}");
        if exp_good && exp_done {
            assert_eq!(ast, exp_ast, "ast mismatch\ninput: {input:?}");
        }
    }

    #[rstest]
    #[case("print(x);", true, true, Node::program(vec![Node::stmt(ParseToken::Print, vec![Node::value("x")])]))]
    #[case("print(chr(65));", true, true, Node::program(vec![Node::stmt(ParseToken::Print, vec![Node::stmt(ParseToken::ChrCall, vec![Node::value("65")])])]))]
    #[case("print(chr(1),chr(2),chr(3));", true, true, Node::program(vec![Node::stmt(ParseToken::Print, vec![Node::stmt(ParseToken::ChrCall, vec![Node::value("1")]), Node::stmt(ParseToken::ChrCall, vec![Node::value("2")]), Node::stmt(ParseToken::ChrCall, vec![Node::value("3")])])]))]
    #[case("print(\"a b\", chr(1+2), a*4);", true, true, Node::program(vec![Node::stmt(ParseToken::Print, vec![Node::stmt("strlit_out", vec![Node::value("\"a b\"")]), Node::stmt(ParseToken::ChrCall, vec![Node::expr("+", vec![Node::value("1"), Node::value("2")])]), Node::expr("*", vec![Node::value("a"), Node::value("4")])])]))]
    #[case("print(chr(1-2), \"a b\", 4/a);", true, true, Node::program(vec![Node::stmt(ParseToken::Print, vec![Node::stmt(ParseToken::ChrCall, vec![Node::expr("-", vec![Node::value("1"), Node::value("2")])]), Node::stmt("strlit_out", vec![Node::value("\"a b\"")]), Node::expr("/", vec![Node::value("4"), Node::value("a")])])]))]
    #[case("print(a+xyz_3[b*(c==d-f)]%g<=h);", true, true, Node::program(vec![Node::stmt(ParseToken::Print, vec![Node::expr("<=", vec![Node::expr("+", vec![Node::value("a"), Node::expr("%", vec![Node::expr("array_var", vec![Node::value("xyz_3"), Node::expr("*", vec![Node::value("b"), Node::expr("==", vec![Node::value("c"), Node::expr("-", vec![Node::value("d"), Node::value("f")])])])]), Node::value("g")])]), Node::value("h")])])]))]
    #[case("print(1)", false, true, Node::none())]
    #[timeout(Duration::from_secs(1))]
    fn test_print_stmt_with_expr(
        #[case] input: &str,
        #[case] exp_good: bool,
        #[case] exp_done: bool,
        #[case] exp_ast: Node,
    ) {
        let tokens = Lexer::new(input.to_string()).lex_input();
        let (good, done, ast) = Parser::new(tokens).parse();
        assert_eq!(good, exp_good, "good flag mismatch\ninput: {input:?}");
        assert_eq!(done, exp_done, "done flag mismatch\ninput: {input:?}");
        if exp_good && exp_done {
            assert_eq!(ast, exp_ast, "ast mismatch\ninput: {input:?}");
        }
    }

    #[rstest]
    #[case("func q(){print(abc+3);}", true, true, Node::program(vec![Node::stmt("func_def", vec![Node::value("q"), Node::program(vec![Node::stmt(ParseToken::Print, vec![Node::expr("+", vec![Node::value("abc"), Node::value("3")])])])])]))]
    #[case("func qq(){print(a+x[b*(c==d-f)]%g<=h);}", true, true, Node::program(vec![Node::stmt("func_def", vec![Node::value("qq"), Node::program(vec![Node::stmt(ParseToken::Print, vec![Node::expr("<=", vec![Node::expr("+", vec![Node::value("a"), Node::expr("%", vec![Node::expr("array_var", vec![Node::value("x"), Node::expr("*", vec![Node::value("b"), Node::expr("==", vec![Node::value("c"), Node::expr("-", vec![Node::value("d"), Node::value("f")])])])]), Node::value("g")])]), Node::value("h")])])])])]))]
    #[timeout(Duration::from_secs(1))]
    fn test_func_def_with_expr(
        #[case] input: &str,
        #[case] exp_good: bool,
        #[case] exp_done: bool,
        #[case] exp_ast: Node,
    ) {
        let tokens = Lexer::new(input.to_string()).lex_input();
        let (good, done, ast) = Parser::new(tokens).parse();
        assert_eq!(good, exp_good, "good flag mismatch\ninput: {input:?}");
        assert_eq!(done, exp_done, "done flag mismatch\ninput: {input:?}");
        if exp_good && exp_done {
            assert_eq!(ast, exp_ast, "ast mismatch\ninput: {input:?}");
        }
    }

    #[rstest]
    #[case("x=1&&2&&3&&4&&5;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("&&", vec![Node::expr("&&", vec![Node::expr("&&", vec![Node::expr("&&", vec![Node::value("1"), Node::value("2")]), Node::value("3")]), Node::value("4")]), Node::value("5")])])]))]
    #[case("x=1 || 2 || 3 || 4 || 5;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("||", vec![Node::expr("||", vec![Node::expr("||", vec![Node::expr("||", vec![Node::value("1"), Node::value("2")]), Node::value("3")]), Node::value("4")]), Node::value("5")])])]))]
    #[case("x=1+2+3+4+5;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("+", vec![Node::expr("+", vec![Node::expr("+", vec![Node::expr("+", vec![Node::value("1"), Node::value("2")]), Node::value("3")]), Node::value("4")]), Node::value("5")])])]))]
    #[case("x=1-2-3-4-5;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("-", vec![Node::expr("-", vec![Node::expr("-", vec![Node::expr("-", vec![Node::value("1"), Node::value("2")]), Node::value("3")]), Node::value("4")]), Node::value("5")])])]))]
    #[case("x=1*2*3*4*5;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("*", vec![Node::expr("*", vec![Node::expr("*", vec![Node::expr("*", vec![Node::value("1"), Node::value("2")]), Node::value("3")]), Node::value("4")]), Node::value("5")])])]))]
    #[case("x=1/2/3/4/5;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("/", vec![Node::expr("/", vec![Node::expr("/", vec![Node::expr("/", vec![Node::value("1"), Node::value("2")]), Node::value("3")]), Node::value("4")]), Node::value("5")])])]))]
    #[case("x=1%2%3%4%5;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("%", vec![Node::expr("%", vec![Node::expr("%", vec![Node::expr("%", vec![Node::value("1"), Node::value("2")]), Node::value("3")]), Node::value("4")]), Node::value("5")])])]))]
    #[case("x=1==2==3==4==5;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("==", vec![Node::expr("==", vec![Node::expr("==", vec![Node::expr("==", vec![Node::value("1"), Node::value("2")]), Node::value("3")]), Node::value("4")]), Node::value("5")])])]))]
    #[case("x=1!=2!=3!=4!=5;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("!=", vec![Node::expr("!=", vec![Node::expr("!=", vec![Node::expr("!=", vec![Node::value("1"), Node::value("2")]), Node::value("3")]), Node::value("4")]), Node::value("5")])])]))]
    #[case("x=1<2<3<4<5;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("<", vec![Node::expr("<", vec![Node::expr("<", vec![Node::expr("<", vec![Node::value("1"), Node::value("2")]), Node::value("3")]), Node::value("4")]), Node::value("5")])])]))]
    #[case("x=1<=2<=3<=4<=5;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("<=", vec![Node::expr("<=", vec![Node::expr("<=", vec![Node::expr("<=", vec![Node::value("1"), Node::value("2")]), Node::value("3")]), Node::value("4")]), Node::value("5")])])]))]
    #[case("x=1>2>3>4>5;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr(">", vec![Node::expr(">", vec![Node::expr(">", vec![Node::expr(">", vec![Node::value("1"), Node::value("2")]), Node::value("3")]), Node::value("4")]), Node::value("5")])])]))]
    #[case("x=1>=2>=3>=4>=5;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr(">=", vec![Node::expr(">=", vec![Node::expr(">=", vec![Node::expr(">=", vec![Node::value("1"), Node::value("2")]), Node::value("3")]), Node::value("4")]), Node::value("5")])])]))]
    #[case("x=! ! ! ! a;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("!", vec![Node::value("unary"), Node::expr("!", vec![Node::value("unary"), Node::expr("!", vec![Node::value("unary"), Node::expr("!", vec![Node::value("unary"), Node::value("a")])])])])])]))]
    #[case("x=+ + + +a;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("+", vec![Node::value("unary"), Node::expr("+", vec![Node::value("unary"), Node::expr("+", vec![Node::value("unary"), Node::expr("+", vec![Node::value("unary"), Node::value("a")])])])])])]))]
    #[case("x=- - - -a;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("-", vec![Node::value("unary"), Node::expr("-", vec![Node::value("unary"), Node::expr("-", vec![Node::value("unary"), Node::expr("-", vec![Node::value("unary"), Node::value("a")])])])])])]))]
    #[case("x=a && b || c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("||", vec![Node::expr("&&", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])]))]
    #[case("x=a && b==c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("&&", vec![Node::value("a"), Node::expr("==", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a && b!=c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("&&", vec![Node::value("a"), Node::expr("!=", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a && b<c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("&&", vec![Node::value("a"), Node::expr("<", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a && b<=c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("&&", vec![Node::value("a"), Node::expr("<=", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a && b>c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("&&", vec![Node::value("a"), Node::expr(">", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a && b>=c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("&&", vec![Node::value("a"), Node::expr(">=", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a && b+c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("&&", vec![Node::value("a"), Node::expr("+", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a && b-c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("&&", vec![Node::value("a"), Node::expr("-", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a && b*c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("&&", vec![Node::value("a"), Node::expr("*", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a && b/c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("&&", vec![Node::value("a"), Node::expr("/", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a && b%c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("&&", vec![Node::value("a"), Node::expr("%", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a || b && c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("&&", vec![Node::expr("||", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])]))]
    #[case("x=a || b==c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("||", vec![Node::value("a"), Node::expr("==", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a || b!=c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("||", vec![Node::value("a"), Node::expr("!=", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a || b<c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("||", vec![Node::value("a"), Node::expr("<", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a || b<=c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("||", vec![Node::value("a"), Node::expr("<=", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a || b>c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("||", vec![Node::value("a"), Node::expr(">", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a || b>=c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("||", vec![Node::value("a"), Node::expr(">=", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a || b+c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("||", vec![Node::value("a"), Node::expr("+", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a || b-c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("||", vec![Node::value("a"), Node::expr("-", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a || b*c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("||", vec![Node::value("a"), Node::expr("*", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a || b/c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("||", vec![Node::value("a"), Node::expr("/", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a || b%c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("||", vec![Node::value("a"), Node::expr("%", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a==b>c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr(">", vec![Node::expr("==", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])]))]
    #[case("x=a==b+c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("==", vec![Node::value("a"), Node::expr("+", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a==b-c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("==", vec![Node::value("a"), Node::expr("-", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a==b*c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("==", vec![Node::value("a"), Node::expr("*", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a==b/c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("==", vec![Node::value("a"), Node::expr("/", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a==b%c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("==", vec![Node::value("a"), Node::expr("%", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a>b==c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("==", vec![Node::expr(">", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])]))]
    #[case("x=a>b+c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr(">", vec![Node::value("a"), Node::expr("+", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a>b-c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr(">", vec![Node::value("a"), Node::expr("-", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a>b*c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr(">", vec![Node::value("a"), Node::expr("*", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a>b/c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr(">", vec![Node::value("a"), Node::expr("/", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a>b%c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr(">", vec![Node::value("a"), Node::expr("%", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a+b==c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("==", vec![Node::expr("+", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])]))]
    #[case("x=a+b>c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr(">", vec![Node::expr("+", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])]))]
    #[case("x=a+b-c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("-", vec![Node::expr("+", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])]))]
    #[case("x=a+b*c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("+", vec![Node::value("a"), Node::expr("*", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a+b/c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("+", vec![Node::value("a"), Node::expr("/", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a+b%c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("+", vec![Node::value("a"), Node::expr("%", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a-b==c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("==", vec![Node::expr("-", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])]))]
    #[case("x=a-b>c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr(">", vec![Node::expr("-", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])]))]
    #[case("x=a-b+c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("+", vec![Node::expr("-", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])]))]
    #[case("x=a-b*c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("-", vec![Node::value("a"), Node::expr("*", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a-b/c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("-", vec![Node::value("a"), Node::expr("/", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a-b%c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("-", vec![Node::value("a"), Node::expr("%", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a*b==c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("==", vec![Node::expr("*", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])]))]
    #[case("x=a*b>c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr(">", vec![Node::expr("*", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])]))]
    #[case("x=a*b+c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("+", vec![Node::expr("*", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])]))]
    #[case("x=a*b-c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("-", vec![Node::expr("*", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])]))]
    #[case("x=a*b/c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("/", vec![Node::expr("*", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])]))]
    #[case("x=a*b%c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("%", vec![Node::expr("*", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])]))]
    #[case("x=a/b==c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("==", vec![Node::expr("/", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])]))]
    #[case("x=a/b>c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr(">", vec![Node::expr("/", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])]))]
    #[case("x=a/b+c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("+", vec![Node::expr("/", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])]))]
    #[case("x=a/b-c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("-", vec![Node::expr("/", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])]))]
    #[case("x=a/b*c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("*", vec![Node::expr("/", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])]))]
    #[case("x=a/b%c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("%", vec![Node::expr("/", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])]))]
    #[case("x=a%b==c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("==", vec![Node::expr("%", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])]))]
    #[case("x=a%b>c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr(">", vec![Node::expr("%", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])]))]
    #[case("x=a%b+c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("+", vec![Node::expr("%", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])]))]
    #[case("x=a%b-c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("-", vec![Node::expr("%", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])]))]
    #[case("x=a%b*c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("*", vec![Node::expr("%", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])]))]
    #[case("x=a%b/c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("/", vec![Node::expr("%", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])]))]
    #[case("x=! a && b;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("&&", vec![Node::expr("!", vec![Node::value("unary"), Node::value("a")]), Node::value("b")])])]))]
    #[case("x=! a || b;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("||", vec![Node::expr("!", vec![Node::value("unary"), Node::value("a")]), Node::value("b")])])]))]
    #[case("x=! a==b;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("==", vec![Node::expr("!", vec![Node::value("unary"), Node::value("a")]), Node::value("b")])])]))]
    #[case("x=! a!=b;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("!=", vec![Node::expr("!", vec![Node::value("unary"), Node::value("a")]), Node::value("b")])])]))]
    #[case("x=! a<b;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("<", vec![Node::expr("!", vec![Node::value("unary"), Node::value("a")]), Node::value("b")])])]))]
    #[case("x=! a<=b;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("<=", vec![Node::expr("!", vec![Node::value("unary"), Node::value("a")]), Node::value("b")])])]))]
    #[case("x=! a>b;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr(">", vec![Node::expr("!", vec![Node::value("unary"), Node::value("a")]), Node::value("b")])])]))]
    #[case("x=! a>=b;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr(">=", vec![Node::expr("!", vec![Node::value("unary"), Node::value("a")]), Node::value("b")])])]))]
    #[case("x=! a+b;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("+", vec![Node::expr("!", vec![Node::value("unary"), Node::value("a")]), Node::value("b")])])]))]
    #[case("x=! a-b;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("-", vec![Node::expr("!", vec![Node::value("unary"), Node::value("a")]), Node::value("b")])])]))]
    #[case("x=! a*b;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("*", vec![Node::expr("!", vec![Node::value("unary"), Node::value("a")]), Node::value("b")])])]))]
    #[case("x=! a/b;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("/", vec![Node::expr("!", vec![Node::value("unary"), Node::value("a")]), Node::value("b")])])]))]
    #[case("x=! a%b;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("%", vec![Node::expr("!", vec![Node::value("unary"), Node::value("a")]), Node::value("b")])])]))]
    #[case("x=a!=+b;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("!=", vec![Node::value("a"), Node::expr("+", vec![Node::value("unary"), Node::value("b")])])])]))]
    #[case("x=-a<c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("<", vec![Node::expr("-", vec![Node::value("unary"), Node::value("a")]), Node::value("c")])])]))]
    #[case("x=a+ +b;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("+", vec![Node::value("a"), Node::expr("+", vec![Node::value("unary"), Node::value("b")])])])]))]
    #[case("x=a+-b;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("+", vec![Node::value("a"), Node::expr("-", vec![Node::value("unary"), Node::value("b")])])])]))]
    #[case("x=+a+b;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("+", vec![Node::expr("+", vec![Node::value("unary"), Node::value("a")]), Node::value("b")])])]))]
    #[case("x=-a+b;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("+", vec![Node::expr("-", vec![Node::value("unary"), Node::value("a")]), Node::value("b")])])]))]
    #[case("x=a-+b;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("-", vec![Node::value("a"), Node::expr("+", vec![Node::value("unary"), Node::value("b")])])])]))]
    #[case("x=a- -b;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("-", vec![Node::value("a"), Node::expr("-", vec![Node::value("unary"), Node::value("b")])])])]))]
    #[case("x=+a-b;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("-", vec![Node::expr("+", vec![Node::value("unary"), Node::value("a")]), Node::value("b")])])]))]
    #[case("x=-a-b;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("-", vec![Node::expr("-", vec![Node::value("unary"), Node::value("a")]), Node::value("b")])])]))]
    #[case("x=a*-b;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("*", vec![Node::value("a"), Node::expr("-", vec![Node::value("unary"), Node::value("b")])])])]))]
    #[case("x=+a*c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("*", vec![Node::expr("+", vec![Node::value("unary"), Node::value("a")]), Node::value("c")])])]))]
    #[case("x=a/+b;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("/", vec![Node::value("a"), Node::expr("+", vec![Node::value("unary"), Node::value("b")])])])]))]
    #[case("x=-a/c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("/", vec![Node::expr("-", vec![Node::value("unary"), Node::value("a")]), Node::value("c")])])]))]
    #[case("x=a%-b;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("%", vec![Node::value("a"), Node::expr("-", vec![Node::value("unary"), Node::value("b")])])])]))]
    #[case("x=+a%c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("%", vec![Node::expr("+", vec![Node::value("unary"), Node::value("a")]), Node::value("c")])])]))]
    #[case("x=1 && (2 && 3 && 4) && 5;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("&&", vec![Node::expr("&&", vec![Node::value("1"), Node::expr("&&", vec![Node::expr("&&", vec![Node::value("2"), Node::value("3")]), Node::value("4")])]), Node::value("5")])])]))]
    #[case("x=1 || (2 || 3 || 4) || 5;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("||", vec![Node::expr("||", vec![Node::value("1"), Node::expr("||", vec![Node::expr("||", vec![Node::value("2"), Node::value("3")]), Node::value("4")])]), Node::value("5")])])]))]
    #[case("x=1==(2==3==4)==5;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("==", vec![Node::expr("==", vec![Node::value("1"), Node::expr("==", vec![Node::expr("==", vec![Node::value("2"), Node::value("3")]), Node::value("4")])]), Node::value("5")])])]))]
    #[case("x=1!=(2!=3!=4)!=5;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("!=", vec![Node::expr("!=", vec![Node::value("1"), Node::expr("!=", vec![Node::expr("!=", vec![Node::value("2"), Node::value("3")]), Node::value("4")])]), Node::value("5")])])]))]
    #[case("x=1<(2<3<4)<5;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("<", vec![Node::expr("<", vec![Node::value("1"), Node::expr("<", vec![Node::expr("<", vec![Node::value("2"), Node::value("3")]), Node::value("4")])]), Node::value("5")])])]))]
    #[case("x=1<=(2<=3<=4)<=5;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("<=", vec![Node::expr("<=", vec![Node::value("1"), Node::expr("<=", vec![Node::expr("<=", vec![Node::value("2"), Node::value("3")]), Node::value("4")])]), Node::value("5")])])]))]
    #[case("x=1>(2>3>4)>5;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr(">", vec![Node::expr(">", vec![Node::value("1"), Node::expr(">", vec![Node::expr(">", vec![Node::value("2"), Node::value("3")]), Node::value("4")])]), Node::value("5")])])]))]
    #[case("x=1>=(2>=3>=4)>=5;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr(">=", vec![Node::expr(">=", vec![Node::value("1"), Node::expr(">=", vec![Node::expr(">=", vec![Node::value("2"), Node::value("3")]), Node::value("4")])]), Node::value("5")])])]))]
    #[case("x=1+(2+3+4)+5;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("+", vec![Node::expr("+", vec![Node::value("1"), Node::expr("+", vec![Node::expr("+", vec![Node::value("2"), Node::value("3")]), Node::value("4")])]), Node::value("5")])])]))]
    #[case("x=1-(2-3-4)-5;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("-", vec![Node::expr("-", vec![Node::value("1"), Node::expr("-", vec![Node::expr("-", vec![Node::value("2"), Node::value("3")]), Node::value("4")])]), Node::value("5")])])]))]
    #[case("x=1*(2*3*4)*5;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("*", vec![Node::expr("*", vec![Node::value("1"), Node::expr("*", vec![Node::expr("*", vec![Node::value("2"), Node::value("3")]), Node::value("4")])]), Node::value("5")])])]))]
    #[case("x=1/(2/3/4)/5;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("/", vec![Node::expr("/", vec![Node::value("1"), Node::expr("/", vec![Node::expr("/", vec![Node::value("2"), Node::value("3")]), Node::value("4")])]), Node::value("5")])])]))]
    #[case("x=1%(2%3%4)%5;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("%", vec![Node::expr("%", vec![Node::value("1"), Node::expr("%", vec![Node::expr("%", vec![Node::value("2"), Node::value("3")]), Node::value("4")])]), Node::value("5")])])]))]
    #[case("x=(a==b)+c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("+", vec![Node::expr("==", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])]))]
    #[case("x=(a!=b)-c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("-", vec![Node::expr("!=", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])]))]
    #[case("x=(a<b)*c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("*", vec![Node::expr("<", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])]))]
    #[case("x=(a<=b)/c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("/", vec![Node::expr("<=", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])]))]
    #[case("x=(a>b)%c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("%", vec![Node::expr(">", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])]))]
    #[case("x=a+(b>=c);", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("+", vec![Node::value("a"), Node::expr(">=", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=(a-b)*c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("*", vec![Node::expr("-", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])]))]
    #[case("x=(a+b)%c;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("%", vec![Node::expr("+", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])]))]
    #[case("x=a*(b==c);", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("*", vec![Node::value("a"), Node::expr("==", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a/(b!=c);", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("/", vec![Node::value("a"), Node::expr("!=", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=a%(b<c);", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("%", vec![Node::value("a"), Node::expr("<", vec![Node::value("b"), Node::value("c")])])])]))]
    #[case("x=+(a<=b);", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("+", vec![Node::value("unary"), Node::expr("<=", vec![Node::value("a"), Node::value("b")])])])]))]
    #[case("x=-(a>b);", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("-", vec![Node::value("unary"), Node::expr(">", vec![Node::value("a"), Node::value("b")])])])]))]
    #[case("x=+(a+b);", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("+", vec![Node::value("unary"), Node::expr("+", vec![Node::value("a"), Node::value("b")])])])]))]
    #[case("x=-(a-b);", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("-", vec![Node::value("unary"), Node::expr("-", vec![Node::value("a"), Node::value("b")])])])]))]
    #[case("x=+(a*b);", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("+", vec![Node::value("unary"), Node::expr("*", vec![Node::value("a"), Node::value("b")])])])]))]
    #[case("x=-(a/b);", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("-", vec![Node::value("unary"), Node::expr("/", vec![Node::value("a"), Node::value("b")])])])]))]
    #[case("x=+(a%b);", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("+", vec![Node::value("unary"), Node::expr("%", vec![Node::value("a"), Node::value("b")])])])]))]
    #[timeout(Duration::from_secs(1))]
    fn test_expr_prec_assoc(
        #[case] input: &str,
        #[case] exp_good: bool,
        #[case] exp_done: bool,
        #[case] exp_ast: Node,
    ) {
        let tokens = Lexer::new(input.to_string()).lex_input();
        let (good, done, ast) = Parser::new(tokens).parse();
        assert_eq!(good, exp_good, "good flag mismatch\ninput: {input:?}");
        assert_eq!(done, exp_done, "done flag mismatch\ninput: {input:?}");
        if exp_good && exp_done {
            assert_eq!(ast, exp_ast, "ast mismatch\ninput: {input:?}");
        }
    }

    #[rstest]
    #[case("x=readint();", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::stmt("readint", vec![])])]))]
    #[case("x=readint(y);", false, false, Node::none())]
    #[case("x=readint;", false, false, Node::none())]
    #[case("x=readint);", false, false, Node::none())]
    #[case("x=readint(;", false, false, Node::none())]
    #[case("readint();", true, false, Node::none())]
    #[case("x=readint();y=readint();", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::stmt("readint", vec![])]), Node::stmt(ParseToken::Assn, vec![Node::value("y"), Node::stmt("readint", vec![])])]))]
    #[case("x=rnd(1);", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::stmt(ParseToken::RndCall, vec![Node::value("1")])])]))]
    #[case("x=rnd();", false, false, Node::none())]
    #[case("x=rnd;", false, false, Node::none())]
    #[case("x=rnd 1);", false, false, Node::none())]
    #[case("x=rnd(1;", false, false, Node::none())]
    #[case("rnd(1);", true, false, Node::none())]
    #[case("x=rnd(a);y=rnd(0);", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::stmt(ParseToken::RndCall, vec![Node::value("a")])]), Node::stmt(ParseToken::Assn, vec![Node::value("y"), Node::stmt(ParseToken::RndCall, vec![Node::value("0")])])]))]
    #[timeout(Duration::from_secs(1))]
    fn test_readint_rnd(
        #[case] input: &str,
        #[case] exp_good: bool,
        #[case] exp_done: bool,
        #[case] exp_ast: Node,
    ) {
        let tokens = Lexer::new(input.to_string()).lex_input();
        let (good, done, ast) = Parser::new(tokens).parse();
        assert_eq!(good, exp_good, "good flag mismatch\ninput: {input:?}");
        assert_eq!(done, exp_done, "done flag mismatch\ninput: {input:?}");
        if exp_good && exp_done {
            assert_eq!(ast, exp_ast, "ast mismatch\ninput: {input:?}");
        }
    }

    #[rstest]
    #[case("a[1] = 2;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::expr("array_var", vec![Node::value("a"), Node::value("1")]), Node::value("2")])]))]
    #[case("a = b[2];", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("a"), Node::expr("array_var", vec![Node::value("b"), Node::value("2")])])]))]
    #[case("abc[5*2+a]=bcd[5<=1/4]/cde[! 0>x];", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::expr("array_var", vec![Node::value("abc"), Node::expr("+", vec![Node::expr("*", vec![Node::value("5"), Node::value("2")]), Node::value("a")])]), Node::expr("/", vec![Node::expr("array_var", vec![Node::value("bcd"), Node::expr("<=", vec![Node::value("5"), Node::expr("/", vec![Node::value("1"), Node::value("4")])])]), Node::expr("array_var", vec![Node::value("cde"), Node::expr(">", vec![Node::expr("!", vec![Node::value("unary"), Node::value("0")]), Node::value("x")])])])])]))]
    #[timeout(Duration::from_secs(1))]
    fn test_array_item(
        #[case] input: &str,
        #[case] exp_good: bool,
        #[case] exp_done: bool,
        #[case] exp_ast: Node,
    ) {
        let tokens = Lexer::new(input.to_string()).lex_input();
        let (good, done, ast) = Parser::new(tokens).parse();
        assert_eq!(good, exp_good, "good flag mismatch\ninput: {input:?}");
        assert_eq!(done, exp_done, "done flag mismatch\ninput: {input:?}");
        if exp_good && exp_done {
            assert_eq!(ast, exp_ast, "ast mismatch\ninput: {input:?}");
        }
    }

    #[rstest]
    #[case("x=a==b+c[x-y[2]]*+d!=e-f/-g<h+i%+j;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("<", vec![Node::expr("!=", vec![Node::expr("==", vec![Node::value("a"), Node::expr("+", vec![Node::value("b"), Node::expr("*", vec![Node::expr("array_var", vec![Node::value("c"), Node::expr("-", vec![Node::value("x"), Node::expr("array_var", vec![Node::value("y"), Node::value("2")])])]), Node::expr("+", vec![Node::value("unary"), Node::value("d")])])])]), Node::expr("-", vec![Node::value("e"), Node::expr("/", vec![Node::value("f"), Node::expr("-", vec![Node::value("unary"), Node::value("g")])])])]), Node::expr("+", vec![Node::value("h"), Node::expr("%", vec![Node::value("i"), Node::expr("+", vec![Node::value("unary"), Node::value("j")])])])])])]))]
    #[case("x=a==b+(c*+(d!=e[2*z]-f/-g)<h+i)%+j;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("==", vec![Node::value("a"), Node::expr("+", vec![Node::value("b"), Node::expr("%", vec![Node::expr("<", vec![Node::expr("*", vec![Node::value("c"), Node::expr("+", vec![Node::value("unary"), Node::expr("!=", vec![Node::value("d"), Node::expr("-", vec![Node::expr("array_var", vec![Node::value("e"), Node::expr("*", vec![Node::value("2"), Node::value("z")])]), Node::expr("/", vec![Node::value("f"), Node::expr("-", vec![Node::value("unary"), Node::value("g")])])])])])]), Node::expr("+", vec![Node::value("h"), Node::value("i")])]), Node::expr("+", vec![Node::value("unary"), Node::value("j")])])])])])]))]
    #[case("x=a[x[y[z]]%4]+ +b*c<=d- -e/f>g+-h%i>=j;", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr(">=", vec![Node::expr(">", vec![Node::expr("<=", vec![Node::expr("+", vec![Node::expr("array_var", vec![Node::value("a"), Node::expr("%", vec![Node::expr("array_var", vec![Node::value("x"), Node::expr("array_var", vec![Node::value("y"), Node::value("z")])]), Node::value("4")])]), Node::expr("*", vec![Node::expr("+", vec![Node::value("unary"), Node::value("b")]), Node::value("c")])]), Node::expr("-", vec![Node::value("d"), Node::expr("/", vec![Node::expr("-", vec![Node::value("unary"), Node::value("e")]), Node::value("f")])])]), Node::expr("+", vec![Node::value("g"), Node::expr("%", vec![Node::expr("-", vec![Node::value("unary"), Node::value("h")]), Node::value("i")])])]), Node::value("j")])])]))]
    #[case("x=a+ +(b*c<=d)- -e/(f>g+-h%i)>=j[-z];", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr(">=", vec![Node::expr("-", vec![Node::expr("+", vec![Node::value("a"), Node::expr("+", vec![Node::value("unary"), Node::expr("<=", vec![Node::expr("*", vec![Node::value("b"), Node::value("c")]), Node::value("d")])])]), Node::expr("/", vec![Node::expr("-", vec![Node::value("unary"), Node::value("e")]), Node::expr(">", vec![Node::value("f"), Node::expr("+", vec![Node::value("g"), Node::expr("%", vec![Node::expr("-", vec![Node::value("unary"), Node::value("h")]), Node::value("i")])])])])]), Node::expr("array_var", vec![Node::value("j"), Node::expr("-", vec![Node::value("unary"), Node::value("z")])])])])]))]
    #[case("print(rnd(readint()==15e3),rnd(rnd(rnd(readint()))));", true, true, Node::program(vec![Node::stmt(ParseToken::Print, vec![Node::stmt(ParseToken::RndCall, vec![Node::expr("==", vec![Node::stmt("readint", vec![]), Node::value("15e3")])]), Node::stmt(ParseToken::RndCall, vec![Node::stmt(ParseToken::RndCall, vec![Node::stmt(ParseToken::RndCall, vec![Node::stmt("readint", vec![])])])])])]))]
    #[case("x=a==b+c*+d!=e-/-g<h+i%+j;", false, false, Node::none())]
    #[case("x=a==b+(c*+(d!=e-f/-g)<h+i)%+;", false, false, Node::none())]
    #[case("x=a++b*c<=d- -e x/f>g+-h%i>=j;", false, false, Node::none())]
    #[case("x=a++b*c<=d)- -e/(f>g+-h%i)>=j;", false, false, Node::none())]
    #[case("x=((a[(b[c[(d[((e[f]))])]])]));", true, true, Node::program(vec![Node::stmt(ParseToken::Assn, vec![Node::value("x"), Node::expr("array_var", vec![Node::value("a"), Node::expr("array_var", vec![Node::value("b"), Node::expr("array_var", vec![Node::value("c"), Node::expr("array_var", vec![Node::value("d"), Node::expr("array_var", vec![Node::value("e"), Node::value("f")])])])])])])]))]
    #[case("x=((a[(b[c[(d[((e[f]))]])])]));", false, false, Node::none())]
    #[case("while((a+b)%d+a()!=1){print();}", true, true, Node::program(vec![Node::stmt(ParseToken::While, vec![Node::expr("!=", vec![Node::expr("+", vec![Node::expr("%", vec![Node::expr("+", vec![Node::value("a"), Node::value("b")]), Node::value("d")]), Node::stmt("func_call", vec![Node::value("a")])]), Node::value("1")]), Node::program(vec![Node::stmt(ParseToken::Print, vec![])])])]))]
    #[timeout(Duration::from_secs(1))]
    fn test_expr_complex(
        #[case] input: &str,
        #[case] exp_good: bool,
        #[case] exp_done: bool,
        #[case] exp_ast: Node,
    ) {
        let tokens = Lexer::new(input.to_string()).lex_input();
        let (good, done, ast) = Parser::new(tokens).parse();
        assert_eq!(good, exp_good, "good flag mismatch\ninput: {input:?}");
        assert_eq!(done, exp_done, "done flag mismatch\ninput: {input:?}");
        if exp_good && exp_done {
            assert_eq!(ast, exp_ast, "ast mismatch\ninput: {input:?}");
        }
    }
