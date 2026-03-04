//! Parser tests — auto-generated from parseit_test.lua
//! Do not edit by hand.

use rstest::rstest;
use std::time::Duration;
use tamandua_rs::parser::{Node, Parser};
use tamandua_rs::lexer::Lexer;

    #[rstest]
    #[case("", true, true, Some(Node::program(vec![])))]
    #[case("return", false, true, None)]
    #[case("elsif", true, false, None)]
    #[case("else", true, false, None)]
    #[case("ab", false, true, None)]
    #[case("ab;", false, false, None)]
    #[case("123", true, false, None)]
    #[case("123;", true, false, None)]
    #[case("\"xyz\"", true, false, None)]
    #[case("<=", true, false, None)]
    #[case("{", true, false, None)]
    #[case("", true, false, None)]
    #[case("\"", true, false, None)]
    #[timeout(Duration::from_secs(1))]
    fn test_simple(
        #[case] input: &str,
        #[case] exp_good: bool,
        #[case] exp_done: bool,
        #[case] exp_ast: Option<Node>,
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
    #[case("print();", true, true, Some(Node::program(vec![Node::stmt("print", vec![])])))]
    #[case("print();print();print();", true, true, Some(Node::program(vec![Node::stmt("print", vec![]), Node::stmt("print", vec![]), Node::stmt("print", vec![])])))]
    #[case("print(\"abc\");", true, true, Some(Node::program(vec![Node::stmt("print", vec![Node::stmt("strlit_out", vec![Node::value("\"abc\"")])])])))]
    #[case("print(\"a\",\"b\",\"c\",\"d\",\"e\");", true, true, Some(Node::program(vec![Node::stmt("print", vec![Node::stmt("strlit_out", vec![Node::value("\"a\"")]), Node::stmt("strlit_out", vec![Node::value("\"b\"")]), Node::stmt("strlit_out", vec![Node::value("\"c\"")]), Node::stmt("strlit_out", vec![Node::value("\"d\"")]), Node::stmt("strlit_out", vec![Node::value("\"e\"")])])])))]
    #[case("print()", false, true, None)]
    #[case("print", false, true, None)]
    #[case("print \"a\";", false, false, None)]
    #[case("print\"a\");", false, false, None)]
    #[case("print(\"a\";", false, false, None)]
    #[case("print(if);", false, false, None)]
    #[case("print(print);", false, false, None)]
    #[case("print(\"a\"\"b\");", false, false, None)]
    #[case("print(,\"a\");", false, false, None)]
    #[case("print(\"a\",);", false, false, None)]
    #[case("print(,);", false, false, None)]
    #[case("print(\"a\",,\"b\");", false, false, None)]
    #[case("print(\"a\")else;", false, false, None)]
    #[case("print(\"a\");else", true, false, None)]
    #[case("println();", true, true, Some(Node::program(vec![Node::stmt("println", vec![])])))]
    #[case("println();println();println();", true, true, Some(Node::program(vec![Node::stmt("println", vec![]), Node::stmt("println", vec![]), Node::stmt("println", vec![])])))]
    #[case("println(\"abc\");", true, true, Some(Node::program(vec![Node::stmt("println", vec![Node::stmt("strlit_out", vec![Node::value("\"abc\"")])])])))]
    #[case("println(\"a\",\"b\",\"c\",\"d\",\"e\");", true, true, Some(Node::program(vec![Node::stmt("println", vec![Node::stmt("strlit_out", vec![Node::value("\"a\"")]), Node::stmt("strlit_out", vec![Node::value("\"b\"")]), Node::stmt("strlit_out", vec![Node::value("\"c\"")]), Node::stmt("strlit_out", vec![Node::value("\"d\"")]), Node::stmt("strlit_out", vec![Node::value("\"e\"")])])])))]
    #[case("println()", false, true, None)]
    #[case("println", false, true, None)]
    #[case("println \"a\";", false, false, None)]
    #[case("println\"a\");", false, false, None)]
    #[case("println(\"a\";", false, false, None)]
    #[case("println(if);", false, false, None)]
    #[case("println(println);", false, false, None)]
    #[case("println(\"a\"\"b\");", false, false, None)]
    #[case("println(,\"a\");", false, false, None)]
    #[case("println(\"a\",);", false, false, None)]
    #[case("println(,);", false, false, None)]
    #[case("println(\"a\",,\"b\");", false, false, None)]
    #[case("println(\"a\")else;", false, false, None)]
    #[case("println(\"a\");else", true, false, None)]
    #[case("print();println();", true, true, Some(Node::program(vec![Node::stmt("print", vec![]), Node::stmt("println", vec![])])))]
    #[case("println();print();", true, true, Some(Node::program(vec![Node::stmt("println", vec![]), Node::stmt("print", vec![])])))]
    #[case("print('a');println('b');", true, true, Some(Node::program(vec![Node::stmt("print", vec![Node::stmt("strlit_out", vec![Node::value("'a'")])]), Node::stmt("println", vec![Node::stmt("strlit_out", vec![Node::value("'b'")])])])))]
    #[case("println('x');print('y');", true, true, Some(Node::program(vec![Node::stmt("println", vec![Node::stmt("strlit_out", vec![Node::value("'x'")])]), Node::stmt("print", vec![Node::stmt("strlit_out", vec![Node::value("'y'")])])])))]
    #[case("\"a\";", true, false, None)]
    #[case("(\"a\");", true, false, None)]
    #[timeout(Duration::from_secs(1))]
    fn test_print_stmt_no_expr(
        #[case] input: &str,
        #[case] exp_good: bool,
        #[case] exp_done: bool,
        #[case] exp_ast: Option<Node>,
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
    #[case("func s(){}", true, true, Some(Node::program(vec![Node::stmt("func_def", vec![Node::value("s"), Node::program(vec![])])])))]
    #[case("func(){}", false, false, None)]
    #[case("func{}", false, false, None)]
    #[case("func &s(){}", false, false, None)]
    #[case("func s{}", false, false, None)]
    #[case("func s()", false, true, None)]
    #[case("func s()end", false, false, None)]
    #[case("func s()){}", false, false, None)]
    #[case("func (s)(){}", false, false, None)]
    #[case("func s(){print(\"abc\");}", true, true, Some(Node::program(vec![Node::stmt("func_def", vec![Node::value("s"), Node::program(vec![Node::stmt("print", vec![Node::stmt("strlit_out", vec![Node::value("\"abc\"")])])])])])))]
    #[case("func s(){print(\"x\");}", true, true, Some(Node::program(vec![Node::stmt("func_def", vec![Node::value("s"), Node::program(vec![Node::stmt("print", vec![Node::stmt("strlit_out", vec![Node::value("\"x\"")])])])])])))]
    #[case("func s(){print();print();}", true, true, Some(Node::program(vec![Node::stmt("func_def", vec![Node::value("s"), Node::program(vec![Node::stmt("print", vec![]), Node::stmt("print", vec![])])])])))]
    #[case("func sss(){print();print();print();}", true, true, Some(Node::program(vec![Node::stmt("func_def", vec![Node::value("sss"), Node::program(vec![Node::stmt("print", vec![]), Node::stmt("print", vec![]), Node::stmt("print", vec![])])])])))]
    #[timeout(Duration::from_secs(1))]
    fn test_func_def_no_expr(
        #[case] input: &str,
        #[case] exp_good: bool,
        #[case] exp_done: bool,
        #[case] exp_ast: Option<Node>,
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
    #[case("ff();", true, true, Some(Node::program(vec![Node::stmt("func_call", vec![Node::value("ff")])])))]
    #[case("fffffffffffffffffffffffffffffffff();", true, true, Some(Node::program(vec![Node::stmt("func_call", vec![Node::value("fffffffffffffffffffffffffffffffff")])])))]
    #[case("ff();gg();", true, true, Some(Node::program(vec![Node::stmt("func_call", vec![Node::value("ff")]), Node::stmt("func_call", vec![Node::value("gg")])])))]
    #[case("ff()", false, true, None)]
    #[case("ff);", false, false, None)]
    #[case("ff(;", false, false, None)]
    #[case("ff(();", false, false, None)]
    #[case("ff());", false, false, None)]
    #[case("ff()();", false, false, None)]
    #[case("ff gg();", false, false, None)]
    #[case("(ff)();", true, false, None)]
    #[case("ff(a);", false, false, None)]
    #[case("ff(\"abc\");", false, false, None)]
    #[case("ff(2);", false, false, None)]
    #[timeout(Duration::from_secs(1))]
    fn test_function_call_stmt(
        #[case] input: &str,
        #[case] exp_good: bool,
        #[case] exp_done: bool,
        #[case] exp_ast: Option<Node>,
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
    #[case("while(1){}", true, true, Some(Node::program(vec![Node::stmt("while", vec![Node::value("1"), Node::program(vec![])])])))]
    #[case("while(1){print();}", true, true, Some(Node::program(vec![Node::stmt("while", vec![Node::value("1"), Node::program(vec![Node::stmt("print", vec![])])])])))]
    #[case("while(){}", false, false, None)]
    #[case("while{}", false, false, None)]
    #[case("while(1)", false, true, None)]
    #[case("while while(1){}", false, false, None)]
    #[case("while(1)(1){}", false, false, None)]
    #[case("while(1){{}", false, false, None)]
    #[case("while(1){}}", true, false, None)]
    #[timeout(Duration::from_secs(1))]
    fn test_while_loop_simple_expr(
        #[case] input: &str,
        #[case] exp_good: bool,
        #[case] exp_done: bool,
        #[case] exp_ast: Option<Node>,
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
    #[case("if(1){}", true, true, Some(Node::program(vec![Node::stmt("if", vec![Node::value("1"), Node::program(vec![])])])))]
    #[case("if(1){print();}", true, true, Some(Node::program(vec![Node::stmt("if", vec![Node::value("1"), Node::program(vec![Node::stmt("print", vec![])])])])))]
    #[case("if(1){}else{}", true, true, Some(Node::program(vec![Node::stmt("if", vec![Node::value("1"), Node::program(vec![]), Node::program(vec![])])])))]
    #[case("if(1){}elsif(1){}", true, true, Some(Node::program(vec![Node::stmt("if", vec![Node::value("1"), Node::program(vec![]), Node::value("1"), Node::program(vec![])])])))]
    #[case("if(){print();}", false, false, None)]
    #[case("if{print();}", false, false, None)]
    #[case("if(1)print();}", false, false, None)]
    #[case("if print();}", false, false, None)]
    #[case("if(1){", false, true, None)]
    #[case("if(1)(1){}", false, false, None)]
    #[case("if(1){}else{}elsif(1){}", true, false, None)]
    #[case("if(1){}}", true, false, None)]
    #[timeout(Duration::from_secs(1))]
    fn test_if_stmt_simple_expr(
        #[case] input: &str,
        #[case] exp_good: bool,
        #[case] exp_done: bool,
        #[case] exp_ast: Option<Node>,
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
    #[case("abc=123;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("abc"), Node::value("123")])])))]
    #[case("abc=xyz;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("abc"), Node::value("xyz")])])))]
    #[case("abc[1]=xyz;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::expr("array_var", vec![Node::value("abc"), Node::value("1")]), Node::value("xyz")])])))]
    #[case("=123;", true, false, None)]
    #[case("123=123;", true, false, None)]
    #[case("else=123;", true, false, None)]
    #[case("abc 123;", false, false, None)]
    #[case("abc==123;", false, false, None)]
    #[case("abc=123", false, true, None)]
    #[case("abc=;", false, false, None)]
    #[case("abc=else;", false, false, None)]
    #[case("abc=1 2;", false, false, None)]
    #[case("abc=1 else;", false, false, None)]
    #[case("x=foo();", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::stmt("func_call", vec![Node::value("foo")])])])))]
    #[case("x=();", false, false, None)]
    #[case("x=1&&2;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("&&", vec![Node::value("1"), Node::value("2")])])])))]
    #[case("x=1 || 2;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("||", vec![Node::value("1"), Node::value("2")])])])))]
    #[case("x=1 + 2;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("+", vec![Node::value("1"), Node::value("2")])])])))]
    #[case("x=1+2;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("+", vec![Node::value("1"), Node::value("2")])])])))]
    #[case("x=a+2;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("+", vec![Node::value("a"), Node::value("2")])])])))]
    #[case("x=1+b;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("+", vec![Node::value("1"), Node::value("b")])])])))]
    #[case("x=a+b;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("+", vec![Node::value("a"), Node::value("b")])])])))]
    #[case("x=1+;", false, false, None)]
    #[case("x=1 - 2;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("-", vec![Node::value("1"), Node::value("2")])])])))]
    #[case("x=1-2;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("-", vec![Node::value("1"), Node::value("2")])])])))]
    #[case("x=1-;", false, false, None)]
    #[case("x=1*2;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("*", vec![Node::value("1"), Node::value("2")])])])))]
    #[case("x=a*2;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("*", vec![Node::value("a"), Node::value("2")])])])))]
    #[case("x=1*b;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("*", vec![Node::value("1"), Node::value("b")])])])))]
    #[case("x=a*b;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("*", vec![Node::value("a"), Node::value("b")])])])))]
    #[case("x=1*;", false, false, None)]
    #[case("x=1/2;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("/", vec![Node::value("1"), Node::value("2")])])])))]
    #[case("x=1/;", false, false, None)]
    #[case("x=1%2;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("%", vec![Node::value("1"), Node::value("2")])])])))]
    #[case("x=1%1;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("%", vec![Node::value("1"), Node::value("1")])])])))]
    #[case("x=1%0;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("%", vec![Node::value("1"), Node::value("0")])])])))]
    #[case("x=1%;", false, false, None)]
    #[case("x=1==2;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("==", vec![Node::value("1"), Node::value("2")])])])))]
    #[case("x=a==2;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("==", vec![Node::value("a"), Node::value("2")])])])))]
    #[case("x=1==b;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("==", vec![Node::value("1"), Node::value("b")])])])))]
    #[case("x=a==b;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("==", vec![Node::value("a"), Node::value("b")])])])))]
    #[case("x=1==;", false, false, None)]
    #[case("x=1!=2;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("!=", vec![Node::value("1"), Node::value("2")])])])))]
    #[case("x=1!=;", false, false, None)]
    #[case("x=1<2;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("<", vec![Node::value("1"), Node::value("2")])])])))]
    #[case("x=1<;", false, false, None)]
    #[case("x=1<=2;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("<=", vec![Node::value("1"), Node::value("2")])])])))]
    #[case("x=1<=;", false, false, None)]
    #[case("x=1>2;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr(">", vec![Node::value("1"), Node::value("2")])])])))]
    #[case("x=1>;", false, false, None)]
    #[case("x=1>=2;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr(">=", vec![Node::value("1"), Node::value("2")])])])))]
    #[case("x=+a;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("+", vec![Node::value("unary"), Node::value("a")])])])))]
    #[case("x=-a;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("-", vec![Node::value("unary"), Node::value("a")])])])))]
    #[case("x=1>=;", false, false, None)]
    #[case("x=(1);", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::value("1")])])))]
    #[case("x=(a);", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::value("a")])])))]
    #[case("x=a[1];", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("array_var", vec![Node::value("a"), Node::value("1")])])])))]
    #[case("x=(1;", false, false, None)]
    #[case("x=a[1;", false, false, None)]
    #[case("x=a 1];", false, false, None)]
    #[case("x=a[];", false, false, None)]
    #[case("x=(x);", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::value("x")])])))]
    #[case("(x)=x;", true, false, None)]
    #[case("x[1]=(x[1]);", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::expr("array_var", vec![Node::value("x"), Node::value("1")]), Node::expr("array_var", vec![Node::value("x"), Node::value("1")])])])))]
    #[case("(x[1])=x[1];", true, false, None)]
    #[case("x=f()();", false, false, None)]
    #[case("x=3();", false, false, None)]
    #[case("x=(x)();", false, false, None)]
    #[timeout(Duration::from_secs(1))]
    fn test_assn_stmt(
        #[case] input: &str,
        #[case] exp_good: bool,
        #[case] exp_done: bool,
        #[case] exp_ast: Option<Node>,
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
    #[case("return x;", true, true, Some(Node::program(vec![Node::stmt("return", vec![Node::value("x")])])))]
    #[case("return x", false, true, None)]
    #[case("return -34;", true, true, Some(Node::program(vec![Node::stmt("return", vec![Node::expr("-", vec![Node::value("unary"), Node::value("34")])])])))]
    #[case("return;", false, false, None)]
    #[case("return(x);", true, true, Some(Node::program(vec![Node::stmt("return", vec![Node::value("x")])])))]
    #[case("return(3+1<=4*(x-y));", true, true, Some(Node::program(vec![Node::stmt("return", vec![Node::expr("<=", vec![Node::expr("+", vec![Node::value("3"), Node::value("1")]), Node::expr("*", vec![Node::value("4"), Node::expr("-", vec![Node::value("x"), Node::value("y")])])])])])))]
    #[timeout(Duration::from_secs(1))]
    fn test_return_stmt(
        #[case] input: &str,
        #[case] exp_good: bool,
        #[case] exp_done: bool,
        #[case] exp_ast: Option<Node>,
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
    #[case("++abc;", true, true, Some(Node::program(vec![Node::stmt("inc", vec![Node::value("abc")])])))]
    #[case("--zz;", true, true, Some(Node::program(vec![Node::stmt("dec", vec![Node::value("zz")])])))]
    #[case("++x[1];", true, true, Some(Node::program(vec![Node::stmt("inc", vec![Node::expr("array_var", vec![Node::value("x"), Node::value("1")])])])))]
    #[case("--y[2];", true, true, Some(Node::program(vec![Node::stmt("dec", vec![Node::expr("array_var", vec![Node::value("y"), Node::value("2")])])])))]
    #[case("++aa[(n+1)*3];", true, true, Some(Node::program(vec![Node::stmt("inc", vec![Node::expr("array_var", vec![Node::value("aa"), Node::expr("*", vec![Node::expr("+", vec![Node::value("n"), Node::value("1")]), Node::value("3")])])])])))]
    #[case("--bb[4/(2-k)];", true, true, Some(Node::program(vec![Node::stmt("dec", vec![Node::expr("array_var", vec![Node::value("bb"), Node::expr("/", vec![Node::value("4"), Node::expr("-", vec![Node::value("2"), Node::value("k")])])])])])))]
    #[case("++abc", false, true, None)]
    #[case("--zz", false, true, None)]
    #[case("++;", false, false, None)]
    #[case("--;", false, false, None)]
    #[case("++a+b;", false, false, None)]
    #[case("--c-d;", false, false, None)]
    #[case("+++b;", false, false, None)]
    #[case("---d;", false, false, None)]
    #[case("++(b);", false, false, None)]
    #[case("--(d);", false, false, None)]
    #[case("++!bl", false, false, None)]
    #[case("--!d;", false, false, None)]
    #[case("print(1+(++abc));", false, false, None)]
    #[case("print((++abc)-1);", false, false, None)]
    #[case("qq++;", false, false, None)]
    #[case("ww--;", false, false, None)]
    #[timeout(Duration::from_secs(1))]
    fn test_inc_dec(
        #[case] input: &str,
        #[case] exp_good: bool,
        #[case] exp_done: bool,
        #[case] exp_ast: Option<Node>,
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
    #[case("print(x);", true, true, Some(Node::program(vec![Node::stmt("print", vec![Node::value("x")])])))]
    #[case("print(chr(65));", true, true, Some(Node::program(vec![Node::stmt("print", vec![Node::stmt("chr_call", vec![Node::value("65")])])])))]
    #[case("print(chr(1),chr(2),chr(3));", true, true, Some(Node::program(vec![Node::stmt("print", vec![Node::stmt("chr_call", vec![Node::value("1")]), Node::stmt("chr_call", vec![Node::value("2")]), Node::stmt("chr_call", vec![Node::value("3")])])])))]
    #[case("print(\"a b\", chr(1+2), a*4);", true, true, Some(Node::program(vec![Node::stmt("print", vec![Node::stmt("strlit_out", vec![Node::value("\"a b\"")]), Node::stmt("chr_call", vec![Node::expr("+", vec![Node::value("1"), Node::value("2")])]), Node::expr("*", vec![Node::value("a"), Node::value("4")])])])))]
    #[case("print(chr(1-2), \"a b\", 4/a);", true, true, Some(Node::program(vec![Node::stmt("print", vec![Node::stmt("chr_call", vec![Node::expr("-", vec![Node::value("1"), Node::value("2")])]), Node::stmt("strlit_out", vec![Node::value("\"a b\"")]), Node::expr("/", vec![Node::value("4"), Node::value("a")])])])))]
    #[case("print(a+xyz_3[b*(c==d-f)]%g<=h);", true, true, Some(Node::program(vec![Node::stmt("print", vec![Node::expr("<=", vec![Node::expr("+", vec![Node::value("a"), Node::expr("%", vec![Node::expr("array_var", vec![Node::value("xyz_3"), Node::expr("*", vec![Node::value("b"), Node::expr("==", vec![Node::value("c"), Node::expr("-", vec![Node::value("d"), Node::value("f")])])])]), Node::value("g")])]), Node::value("h")])])])))]
    #[case("print(1)", false, true, None)]
    #[timeout(Duration::from_secs(1))]
    fn test_print_stmt_with_expr(
        #[case] input: &str,
        #[case] exp_good: bool,
        #[case] exp_done: bool,
        #[case] exp_ast: Option<Node>,
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
    #[case("func q(){print(abc+3);}", true, true, Some(Node::program(vec![Node::stmt("func_def", vec![Node::value("q"), Node::program(vec![Node::stmt("print", vec![Node::expr("+", vec![Node::value("abc"), Node::value("3")])])])])])))]
    #[case("func qq(){print(a+x[b*(c==d-f)]%g<=h);}", true, true, Some(Node::program(vec![Node::stmt("func_def", vec![Node::value("qq"), Node::program(vec![Node::stmt("print", vec![Node::expr("<=", vec![Node::expr("+", vec![Node::value("a"), Node::expr("%", vec![Node::expr("array_var", vec![Node::value("x"), Node::expr("*", vec![Node::value("b"), Node::expr("==", vec![Node::value("c"), Node::expr("-", vec![Node::value("d"), Node::value("f")])])])]), Node::value("g")])]), Node::value("h")])])])])])))]
    #[timeout(Duration::from_secs(1))]
    fn test_func_def_with_expr(
        #[case] input: &str,
        #[case] exp_good: bool,
        #[case] exp_done: bool,
        #[case] exp_ast: Option<Node>,
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
    #[case("x=1&&2&&3&&4&&5;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("&&", vec![Node::expr("&&", vec![Node::expr("&&", vec![Node::expr("&&", vec![Node::value("1"), Node::value("2")]), Node::value("3")]), Node::value("4")]), Node::value("5")])])])))]
    #[case("x=1 || 2 || 3 || 4 || 5;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("||", vec![Node::expr("||", vec![Node::expr("||", vec![Node::expr("||", vec![Node::value("1"), Node::value("2")]), Node::value("3")]), Node::value("4")]), Node::value("5")])])])))]
    #[case("x=1+2+3+4+5;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("+", vec![Node::expr("+", vec![Node::expr("+", vec![Node::expr("+", vec![Node::value("1"), Node::value("2")]), Node::value("3")]), Node::value("4")]), Node::value("5")])])])))]
    #[case("x=1-2-3-4-5;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("-", vec![Node::expr("-", vec![Node::expr("-", vec![Node::expr("-", vec![Node::value("1"), Node::value("2")]), Node::value("3")]), Node::value("4")]), Node::value("5")])])])))]
    #[case("x=1*2*3*4*5;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("*", vec![Node::expr("*", vec![Node::expr("*", vec![Node::expr("*", vec![Node::value("1"), Node::value("2")]), Node::value("3")]), Node::value("4")]), Node::value("5")])])])))]
    #[case("x=1/2/3/4/5;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("/", vec![Node::expr("/", vec![Node::expr("/", vec![Node::expr("/", vec![Node::value("1"), Node::value("2")]), Node::value("3")]), Node::value("4")]), Node::value("5")])])])))]
    #[case("x=1%2%3%4%5;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("%", vec![Node::expr("%", vec![Node::expr("%", vec![Node::expr("%", vec![Node::value("1"), Node::value("2")]), Node::value("3")]), Node::value("4")]), Node::value("5")])])])))]
    #[case("x=1==2==3==4==5;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("==", vec![Node::expr("==", vec![Node::expr("==", vec![Node::expr("==", vec![Node::value("1"), Node::value("2")]), Node::value("3")]), Node::value("4")]), Node::value("5")])])])))]
    #[case("x=1!=2!=3!=4!=5;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("!=", vec![Node::expr("!=", vec![Node::expr("!=", vec![Node::expr("!=", vec![Node::value("1"), Node::value("2")]), Node::value("3")]), Node::value("4")]), Node::value("5")])])])))]
    #[case("x=1<2<3<4<5;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("<", vec![Node::expr("<", vec![Node::expr("<", vec![Node::expr("<", vec![Node::value("1"), Node::value("2")]), Node::value("3")]), Node::value("4")]), Node::value("5")])])])))]
    #[case("x=1<=2<=3<=4<=5;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("<=", vec![Node::expr("<=", vec![Node::expr("<=", vec![Node::expr("<=", vec![Node::value("1"), Node::value("2")]), Node::value("3")]), Node::value("4")]), Node::value("5")])])])))]
    #[case("x=1>2>3>4>5;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr(">", vec![Node::expr(">", vec![Node::expr(">", vec![Node::expr(">", vec![Node::value("1"), Node::value("2")]), Node::value("3")]), Node::value("4")]), Node::value("5")])])])))]
    #[case("x=1>=2>=3>=4>=5;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr(">=", vec![Node::expr(">=", vec![Node::expr(">=", vec![Node::expr(">=", vec![Node::value("1"), Node::value("2")]), Node::value("3")]), Node::value("4")]), Node::value("5")])])])))]
    #[case("x=! ! ! ! a;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("!", vec![Node::value("unary"), Node::expr("!", vec![Node::value("unary"), Node::expr("!", vec![Node::value("unary"), Node::expr("!", vec![Node::value("unary"), Node::value("a")])])])])])])))]
    #[case("x=+ + + +a;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("+", vec![Node::value("unary"), Node::expr("+", vec![Node::value("unary"), Node::expr("+", vec![Node::value("unary"), Node::expr("+", vec![Node::value("unary"), Node::value("a")])])])])])])))]
    #[case("x=- - - -a;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("-", vec![Node::value("unary"), Node::expr("-", vec![Node::value("unary"), Node::expr("-", vec![Node::value("unary"), Node::expr("-", vec![Node::value("unary"), Node::value("a")])])])])])])))]
    #[case("x=a && b || c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("||", vec![Node::expr("&&", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])])))]
    #[case("x=a && b==c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("&&", vec![Node::value("a"), Node::expr("==", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a && b!=c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("&&", vec![Node::value("a"), Node::expr("!=", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a && b<c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("&&", vec![Node::value("a"), Node::expr("<", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a && b<=c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("&&", vec![Node::value("a"), Node::expr("<=", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a && b>c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("&&", vec![Node::value("a"), Node::expr(">", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a && b>=c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("&&", vec![Node::value("a"), Node::expr(">=", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a && b+c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("&&", vec![Node::value("a"), Node::expr("+", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a && b-c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("&&", vec![Node::value("a"), Node::expr("-", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a && b*c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("&&", vec![Node::value("a"), Node::expr("*", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a && b/c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("&&", vec![Node::value("a"), Node::expr("/", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a && b%c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("&&", vec![Node::value("a"), Node::expr("%", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a || b && c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("&&", vec![Node::expr("||", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])])))]
    #[case("x=a || b==c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("||", vec![Node::value("a"), Node::expr("==", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a || b!=c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("||", vec![Node::value("a"), Node::expr("!=", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a || b<c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("||", vec![Node::value("a"), Node::expr("<", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a || b<=c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("||", vec![Node::value("a"), Node::expr("<=", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a || b>c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("||", vec![Node::value("a"), Node::expr(">", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a || b>=c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("||", vec![Node::value("a"), Node::expr(">=", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a || b+c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("||", vec![Node::value("a"), Node::expr("+", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a || b-c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("||", vec![Node::value("a"), Node::expr("-", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a || b*c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("||", vec![Node::value("a"), Node::expr("*", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a || b/c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("||", vec![Node::value("a"), Node::expr("/", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a || b%c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("||", vec![Node::value("a"), Node::expr("%", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a==b>c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr(">", vec![Node::expr("==", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])])))]
    #[case("x=a==b+c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("==", vec![Node::value("a"), Node::expr("+", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a==b-c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("==", vec![Node::value("a"), Node::expr("-", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a==b*c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("==", vec![Node::value("a"), Node::expr("*", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a==b/c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("==", vec![Node::value("a"), Node::expr("/", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a==b%c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("==", vec![Node::value("a"), Node::expr("%", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a>b==c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("==", vec![Node::expr(">", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])])))]
    #[case("x=a>b+c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr(">", vec![Node::value("a"), Node::expr("+", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a>b-c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr(">", vec![Node::value("a"), Node::expr("-", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a>b*c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr(">", vec![Node::value("a"), Node::expr("*", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a>b/c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr(">", vec![Node::value("a"), Node::expr("/", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a>b%c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr(">", vec![Node::value("a"), Node::expr("%", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a+b==c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("==", vec![Node::expr("+", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])])))]
    #[case("x=a+b>c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr(">", vec![Node::expr("+", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])])))]
    #[case("x=a+b-c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("-", vec![Node::expr("+", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])])))]
    #[case("x=a+b*c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("+", vec![Node::value("a"), Node::expr("*", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a+b/c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("+", vec![Node::value("a"), Node::expr("/", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a+b%c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("+", vec![Node::value("a"), Node::expr("%", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a-b==c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("==", vec![Node::expr("-", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])])))]
    #[case("x=a-b>c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr(">", vec![Node::expr("-", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])])))]
    #[case("x=a-b+c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("+", vec![Node::expr("-", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])])))]
    #[case("x=a-b*c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("-", vec![Node::value("a"), Node::expr("*", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a-b/c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("-", vec![Node::value("a"), Node::expr("/", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a-b%c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("-", vec![Node::value("a"), Node::expr("%", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a*b==c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("==", vec![Node::expr("*", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])])))]
    #[case("x=a*b>c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr(">", vec![Node::expr("*", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])])))]
    #[case("x=a*b+c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("+", vec![Node::expr("*", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])])))]
    #[case("x=a*b-c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("-", vec![Node::expr("*", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])])))]
    #[case("x=a*b/c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("/", vec![Node::expr("*", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])])))]
    #[case("x=a*b%c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("%", vec![Node::expr("*", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])])))]
    #[case("x=a/b==c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("==", vec![Node::expr("/", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])])))]
    #[case("x=a/b>c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr(">", vec![Node::expr("/", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])])))]
    #[case("x=a/b+c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("+", vec![Node::expr("/", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])])))]
    #[case("x=a/b-c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("-", vec![Node::expr("/", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])])))]
    #[case("x=a/b*c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("*", vec![Node::expr("/", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])])))]
    #[case("x=a/b%c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("%", vec![Node::expr("/", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])])))]
    #[case("x=a%b==c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("==", vec![Node::expr("%", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])])))]
    #[case("x=a%b>c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr(">", vec![Node::expr("%", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])])))]
    #[case("x=a%b+c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("+", vec![Node::expr("%", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])])))]
    #[case("x=a%b-c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("-", vec![Node::expr("%", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])])))]
    #[case("x=a%b*c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("*", vec![Node::expr("%", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])])))]
    #[case("x=a%b/c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("/", vec![Node::expr("%", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])])))]
    #[case("x=! a && b;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("&&", vec![Node::expr("!", vec![Node::value("unary"), Node::value("a")]), Node::value("b")])])])))]
    #[case("x=! a || b;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("||", vec![Node::expr("!", vec![Node::value("unary"), Node::value("a")]), Node::value("b")])])])))]
    #[case("x=! a==b;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("==", vec![Node::expr("!", vec![Node::value("unary"), Node::value("a")]), Node::value("b")])])])))]
    #[case("x=! a!=b;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("!=", vec![Node::expr("!", vec![Node::value("unary"), Node::value("a")]), Node::value("b")])])])))]
    #[case("x=! a<b;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("<", vec![Node::expr("!", vec![Node::value("unary"), Node::value("a")]), Node::value("b")])])])))]
    #[case("x=! a<=b;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("<=", vec![Node::expr("!", vec![Node::value("unary"), Node::value("a")]), Node::value("b")])])])))]
    #[case("x=! a>b;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr(">", vec![Node::expr("!", vec![Node::value("unary"), Node::value("a")]), Node::value("b")])])])))]
    #[case("x=! a>=b;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr(">=", vec![Node::expr("!", vec![Node::value("unary"), Node::value("a")]), Node::value("b")])])])))]
    #[case("x=! a+b;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("+", vec![Node::expr("!", vec![Node::value("unary"), Node::value("a")]), Node::value("b")])])])))]
    #[case("x=! a-b;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("-", vec![Node::expr("!", vec![Node::value("unary"), Node::value("a")]), Node::value("b")])])])))]
    #[case("x=! a*b;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("*", vec![Node::expr("!", vec![Node::value("unary"), Node::value("a")]), Node::value("b")])])])))]
    #[case("x=! a/b;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("/", vec![Node::expr("!", vec![Node::value("unary"), Node::value("a")]), Node::value("b")])])])))]
    #[case("x=! a%b;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("%", vec![Node::expr("!", vec![Node::value("unary"), Node::value("a")]), Node::value("b")])])])))]
    #[case("x=a!=+b;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("!=", vec![Node::value("a"), Node::expr("+", vec![Node::value("unary"), Node::value("b")])])])])))]
    #[case("x=-a<c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("<", vec![Node::expr("-", vec![Node::value("unary"), Node::value("a")]), Node::value("c")])])])))]
    #[case("x=a+ +b;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("+", vec![Node::value("a"), Node::expr("+", vec![Node::value("unary"), Node::value("b")])])])])))]
    #[case("x=a+-b;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("+", vec![Node::value("a"), Node::expr("-", vec![Node::value("unary"), Node::value("b")])])])])))]
    #[case("x=+a+b;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("+", vec![Node::expr("+", vec![Node::value("unary"), Node::value("a")]), Node::value("b")])])])))]
    #[case("x=-a+b;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("+", vec![Node::expr("-", vec![Node::value("unary"), Node::value("a")]), Node::value("b")])])])))]
    #[case("x=a-+b;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("-", vec![Node::value("a"), Node::expr("+", vec![Node::value("unary"), Node::value("b")])])])])))]
    #[case("x=a- -b;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("-", vec![Node::value("a"), Node::expr("-", vec![Node::value("unary"), Node::value("b")])])])])))]
    #[case("x=+a-b;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("-", vec![Node::expr("+", vec![Node::value("unary"), Node::value("a")]), Node::value("b")])])])))]
    #[case("x=-a-b;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("-", vec![Node::expr("-", vec![Node::value("unary"), Node::value("a")]), Node::value("b")])])])))]
    #[case("x=a*-b;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("*", vec![Node::value("a"), Node::expr("-", vec![Node::value("unary"), Node::value("b")])])])])))]
    #[case("x=+a*c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("*", vec![Node::expr("+", vec![Node::value("unary"), Node::value("a")]), Node::value("c")])])])))]
    #[case("x=a/+b;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("/", vec![Node::value("a"), Node::expr("+", vec![Node::value("unary"), Node::value("b")])])])])))]
    #[case("x=-a/c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("/", vec![Node::expr("-", vec![Node::value("unary"), Node::value("a")]), Node::value("c")])])])))]
    #[case("x=a%-b;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("%", vec![Node::value("a"), Node::expr("-", vec![Node::value("unary"), Node::value("b")])])])])))]
    #[case("x=+a%c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("%", vec![Node::expr("+", vec![Node::value("unary"), Node::value("a")]), Node::value("c")])])])))]
    #[case("x=1 && (2 && 3 && 4) && 5;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("&&", vec![Node::expr("&&", vec![Node::value("1"), Node::expr("&&", vec![Node::expr("&&", vec![Node::value("2"), Node::value("3")]), Node::value("4")])]), Node::value("5")])])])))]
    #[case("x=1 || (2 || 3 || 4) || 5;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("||", vec![Node::expr("||", vec![Node::value("1"), Node::expr("||", vec![Node::expr("||", vec![Node::value("2"), Node::value("3")]), Node::value("4")])]), Node::value("5")])])])))]
    #[case("x=1==(2==3==4)==5;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("==", vec![Node::expr("==", vec![Node::value("1"), Node::expr("==", vec![Node::expr("==", vec![Node::value("2"), Node::value("3")]), Node::value("4")])]), Node::value("5")])])])))]
    #[case("x=1!=(2!=3!=4)!=5;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("!=", vec![Node::expr("!=", vec![Node::value("1"), Node::expr("!=", vec![Node::expr("!=", vec![Node::value("2"), Node::value("3")]), Node::value("4")])]), Node::value("5")])])])))]
    #[case("x=1<(2<3<4)<5;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("<", vec![Node::expr("<", vec![Node::value("1"), Node::expr("<", vec![Node::expr("<", vec![Node::value("2"), Node::value("3")]), Node::value("4")])]), Node::value("5")])])])))]
    #[case("x=1<=(2<=3<=4)<=5;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("<=", vec![Node::expr("<=", vec![Node::value("1"), Node::expr("<=", vec![Node::expr("<=", vec![Node::value("2"), Node::value("3")]), Node::value("4")])]), Node::value("5")])])])))]
    #[case("x=1>(2>3>4)>5;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr(">", vec![Node::expr(">", vec![Node::value("1"), Node::expr(">", vec![Node::expr(">", vec![Node::value("2"), Node::value("3")]), Node::value("4")])]), Node::value("5")])])])))]
    #[case("x=1>=(2>=3>=4)>=5;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr(">=", vec![Node::expr(">=", vec![Node::value("1"), Node::expr(">=", vec![Node::expr(">=", vec![Node::value("2"), Node::value("3")]), Node::value("4")])]), Node::value("5")])])])))]
    #[case("x=1+(2+3+4)+5;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("+", vec![Node::expr("+", vec![Node::value("1"), Node::expr("+", vec![Node::expr("+", vec![Node::value("2"), Node::value("3")]), Node::value("4")])]), Node::value("5")])])])))]
    #[case("x=1-(2-3-4)-5;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("-", vec![Node::expr("-", vec![Node::value("1"), Node::expr("-", vec![Node::expr("-", vec![Node::value("2"), Node::value("3")]), Node::value("4")])]), Node::value("5")])])])))]
    #[case("x=1*(2*3*4)*5;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("*", vec![Node::expr("*", vec![Node::value("1"), Node::expr("*", vec![Node::expr("*", vec![Node::value("2"), Node::value("3")]), Node::value("4")])]), Node::value("5")])])])))]
    #[case("x=1/(2/3/4)/5;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("/", vec![Node::expr("/", vec![Node::value("1"), Node::expr("/", vec![Node::expr("/", vec![Node::value("2"), Node::value("3")]), Node::value("4")])]), Node::value("5")])])])))]
    #[case("x=1%(2%3%4)%5;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("%", vec![Node::expr("%", vec![Node::value("1"), Node::expr("%", vec![Node::expr("%", vec![Node::value("2"), Node::value("3")]), Node::value("4")])]), Node::value("5")])])])))]
    #[case("x=(a==b)+c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("+", vec![Node::expr("==", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])])))]
    #[case("x=(a!=b)-c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("-", vec![Node::expr("!=", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])])))]
    #[case("x=(a<b)*c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("*", vec![Node::expr("<", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])])))]
    #[case("x=(a<=b)/c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("/", vec![Node::expr("<=", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])])))]
    #[case("x=(a>b)%c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("%", vec![Node::expr(">", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])])))]
    #[case("x=a+(b>=c);", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("+", vec![Node::value("a"), Node::expr(">=", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=(a-b)*c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("*", vec![Node::expr("-", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])])))]
    #[case("x=(a+b)%c;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("%", vec![Node::expr("+", vec![Node::value("a"), Node::value("b")]), Node::value("c")])])])))]
    #[case("x=a*(b==c);", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("*", vec![Node::value("a"), Node::expr("==", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a/(b!=c);", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("/", vec![Node::value("a"), Node::expr("!=", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=a%(b<c);", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("%", vec![Node::value("a"), Node::expr("<", vec![Node::value("b"), Node::value("c")])])])])))]
    #[case("x=+(a<=b);", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("+", vec![Node::value("unary"), Node::expr("<=", vec![Node::value("a"), Node::value("b")])])])])))]
    #[case("x=-(a>b);", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("-", vec![Node::value("unary"), Node::expr(">", vec![Node::value("a"), Node::value("b")])])])])))]
    #[case("x=+(a+b);", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("+", vec![Node::value("unary"), Node::expr("+", vec![Node::value("a"), Node::value("b")])])])])))]
    #[case("x=-(a-b);", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("-", vec![Node::value("unary"), Node::expr("-", vec![Node::value("a"), Node::value("b")])])])])))]
    #[case("x=+(a*b);", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("+", vec![Node::value("unary"), Node::expr("*", vec![Node::value("a"), Node::value("b")])])])])))]
    #[case("x=-(a/b);", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("-", vec![Node::value("unary"), Node::expr("/", vec![Node::value("a"), Node::value("b")])])])])))]
    #[case("x=+(a%b);", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("+", vec![Node::value("unary"), Node::expr("%", vec![Node::value("a"), Node::value("b")])])])])))]
    #[timeout(Duration::from_secs(1))]
    fn test_expr_prec_assoc(
        #[case] input: &str,
        #[case] exp_good: bool,
        #[case] exp_done: bool,
        #[case] exp_ast: Option<Node>,
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
    #[case("x=readint();", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::stmt("readint", vec![])])])))]
    #[case("x=readint(y);", false, false, None)]
    #[case("x=readint;", false, false, None)]
    #[case("x=readint);", false, false, None)]
    #[case("x=readint(;", false, false, None)]
    #[case("readint();", true, false, None)]
    #[case("x=readint();y=readint();", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::stmt("readint", vec![])]), Node::stmt("assn", vec![Node::value("y"), Node::stmt("readint", vec![])])])))]
    #[case("x=rnd(1);", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::stmt("rnd", vec![Node::value("1")])])])))]
    #[case("x=rnd();", false, false, None)]
    #[case("x=rnd;", false, false, None)]
    #[case("x=rnd 1);", false, false, None)]
    #[case("x=rnd(1;", false, false, None)]
    #[case("rnd(1);", true, false, None)]
    #[case("x=rnd(a);y=rnd(0);", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::stmt("rnd", vec![Node::value("a")])]), Node::stmt("assn", vec![Node::value("y"), Node::stmt("rnd", vec![Node::value("0")])])])))]
    #[timeout(Duration::from_secs(1))]
    fn test_readint_rnd(
        #[case] input: &str,
        #[case] exp_good: bool,
        #[case] exp_done: bool,
        #[case] exp_ast: Option<Node>,
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
    #[case("a[1] = 2;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::expr("array_var", vec![Node::value("a"), Node::value("1")]), Node::value("2")])])))]
    #[case("a = b[2];", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("a"), Node::expr("array_var", vec![Node::value("b"), Node::value("2")])])])))]
    #[case("abc[5*2+a]=bcd[5<=1/4]/cde[! 0>x];", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::expr("array_var", vec![Node::value("abc"), Node::expr("+", vec![Node::expr("*", vec![Node::value("5"), Node::value("2")]), Node::value("a")])]), Node::expr("/", vec![Node::expr("array_var", vec![Node::value("bcd"), Node::expr("<=", vec![Node::value("5"), Node::expr("/", vec![Node::value("1"), Node::value("4")])])]), Node::expr("array_var", vec![Node::value("cde"), Node::expr(">", vec![Node::expr("!", vec![Node::value("unary"), Node::value("0")]), Node::value("x")])])])])])))]
    #[timeout(Duration::from_secs(1))]
    fn test_array_item(
        #[case] input: &str,
        #[case] exp_good: bool,
        #[case] exp_done: bool,
        #[case] exp_ast: Option<Node>,
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
    #[case("x=a==b+c[x-y[2]]*+d!=e-f/-g<h+i%+j;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("<", vec![Node::expr("!=", vec![Node::expr("==", vec![Node::value("a"), Node::expr("+", vec![Node::value("b"), Node::expr("*", vec![Node::expr("array_var", vec![Node::value("c"), Node::expr("-", vec![Node::value("x"), Node::expr("array_var", vec![Node::value("y"), Node::value("2")])])]), Node::expr("+", vec![Node::value("unary"), Node::value("d")])])])]), Node::expr("-", vec![Node::value("e"), Node::expr("/", vec![Node::value("f"), Node::expr("-", vec![Node::value("unary"), Node::value("g")])])])]), Node::expr("+", vec![Node::value("h"), Node::expr("%", vec![Node::value("i"), Node::expr("+", vec![Node::value("unary"), Node::value("j")])])])])])])))]
    #[case("x=a==b+(c*+(d!=e[2*z]-f/-g)<h+i)%+j;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("==", vec![Node::value("a"), Node::expr("+", vec![Node::value("b"), Node::expr("%", vec![Node::expr("<", vec![Node::expr("*", vec![Node::value("c"), Node::expr("+", vec![Node::value("unary"), Node::expr("!=", vec![Node::value("d"), Node::expr("-", vec![Node::expr("array_var", vec![Node::value("e"), Node::expr("*", vec![Node::value("2"), Node::value("z")])]), Node::expr("/", vec![Node::value("f"), Node::expr("-", vec![Node::value("unary"), Node::value("g")])])])])])]), Node::expr("+", vec![Node::value("h"), Node::value("i")])]), Node::expr("+", vec![Node::value("unary"), Node::value("j")])])])])])])))]
    #[case("x=a[x[y[z]]%4]+ +b*c<=d- -e/f>g+-h%i>=j;", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr(">=", vec![Node::expr(">", vec![Node::expr("<=", vec![Node::expr("+", vec![Node::expr("array_var", vec![Node::value("a"), Node::expr("%", vec![Node::expr("array_var", vec![Node::value("x"), Node::expr("array_var", vec![Node::value("y"), Node::value("z")])]), Node::value("4")])]), Node::expr("*", vec![Node::expr("+", vec![Node::value("unary"), Node::value("b")]), Node::value("c")])]), Node::expr("-", vec![Node::value("d"), Node::expr("/", vec![Node::expr("-", vec![Node::value("unary"), Node::value("e")]), Node::value("f")])])]), Node::expr("+", vec![Node::value("g"), Node::expr("%", vec![Node::expr("-", vec![Node::value("unary"), Node::value("h")]), Node::value("i")])])]), Node::value("j")])])])))]
    #[case("x=a+ +(b*c<=d)- -e/(f>g+-h%i)>=j[-z];", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr(">=", vec![Node::expr("-", vec![Node::expr("+", vec![Node::value("a"), Node::expr("+", vec![Node::value("unary"), Node::expr("<=", vec![Node::expr("*", vec![Node::value("b"), Node::value("c")]), Node::value("d")])])]), Node::expr("/", vec![Node::expr("-", vec![Node::value("unary"), Node::value("e")]), Node::expr(">", vec![Node::value("f"), Node::expr("+", vec![Node::value("g"), Node::expr("%", vec![Node::expr("-", vec![Node::value("unary"), Node::value("h")]), Node::value("i")])])])])]), Node::expr("array_var", vec![Node::value("j"), Node::expr("-", vec![Node::value("unary"), Node::value("z")])])])])])))]
    #[case("print(rnd(readint()==15e3),rnd(rnd(rnd(readint()))));", true, true, Some(Node::program(vec![Node::stmt("print", vec![Node::stmt("rnd", vec![Node::expr("==", vec![Node::stmt("readint", vec![]), Node::value("15e3")])]), Node::stmt("rnd", vec![Node::stmt("rnd", vec![Node::stmt("rnd", vec![Node::stmt("readint", vec![])])])])])])))]
    #[case("x=a==b+c*+d!=e-/-g<h+i%+j;", false, false, None)]
    #[case("x=a==b+(c*+(d!=e-f/-g)<h+i)%+;", false, false, None)]
    #[case("x=a++b*c<=d- -e x/f>g+-h%i>=j;", false, false, None)]
    #[case("x=a++b*c<=d)- -e/(f>g+-h%i)>=j;", false, false, None)]
    #[case("x=((a[(b[c[(d[((e[f]))])]])]));", true, true, Some(Node::program(vec![Node::stmt("assn", vec![Node::value("x"), Node::expr("array_var", vec![Node::value("a"), Node::expr("array_var", vec![Node::value("b"), Node::expr("array_var", vec![Node::value("c"), Node::expr("array_var", vec![Node::value("d"), Node::expr("array_var", vec![Node::value("e"), Node::value("f")])])])])])])])))]
    #[case("x=((a[(b[c[(d[((e[f]))]])])]));", false, false, None)]
    #[case("while((a+b)%d+a()!=1){print();}", true, true, Some(Node::program(vec![Node::stmt("while", vec![Node::expr("!=", vec![Node::expr("+", vec![Node::expr("%", vec![Node::expr("+", vec![Node::value("a"), Node::value("b")]), Node::value("d")]), Node::stmt("func_call", vec![Node::value("a")])]), Node::value("1")]), Node::program(vec![Node::stmt("print", vec![])])])])))]
    #[timeout(Duration::from_secs(1))]
    fn test_expr_complex(
        #[case] input: &str,
        #[case] exp_good: bool,
        #[case] exp_done: bool,
        #[case] exp_ast: Option<Node>,
    ) {
        let tokens = Lexer::new(input.to_string()).lex_input();
        let (good, done, ast) = Parser::new(tokens).parse();
        assert_eq!(good, exp_good, "good flag mismatch\ninput: {input:?}");
        assert_eq!(done, exp_done, "done flag mismatch\ninput: {input:?}");
        if exp_good && exp_done {
            assert_eq!(ast, exp_ast, "ast mismatch\ninput: {input:?}");
        }
    }
