//! Parser tests — auto-generated from parseit_test.lua
//! Do not edit by hand.

use rstest::rstest;
use std::time::Duration;
use tamandua_rs::parser::*;
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
    #[case("print();", true, true, Node::program(vec![Node::stmt(StmtToken::Print, vec![])]))]
    #[case("print();print();print();", true, true, Node::program(vec![Node::stmt(StmtToken::Print, vec![]), Node::stmt(StmtToken::Print, vec![]), Node::stmt(StmtToken::Print, vec![])]))]
    #[case("print(\"abc\");", true, true, Node::program(vec![Node::stmt(StmtToken::Print, vec![Node::stmt(StmtToken::StrlitOut, vec![Node::strlit("\"abc\"")])])]))]
    #[case("print(\"a\",\"b\",\"c\",\"d\",\"e\");", true, true, Node::program(vec![Node::stmt(StmtToken::Print, vec![Node::stmt(StmtToken::StrlitOut, vec![Node::strlit("\"a\"")]), Node::stmt(StmtToken::StrlitOut, vec![Node::strlit("\"b\"")]), Node::stmt(StmtToken::StrlitOut, vec![Node::strlit("\"c\"")]), Node::stmt(StmtToken::StrlitOut, vec![Node::strlit("\"d\"")]), Node::stmt(StmtToken::StrlitOut, vec![Node::strlit("\"e\"")])])]))]
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
    #[case("println();", true, true, Node::program(vec![Node::stmt(StmtToken::Println, vec![])]))]
    #[case("println();println();println();", true, true, Node::program(vec![Node::stmt(StmtToken::Println, vec![]), Node::stmt(StmtToken::Println, vec![]), Node::stmt(StmtToken::Println, vec![])]))]
    #[case("println(\"abc\");", true, true, Node::program(vec![Node::stmt(StmtToken::Println, vec![Node::stmt(StmtToken::StrlitOut, vec![Node::strlit("\"abc\"")])])]))]
    #[case("println(\"a\",\"b\",\"c\",\"d\",\"e\");", true, true, Node::program(vec![Node::stmt(StmtToken::Println, vec![Node::stmt(StmtToken::StrlitOut, vec![Node::strlit("\"a\"")]), Node::stmt(StmtToken::StrlitOut, vec![Node::strlit("\"b\"")]), Node::stmt(StmtToken::StrlitOut, vec![Node::strlit("\"c\"")]), Node::stmt(StmtToken::StrlitOut, vec![Node::strlit("\"d\"")]), Node::stmt(StmtToken::StrlitOut, vec![Node::strlit("\"e\"")])])]))]
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
    #[case("print();println();", true, true, Node::program(vec![Node::stmt(StmtToken::Print, vec![]), Node::stmt(StmtToken::Println, vec![])]))]
    #[case("println();print();", true, true, Node::program(vec![Node::stmt(StmtToken::Println, vec![]), Node::stmt(StmtToken::Print, vec![])]))]
    #[case("print('a');println('b');", true, true, Node::program(vec![Node::stmt(StmtToken::Print, vec![Node::stmt(StmtToken::StrlitOut, vec![Node::strlit("'a'")])]), Node::stmt(StmtToken::Println, vec![Node::stmt(StmtToken::StrlitOut, vec![Node::strlit("'b'")])])]))]
    #[case("println('x');print('y');", true, true, Node::program(vec![Node::stmt(StmtToken::Println, vec![Node::stmt(StmtToken::StrlitOut, vec![Node::strlit("'x'")])]), Node::stmt(StmtToken::Print, vec![Node::stmt(StmtToken::StrlitOut, vec![Node::strlit("'y'")])])]))]
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
    #[case("func s(){}", true, true, Node::program(vec![Node::stmt(StmtToken::FuncDef, vec![Node::strlit("s"), Node::program(vec![])])]))]
    #[case("func(){}", false, false, Node::none())]
    #[case("func{}", false, false, Node::none())]
    #[case("func &s(){}", false, false, Node::none())]
    #[case("func s{}", false, false, Node::none())]
    #[case("func s()", false, true, Node::none())]
    #[case("func s()end", false, false, Node::none())]
    #[case("func s()){}", false, false, Node::none())]
    #[case("func (s)(){}", false, false, Node::none())]
    #[case("func s(){print(\"abc\");}", true, true, Node::program(vec![Node::stmt(StmtToken::FuncDef, vec![Node::strlit("s"), Node::program(vec![Node::stmt(StmtToken::Print, vec![Node::stmt(StmtToken::StrlitOut, vec![Node::strlit("\"abc\"")])])])])]))]
    #[case("func s(){print(\"x\");}", true, true, Node::program(vec![Node::stmt(StmtToken::FuncDef, vec![Node::strlit("s"), Node::program(vec![Node::stmt(StmtToken::Print, vec![Node::stmt(StmtToken::StrlitOut, vec![Node::strlit("\"x\"")])])])])]))]
    #[case("func s(){print();print();}", true, true, Node::program(vec![Node::stmt(StmtToken::FuncDef, vec![Node::strlit("s"), Node::program(vec![Node::stmt(StmtToken::Print, vec![]), Node::stmt(StmtToken::Print, vec![])])])]))]
    #[case("func sss(){print();print();print();}", true, true, Node::program(vec![Node::stmt(StmtToken::FuncDef, vec![Node::strlit("sss"), Node::program(vec![Node::stmt(StmtToken::Print, vec![]), Node::stmt(StmtToken::Print, vec![]), Node::stmt(StmtToken::Print, vec![])])])]))]
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
    #[case("ff();", true, true, Node::program(vec![Node::stmt(StmtToken::FuncCall, vec![Node::strlit("ff")])]))]
    #[case("fffffffffffffffffffffffffffffffff();", true, true, Node::program(vec![Node::stmt(StmtToken::FuncCall, vec![Node::strlit("fffffffffffffffffffffffffffffffff")])]))]
    #[case("ff();gg();", true, true, Node::program(vec![Node::stmt(StmtToken::FuncCall, vec![Node::strlit("ff")]), Node::stmt(StmtToken::FuncCall, vec![Node::strlit("gg")])]))]
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
    #[case("while(1){}", true, true, Node::program(vec![Node::stmt(StmtToken::While, vec![Node::value(ValueToken::numlit("1")), Node::program(vec![])])]))]
    #[case("while(1){print();}", true, true, Node::program(vec![Node::stmt(StmtToken::While, vec![Node::value(ValueToken::numlit("1")), Node::program(vec![Node::stmt(StmtToken::Print, vec![])])])]))]
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
    #[case("if(1){}", true, true, Node::program(vec![Node::stmt(StmtToken::If, vec![Node::value(ValueToken::numlit("1")), Node::program(vec![])])]))]
    #[case("if(1){print();}", true, true, Node::program(vec![Node::stmt(StmtToken::If, vec![Node::value(ValueToken::numlit("1")), Node::program(vec![Node::stmt(StmtToken::Print, vec![])])])]))]
    #[case("if(1){}else{}", true, true, Node::program(vec![Node::stmt(StmtToken::If, vec![Node::value(ValueToken::numlit("1")), Node::program(vec![]), Node::program(vec![])])]))]
    #[case("if(1){}elsif(1){}", true, true, Node::program(vec![Node::stmt(StmtToken::If, vec![Node::value(ValueToken::numlit("1")), Node::program(vec![]), Node::value(ValueToken::numlit("1")), Node::program(vec![])])]))]
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
    #[case("abc=123;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("abc")), Node::value(ValueToken::numlit("123"))])]))]
    #[case("abc=xyz;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("abc")), Node::value(ValueToken::strlit("xyz"))])]))]
    #[case("abc[1]=xyz;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::expr(ExprToken::ArrayVar, vec![Node::strlit("abc"), Node::value(ValueToken::numlit("1"))]), Node::value(ValueToken::strlit("xyz"))])]))]
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
    #[case("x=foo();", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::stmt(StmtToken::FuncCall, vec![Node::strlit("foo")])])]))]
    #[case("x=();", false, false, Node::none())]
    #[case("x=1&&2;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("&&"), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::numlit("2"))])])]))]
    #[case("x=1 || 2;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("||"), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::numlit("2"))])])]))]
    #[case("x=1 + 2;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::numlit("2"))])])]))]
    #[case("x=1+2;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::numlit("2"))])])]))]
    #[case("x=a+2;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::numlit("2"))])])]))]
    #[case("x=1+b;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::strlit("b"))])])]))]
    #[case("x=a+b;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))])])]))]
    #[case("x=1+;", false, false, Node::none())]
    #[case("x=1 - 2;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("-"), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::numlit("2"))])])]))]
    #[case("x=1-2;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("-"), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::numlit("2"))])])]))]
    #[case("x=1-;", false, false, Node::none())]
    #[case("x=1*2;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("*"), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::numlit("2"))])])]))]
    #[case("x=a*2;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("*"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::numlit("2"))])])]))]
    #[case("x=1*b;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("*"), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::strlit("b"))])])]))]
    #[case("x=a*b;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("*"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))])])]))]
    #[case("x=1*;", false, false, Node::none())]
    #[case("x=1/2;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("/"), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::numlit("2"))])])]))]
    #[case("x=1/;", false, false, Node::none())]
    #[case("x=1%2;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("%"), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::numlit("2"))])])]))]
    #[case("x=1%1;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("%"), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::numlit("1"))])])]))]
    #[case("x=1%0;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("%"), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::numlit("0"))])])]))]
    #[case("x=1%;", false, false, Node::none())]
    #[case("x=1==2;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("=="), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::numlit("2"))])])]))]
    #[case("x=a==2;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("=="), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::numlit("2"))])])]))]
    #[case("x=1==b;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("=="), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::strlit("b"))])])]))]
    #[case("x=a==b;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("=="), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))])])]))]
    #[case("x=1==;", false, false, Node::none())]
    #[case("x=1!=2;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("!="), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::numlit("2"))])])]))]
    #[case("x=1!=;", false, false, Node::none())]
    #[case("x=1<2;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("<"), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::numlit("2"))])])]))]
    #[case("x=1<;", false, false, Node::none())]
    #[case("x=1<=2;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("<="), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::numlit("2"))])])]))]
    #[case("x=1<=;", false, false, Node::none())]
    #[case("x=1>2;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op(">"), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::numlit("2"))])])]))]
    #[case("x=1>;", false, false, Node::none())]
    #[case("x=1>=2;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op(">="), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::numlit("2"))])])]))]
    #[case("x=+a;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::un_op("+"), vec![Node::value(ValueToken::strlit("a"))])])]))]
    #[case("x=-a;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::un_op("-"), vec![Node::value(ValueToken::strlit("a"))])])]))]
    #[case("x=1>=;", false, false, Node::none())]
    #[case("x=(1);", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::value(ValueToken::numlit("1"))])]))]
    #[case("x=(a);", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::value(ValueToken::strlit("a"))])]))]
    #[case("x=a[1];", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::ArrayVar, vec![Node::strlit("a"), Node::value(ValueToken::numlit("1"))])])]))]
    #[case("x=(1;", false, false, Node::none())]
    #[case("x=a[1;", false, false, Node::none())]
    #[case("x=a 1];", false, false, Node::none())]
    #[case("x=a[];", false, false, Node::none())]
    #[case("x=(x);", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::value(ValueToken::strlit("x"))])]))]
    #[case("(x)=x;", true, false, Node::none())]
    #[case("x[1]=(x[1]);", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::expr(ExprToken::ArrayVar, vec![Node::strlit("x"), Node::value(ValueToken::numlit("1"))]), Node::expr(ExprToken::ArrayVar, vec![Node::strlit("x"), Node::value(ValueToken::numlit("1"))])])]))]
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
    #[case("return x;", true, true, Node::program(vec![Node::stmt(StmtToken::Return, vec![Node::value(ValueToken::strlit("x"))])]))]
    #[case("return x", false, true, Node::none())]
    #[case("return -34;", true, true, Node::program(vec![Node::stmt(StmtToken::Return, vec![Node::expr(ExprToken::un_op("-"), vec![Node::value(ValueToken::numlit("34"))])])]))]
    #[case("return;", false, false, Node::none())]
    #[case("return(x);", true, true, Node::program(vec![Node::stmt(StmtToken::Return, vec![Node::value(ValueToken::strlit("x"))])]))]
    #[case("return(3+1<=4*(x-y));", true, true, Node::program(vec![Node::stmt(StmtToken::Return, vec![Node::expr(ExprToken::bin_op("<="), vec![Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::numlit("3")), Node::value(ValueToken::numlit("1"))]), Node::expr(ExprToken::bin_op("*"), vec![Node::value(ValueToken::numlit("4")), Node::expr(ExprToken::bin_op("-"), vec![Node::value(ValueToken::strlit("x")), Node::value(ValueToken::strlit("y"))])])])])]))]
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
    #[case("++abc;", true, true, Node::program(vec![Node::stmt(StmtToken::Inc, vec![Node::value(ValueToken::strlit("abc"))])]))]
    #[case("--zz;", true, true, Node::program(vec![Node::stmt(StmtToken::Dec, vec![Node::value(ValueToken::strlit("zz"))])]))]
    #[case("++x[1];", true, true, Node::program(vec![Node::stmt(StmtToken::Inc, vec![Node::expr(ExprToken::ArrayVar, vec![Node::strlit("x"), Node::value(ValueToken::numlit("1"))])])]))]
    #[case("--y[2];", true, true, Node::program(vec![Node::stmt(StmtToken::Dec, vec![Node::expr(ExprToken::ArrayVar, vec![Node::strlit("y"), Node::value(ValueToken::numlit("2"))])])]))]
    #[case("++aa[(n+1)*3];", true, true, Node::program(vec![Node::stmt(StmtToken::Inc, vec![Node::expr(ExprToken::ArrayVar, vec![Node::strlit("aa"), Node::expr(ExprToken::bin_op("*"), vec![Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::strlit("n")), Node::value(ValueToken::numlit("1"))]), Node::value(ValueToken::numlit("3"))])])])]))]
    #[case("--bb[4/(2-k)];", true, true, Node::program(vec![Node::stmt(StmtToken::Dec, vec![Node::expr(ExprToken::ArrayVar, vec![Node::strlit("bb"), Node::expr(ExprToken::bin_op("/"), vec![Node::value(ValueToken::numlit("4")), Node::expr(ExprToken::bin_op("-"), vec![Node::value(ValueToken::numlit("2")), Node::value(ValueToken::strlit("k"))])])])])]))]
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
    #[case("print(x);", true, true, Node::program(vec![Node::stmt(StmtToken::Print, vec![Node::value(ValueToken::strlit("x"))])]))]
    #[case("print(chr(65));", true, true, Node::program(vec![Node::stmt(StmtToken::Print, vec![Node::stmt(StmtToken::ChrCall, vec![Node::value(ValueToken::numlit("65"))])])]))]
    #[case("print(chr(1),chr(2),chr(3));", true, true, Node::program(vec![Node::stmt(StmtToken::Print, vec![Node::stmt(StmtToken::ChrCall, vec![Node::value(ValueToken::numlit("1"))]), Node::stmt(StmtToken::ChrCall, vec![Node::value(ValueToken::numlit("2"))]), Node::stmt(StmtToken::ChrCall, vec![Node::value(ValueToken::numlit("3"))])])]))]
    #[case("print(\"a b\", chr(1+2), a*4);", true, true, Node::program(vec![Node::stmt(StmtToken::Print, vec![Node::stmt(StmtToken::StrlitOut, vec![Node::strlit("\"a b\"")]), Node::stmt(StmtToken::ChrCall, vec![Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::numlit("2"))])]), Node::expr(ExprToken::bin_op("*"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::numlit("4"))])])]))]
    #[case("print(chr(1-2), \"a b\", 4/a);", true, true, Node::program(vec![Node::stmt(StmtToken::Print, vec![Node::stmt(StmtToken::ChrCall, vec![Node::expr(ExprToken::bin_op("-"), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::numlit("2"))])]), Node::stmt(StmtToken::StrlitOut, vec![Node::strlit("\"a b\"")]), Node::expr(ExprToken::bin_op("/"), vec![Node::value(ValueToken::numlit("4")), Node::value(ValueToken::strlit("a"))])])]))]
    #[case("print(a+xyz_3[b*(c==d-f)]%g<=h);", true, true, Node::program(vec![Node::stmt(StmtToken::Print, vec![Node::expr(ExprToken::bin_op("<="), vec![Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("%"), vec![Node::expr(ExprToken::ArrayVar, vec![Node::strlit("xyz_3"), Node::expr(ExprToken::bin_op("*"), vec![Node::value(ValueToken::strlit("b")), Node::expr(ExprToken::bin_op("=="), vec![Node::value(ValueToken::strlit("c")), Node::expr(ExprToken::bin_op("-"), vec![Node::value(ValueToken::strlit("d")), Node::value(ValueToken::strlit("f"))])])])]), Node::value(ValueToken::strlit("g"))])]), Node::value(ValueToken::strlit("h"))])])]))]
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
    #[case("func q(){print(abc+3);}", true, true, Node::program(vec![Node::stmt(StmtToken::FuncDef, vec![Node::strlit("q"), Node::program(vec![Node::stmt(StmtToken::Print, vec![Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::strlit("abc")), Node::value(ValueToken::numlit("3"))])])])])]))]
    #[case("func qq(){print(a+x[b*(c==d-f)]%g<=h);}", true, true, Node::program(vec![Node::stmt(StmtToken::FuncDef, vec![Node::strlit("qq"), Node::program(vec![Node::stmt(StmtToken::Print, vec![Node::expr(ExprToken::bin_op("<="), vec![Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("%"), vec![Node::expr(ExprToken::ArrayVar, vec![Node::strlit("x"), Node::expr(ExprToken::bin_op("*"), vec![Node::value(ValueToken::strlit("b")), Node::expr(ExprToken::bin_op("=="), vec![Node::value(ValueToken::strlit("c")), Node::expr(ExprToken::bin_op("-"), vec![Node::value(ValueToken::strlit("d")), Node::value(ValueToken::strlit("f"))])])])]), Node::value(ValueToken::strlit("g"))])]), Node::value(ValueToken::strlit("h"))])])])])]))]
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
    #[case("x=1&&2&&3&&4&&5;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("&&"), vec![Node::expr(ExprToken::bin_op("&&"), vec![Node::expr(ExprToken::bin_op("&&"), vec![Node::expr(ExprToken::bin_op("&&"), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::numlit("2"))]), Node::value(ValueToken::numlit("3"))]), Node::value(ValueToken::numlit("4"))]), Node::value(ValueToken::numlit("5"))])])]))]
    #[case("x=1 || 2 || 3 || 4 || 5;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("||"), vec![Node::expr(ExprToken::bin_op("||"), vec![Node::expr(ExprToken::bin_op("||"), vec![Node::expr(ExprToken::bin_op("||"), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::numlit("2"))]), Node::value(ValueToken::numlit("3"))]), Node::value(ValueToken::numlit("4"))]), Node::value(ValueToken::numlit("5"))])])]))]
    #[case("x=1+2+3+4+5;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("+"), vec![Node::expr(ExprToken::bin_op("+"), vec![Node::expr(ExprToken::bin_op("+"), vec![Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::numlit("2"))]), Node::value(ValueToken::numlit("3"))]), Node::value(ValueToken::numlit("4"))]), Node::value(ValueToken::numlit("5"))])])]))]
    #[case("x=1-2-3-4-5;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("-"), vec![Node::expr(ExprToken::bin_op("-"), vec![Node::expr(ExprToken::bin_op("-"), vec![Node::expr(ExprToken::bin_op("-"), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::numlit("2"))]), Node::value(ValueToken::numlit("3"))]), Node::value(ValueToken::numlit("4"))]), Node::value(ValueToken::numlit("5"))])])]))]
    #[case("x=1*2*3*4*5;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("*"), vec![Node::expr(ExprToken::bin_op("*"), vec![Node::expr(ExprToken::bin_op("*"), vec![Node::expr(ExprToken::bin_op("*"), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::numlit("2"))]), Node::value(ValueToken::numlit("3"))]), Node::value(ValueToken::numlit("4"))]), Node::value(ValueToken::numlit("5"))])])]))]
    #[case("x=1/2/3/4/5;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("/"), vec![Node::expr(ExprToken::bin_op("/"), vec![Node::expr(ExprToken::bin_op("/"), vec![Node::expr(ExprToken::bin_op("/"), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::numlit("2"))]), Node::value(ValueToken::numlit("3"))]), Node::value(ValueToken::numlit("4"))]), Node::value(ValueToken::numlit("5"))])])]))]
    #[case("x=1%2%3%4%5;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("%"), vec![Node::expr(ExprToken::bin_op("%"), vec![Node::expr(ExprToken::bin_op("%"), vec![Node::expr(ExprToken::bin_op("%"), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::numlit("2"))]), Node::value(ValueToken::numlit("3"))]), Node::value(ValueToken::numlit("4"))]), Node::value(ValueToken::numlit("5"))])])]))]
    #[case("x=1==2==3==4==5;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("=="), vec![Node::expr(ExprToken::bin_op("=="), vec![Node::expr(ExprToken::bin_op("=="), vec![Node::expr(ExprToken::bin_op("=="), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::numlit("2"))]), Node::value(ValueToken::numlit("3"))]), Node::value(ValueToken::numlit("4"))]), Node::value(ValueToken::numlit("5"))])])]))]
    #[case("x=1!=2!=3!=4!=5;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("!="), vec![Node::expr(ExprToken::bin_op("!="), vec![Node::expr(ExprToken::bin_op("!="), vec![Node::expr(ExprToken::bin_op("!="), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::numlit("2"))]), Node::value(ValueToken::numlit("3"))]), Node::value(ValueToken::numlit("4"))]), Node::value(ValueToken::numlit("5"))])])]))]
    #[case("x=1<2<3<4<5;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("<"), vec![Node::expr(ExprToken::bin_op("<"), vec![Node::expr(ExprToken::bin_op("<"), vec![Node::expr(ExprToken::bin_op("<"), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::numlit("2"))]), Node::value(ValueToken::numlit("3"))]), Node::value(ValueToken::numlit("4"))]), Node::value(ValueToken::numlit("5"))])])]))]
    #[case("x=1<=2<=3<=4<=5;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("<="), vec![Node::expr(ExprToken::bin_op("<="), vec![Node::expr(ExprToken::bin_op("<="), vec![Node::expr(ExprToken::bin_op("<="), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::numlit("2"))]), Node::value(ValueToken::numlit("3"))]), Node::value(ValueToken::numlit("4"))]), Node::value(ValueToken::numlit("5"))])])]))]
    #[case("x=1>2>3>4>5;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op(">"), vec![Node::expr(ExprToken::bin_op(">"), vec![Node::expr(ExprToken::bin_op(">"), vec![Node::expr(ExprToken::bin_op(">"), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::numlit("2"))]), Node::value(ValueToken::numlit("3"))]), Node::value(ValueToken::numlit("4"))]), Node::value(ValueToken::numlit("5"))])])]))]
    #[case("x=1>=2>=3>=4>=5;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op(">="), vec![Node::expr(ExprToken::bin_op(">="), vec![Node::expr(ExprToken::bin_op(">="), vec![Node::expr(ExprToken::bin_op(">="), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::numlit("2"))]), Node::value(ValueToken::numlit("3"))]), Node::value(ValueToken::numlit("4"))]), Node::value(ValueToken::numlit("5"))])])]))]
    #[case("x=! ! ! ! a;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::un_op("!"), vec![Node::expr(ExprToken::un_op("!"), vec![Node::expr(ExprToken::un_op("!"), vec![Node::expr(ExprToken::un_op("!"), vec![Node::value(ValueToken::strlit("a"))])])])])])]))]
    #[case("x=+ + + +a;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::un_op("+"), vec![Node::expr(ExprToken::un_op("+"), vec![Node::expr(ExprToken::un_op("+"), vec![Node::expr(ExprToken::un_op("+"), vec![Node::value(ValueToken::strlit("a"))])])])])])]))]
    #[case("x=- - - -a;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::un_op("-"), vec![Node::expr(ExprToken::un_op("-"), vec![Node::expr(ExprToken::un_op("-"), vec![Node::expr(ExprToken::un_op("-"), vec![Node::value(ValueToken::strlit("a"))])])])])])]))]
    #[case("x=a && b || c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("||"), vec![Node::expr(ExprToken::bin_op("&&"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=a && b==c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("&&"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("=="), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a && b!=c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("&&"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("!="), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a && b<c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("&&"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("<"), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a && b<=c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("&&"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("<="), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a && b>c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("&&"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op(">"), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a && b>=c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("&&"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op(">="), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a && b+c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("&&"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a && b-c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("&&"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("-"), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a && b*c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("&&"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("*"), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a && b/c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("&&"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("/"), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a && b%c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("&&"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("%"), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a || b && c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("&&"), vec![Node::expr(ExprToken::bin_op("||"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=a || b==c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("||"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("=="), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a || b!=c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("||"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("!="), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a || b<c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("||"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("<"), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a || b<=c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("||"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("<="), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a || b>c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("||"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op(">"), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a || b>=c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("||"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op(">="), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a || b+c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("||"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a || b-c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("||"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("-"), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a || b*c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("||"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("*"), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a || b/c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("||"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("/"), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a || b%c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("||"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("%"), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a==b>c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op(">"), vec![Node::expr(ExprToken::bin_op("=="), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=a==b+c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("=="), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a==b-c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("=="), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("-"), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a==b*c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("=="), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("*"), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a==b/c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("=="), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("/"), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a==b%c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("=="), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("%"), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a>b==c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("=="), vec![Node::expr(ExprToken::bin_op(">"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=a>b+c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op(">"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a>b-c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op(">"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("-"), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a>b*c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op(">"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("*"), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a>b/c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op(">"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("/"), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a>b%c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op(">"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("%"), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a+b==c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("=="), vec![Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=a+b>c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op(">"), vec![Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=a+b-c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("-"), vec![Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=a+b*c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("*"), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a+b/c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("/"), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a+b%c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("%"), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a-b==c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("=="), vec![Node::expr(ExprToken::bin_op("-"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=a-b>c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op(">"), vec![Node::expr(ExprToken::bin_op("-"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=a-b+c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("+"), vec![Node::expr(ExprToken::bin_op("-"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=a-b*c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("-"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("*"), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a-b/c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("-"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("/"), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a-b%c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("-"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("%"), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a*b==c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("=="), vec![Node::expr(ExprToken::bin_op("*"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=a*b>c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op(">"), vec![Node::expr(ExprToken::bin_op("*"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=a*b+c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("+"), vec![Node::expr(ExprToken::bin_op("*"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=a*b-c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("-"), vec![Node::expr(ExprToken::bin_op("*"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=a*b/c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("/"), vec![Node::expr(ExprToken::bin_op("*"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=a*b%c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("%"), vec![Node::expr(ExprToken::bin_op("*"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=a/b==c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("=="), vec![Node::expr(ExprToken::bin_op("/"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=a/b>c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op(">"), vec![Node::expr(ExprToken::bin_op("/"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=a/b+c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("+"), vec![Node::expr(ExprToken::bin_op("/"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=a/b-c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("-"), vec![Node::expr(ExprToken::bin_op("/"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=a/b*c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("*"), vec![Node::expr(ExprToken::bin_op("/"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=a/b%c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("%"), vec![Node::expr(ExprToken::bin_op("/"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=a%b==c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("=="), vec![Node::expr(ExprToken::bin_op("%"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=a%b>c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op(">"), vec![Node::expr(ExprToken::bin_op("%"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=a%b+c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("+"), vec![Node::expr(ExprToken::bin_op("%"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=a%b-c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("-"), vec![Node::expr(ExprToken::bin_op("%"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=a%b*c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("*"), vec![Node::expr(ExprToken::bin_op("%"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=a%b/c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("/"), vec![Node::expr(ExprToken::bin_op("%"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=! a && b;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("&&"), vec![Node::expr(ExprToken::un_op("!"), vec![Node::value(ValueToken::strlit("a"))]), Node::value(ValueToken::strlit("b"))])])]))]
    #[case("x=! a || b;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("||"), vec![Node::expr(ExprToken::un_op("!"), vec![Node::value(ValueToken::strlit("a"))]), Node::value(ValueToken::strlit("b"))])])]))]
    #[case("x=! a==b;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("=="), vec![Node::expr(ExprToken::un_op("!"), vec![Node::value(ValueToken::strlit("a"))]), Node::value(ValueToken::strlit("b"))])])]))]
    #[case("x=! a!=b;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("!="), vec![Node::expr(ExprToken::un_op("!"), vec![Node::value(ValueToken::strlit("a"))]), Node::value(ValueToken::strlit("b"))])])]))]
    #[case("x=! a<b;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("<"), vec![Node::expr(ExprToken::un_op("!"), vec![Node::value(ValueToken::strlit("a"))]), Node::value(ValueToken::strlit("b"))])])]))]
    #[case("x=! a<=b;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("<="), vec![Node::expr(ExprToken::un_op("!"), vec![Node::value(ValueToken::strlit("a"))]), Node::value(ValueToken::strlit("b"))])])]))]
    #[case("x=! a>b;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op(">"), vec![Node::expr(ExprToken::un_op("!"), vec![Node::value(ValueToken::strlit("a"))]), Node::value(ValueToken::strlit("b"))])])]))]
    #[case("x=! a>=b;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op(">="), vec![Node::expr(ExprToken::un_op("!"), vec![Node::value(ValueToken::strlit("a"))]), Node::value(ValueToken::strlit("b"))])])]))]
    #[case("x=! a+b;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("+"), vec![Node::expr(ExprToken::un_op("!"), vec![Node::value(ValueToken::strlit("a"))]), Node::value(ValueToken::strlit("b"))])])]))]
    #[case("x=! a-b;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("-"), vec![Node::expr(ExprToken::un_op("!"), vec![Node::value(ValueToken::strlit("a"))]), Node::value(ValueToken::strlit("b"))])])]))]
    #[case("x=! a*b;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("*"), vec![Node::expr(ExprToken::un_op("!"), vec![Node::value(ValueToken::strlit("a"))]), Node::value(ValueToken::strlit("b"))])])]))]
    #[case("x=! a/b;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("/"), vec![Node::expr(ExprToken::un_op("!"), vec![Node::value(ValueToken::strlit("a"))]), Node::value(ValueToken::strlit("b"))])])]))]
    #[case("x=! a%b;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("%"), vec![Node::expr(ExprToken::un_op("!"), vec![Node::value(ValueToken::strlit("a"))]), Node::value(ValueToken::strlit("b"))])])]))]
    #[case("x=a!=+b;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("!="), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::un_op("+"), vec![Node::value(ValueToken::strlit("b"))])])])]))]
    #[case("x=-a<c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("<"), vec![Node::expr(ExprToken::un_op("-"), vec![Node::value(ValueToken::strlit("a"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=a+ +b;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::un_op("+"), vec![Node::value(ValueToken::strlit("b"))])])])]))]
    #[case("x=a+-b;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::un_op("-"), vec![Node::value(ValueToken::strlit("b"))])])])]))]
    #[case("x=+a+b;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("+"), vec![Node::expr(ExprToken::un_op("+"), vec![Node::value(ValueToken::strlit("a"))]), Node::value(ValueToken::strlit("b"))])])]))]
    #[case("x=-a+b;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("+"), vec![Node::expr(ExprToken::un_op("-"), vec![Node::value(ValueToken::strlit("a"))]), Node::value(ValueToken::strlit("b"))])])]))]
    #[case("x=a-+b;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("-"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::un_op("+"), vec![Node::value(ValueToken::strlit("b"))])])])]))]
    #[case("x=a- -b;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("-"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::un_op("-"), vec![Node::value(ValueToken::strlit("b"))])])])]))]
    #[case("x=+a-b;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("-"), vec![Node::expr(ExprToken::un_op("+"), vec![Node::value(ValueToken::strlit("a"))]), Node::value(ValueToken::strlit("b"))])])]))]
    #[case("x=-a-b;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("-"), vec![Node::expr(ExprToken::un_op("-"), vec![Node::value(ValueToken::strlit("a"))]), Node::value(ValueToken::strlit("b"))])])]))]
    #[case("x=a*-b;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("*"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::un_op("-"), vec![Node::value(ValueToken::strlit("b"))])])])]))]
    #[case("x=+a*c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("*"), vec![Node::expr(ExprToken::un_op("+"), vec![Node::value(ValueToken::strlit("a"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=a/+b;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("/"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::un_op("+"), vec![Node::value(ValueToken::strlit("b"))])])])]))]
    #[case("x=-a/c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("/"), vec![Node::expr(ExprToken::un_op("-"), vec![Node::value(ValueToken::strlit("a"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=a%-b;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("%"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::un_op("-"), vec![Node::value(ValueToken::strlit("b"))])])])]))]
    #[case("x=+a%c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("%"), vec![Node::expr(ExprToken::un_op("+"), vec![Node::value(ValueToken::strlit("a"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=1 && (2 && 3 && 4) && 5;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("&&"), vec![Node::expr(ExprToken::bin_op("&&"), vec![Node::value(ValueToken::numlit("1")), Node::expr(ExprToken::bin_op("&&"), vec![Node::expr(ExprToken::bin_op("&&"), vec![Node::value(ValueToken::numlit("2")), Node::value(ValueToken::numlit("3"))]), Node::value(ValueToken::numlit("4"))])]), Node::value(ValueToken::numlit("5"))])])]))]
    #[case("x=1 || (2 || 3 || 4) || 5;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("||"), vec![Node::expr(ExprToken::bin_op("||"), vec![Node::value(ValueToken::numlit("1")), Node::expr(ExprToken::bin_op("||"), vec![Node::expr(ExprToken::bin_op("||"), vec![Node::value(ValueToken::numlit("2")), Node::value(ValueToken::numlit("3"))]), Node::value(ValueToken::numlit("4"))])]), Node::value(ValueToken::numlit("5"))])])]))]
    #[case("x=1==(2==3==4)==5;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("=="), vec![Node::expr(ExprToken::bin_op("=="), vec![Node::value(ValueToken::numlit("1")), Node::expr(ExprToken::bin_op("=="), vec![Node::expr(ExprToken::bin_op("=="), vec![Node::value(ValueToken::numlit("2")), Node::value(ValueToken::numlit("3"))]), Node::value(ValueToken::numlit("4"))])]), Node::value(ValueToken::numlit("5"))])])]))]
    #[case("x=1!=(2!=3!=4)!=5;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("!="), vec![Node::expr(ExprToken::bin_op("!="), vec![Node::value(ValueToken::numlit("1")), Node::expr(ExprToken::bin_op("!="), vec![Node::expr(ExprToken::bin_op("!="), vec![Node::value(ValueToken::numlit("2")), Node::value(ValueToken::numlit("3"))]), Node::value(ValueToken::numlit("4"))])]), Node::value(ValueToken::numlit("5"))])])]))]
    #[case("x=1<(2<3<4)<5;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("<"), vec![Node::expr(ExprToken::bin_op("<"), vec![Node::value(ValueToken::numlit("1")), Node::expr(ExprToken::bin_op("<"), vec![Node::expr(ExprToken::bin_op("<"), vec![Node::value(ValueToken::numlit("2")), Node::value(ValueToken::numlit("3"))]), Node::value(ValueToken::numlit("4"))])]), Node::value(ValueToken::numlit("5"))])])]))]
    #[case("x=1<=(2<=3<=4)<=5;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("<="), vec![Node::expr(ExprToken::bin_op("<="), vec![Node::value(ValueToken::numlit("1")), Node::expr(ExprToken::bin_op("<="), vec![Node::expr(ExprToken::bin_op("<="), vec![Node::value(ValueToken::numlit("2")), Node::value(ValueToken::numlit("3"))]), Node::value(ValueToken::numlit("4"))])]), Node::value(ValueToken::numlit("5"))])])]))]
    #[case("x=1>(2>3>4)>5;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op(">"), vec![Node::expr(ExprToken::bin_op(">"), vec![Node::value(ValueToken::numlit("1")), Node::expr(ExprToken::bin_op(">"), vec![Node::expr(ExprToken::bin_op(">"), vec![Node::value(ValueToken::numlit("2")), Node::value(ValueToken::numlit("3"))]), Node::value(ValueToken::numlit("4"))])]), Node::value(ValueToken::numlit("5"))])])]))]
    #[case("x=1>=(2>=3>=4)>=5;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op(">="), vec![Node::expr(ExprToken::bin_op(">="), vec![Node::value(ValueToken::numlit("1")), Node::expr(ExprToken::bin_op(">="), vec![Node::expr(ExprToken::bin_op(">="), vec![Node::value(ValueToken::numlit("2")), Node::value(ValueToken::numlit("3"))]), Node::value(ValueToken::numlit("4"))])]), Node::value(ValueToken::numlit("5"))])])]))]
    #[case("x=1+(2+3+4)+5;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("+"), vec![Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::numlit("1")), Node::expr(ExprToken::bin_op("+"), vec![Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::numlit("2")), Node::value(ValueToken::numlit("3"))]), Node::value(ValueToken::numlit("4"))])]), Node::value(ValueToken::numlit("5"))])])]))]
    #[case("x=1-(2-3-4)-5;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("-"), vec![Node::expr(ExprToken::bin_op("-"), vec![Node::value(ValueToken::numlit("1")), Node::expr(ExprToken::bin_op("-"), vec![Node::expr(ExprToken::bin_op("-"), vec![Node::value(ValueToken::numlit("2")), Node::value(ValueToken::numlit("3"))]), Node::value(ValueToken::numlit("4"))])]), Node::value(ValueToken::numlit("5"))])])]))]
    #[case("x=1*(2*3*4)*5;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("*"), vec![Node::expr(ExprToken::bin_op("*"), vec![Node::value(ValueToken::numlit("1")), Node::expr(ExprToken::bin_op("*"), vec![Node::expr(ExprToken::bin_op("*"), vec![Node::value(ValueToken::numlit("2")), Node::value(ValueToken::numlit("3"))]), Node::value(ValueToken::numlit("4"))])]), Node::value(ValueToken::numlit("5"))])])]))]
    #[case("x=1/(2/3/4)/5;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("/"), vec![Node::expr(ExprToken::bin_op("/"), vec![Node::value(ValueToken::numlit("1")), Node::expr(ExprToken::bin_op("/"), vec![Node::expr(ExprToken::bin_op("/"), vec![Node::value(ValueToken::numlit("2")), Node::value(ValueToken::numlit("3"))]), Node::value(ValueToken::numlit("4"))])]), Node::value(ValueToken::numlit("5"))])])]))]
    #[case("x=1%(2%3%4)%5;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("%"), vec![Node::expr(ExprToken::bin_op("%"), vec![Node::value(ValueToken::numlit("1")), Node::expr(ExprToken::bin_op("%"), vec![Node::expr(ExprToken::bin_op("%"), vec![Node::value(ValueToken::numlit("2")), Node::value(ValueToken::numlit("3"))]), Node::value(ValueToken::numlit("4"))])]), Node::value(ValueToken::numlit("5"))])])]))]
    #[case("x=(a==b)+c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("+"), vec![Node::expr(ExprToken::bin_op("=="), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=(a!=b)-c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("-"), vec![Node::expr(ExprToken::bin_op("!="), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=(a<b)*c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("*"), vec![Node::expr(ExprToken::bin_op("<"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=(a<=b)/c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("/"), vec![Node::expr(ExprToken::bin_op("<="), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=(a>b)%c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("%"), vec![Node::expr(ExprToken::bin_op(">"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=a+(b>=c);", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op(">="), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=(a-b)*c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("*"), vec![Node::expr(ExprToken::bin_op("-"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=(a+b)%c;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("%"), vec![Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])])]))]
    #[case("x=a*(b==c);", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("*"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("=="), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a/(b!=c);", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("/"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("!="), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=a%(b<c);", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("%"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("<"), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))])])])]))]
    #[case("x=+(a<=b);", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::un_op("+"), vec![Node::expr(ExprToken::bin_op("<="), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))])])])]))]
    #[case("x=-(a>b);", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::un_op("-"), vec![Node::expr(ExprToken::bin_op(">"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))])])])]))]
    #[case("x=+(a+b);", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::un_op("+"), vec![Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))])])])]))]
    #[case("x=-(a-b);", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::un_op("-"), vec![Node::expr(ExprToken::bin_op("-"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))])])])]))]
    #[case("x=+(a*b);", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::un_op("+"), vec![Node::expr(ExprToken::bin_op("*"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))])])])]))]
    #[case("x=-(a/b);", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::un_op("-"), vec![Node::expr(ExprToken::bin_op("/"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))])])])]))]
    #[case("x=+(a%b);", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::un_op("+"), vec![Node::expr(ExprToken::bin_op("%"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))])])])]))]
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
    #[case("x=readint();", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::stmt(StmtToken::ReadCall, vec![])])]))]
    #[case("x=readint(y);", false, false, Node::none())]
    #[case("x=readint;", false, false, Node::none())]
    #[case("x=readint);", false, false, Node::none())]
    #[case("x=readint(;", false, false, Node::none())]
    #[case("readint();", true, false, Node::none())]
    #[case("x=readint();y=readint();", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::stmt(StmtToken::ReadCall, vec![])]), Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("y")), Node::stmt(StmtToken::ReadCall, vec![])])]))]
    #[case("x=rnd(1);", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::stmt(StmtToken::RndCall, vec![Node::value(ValueToken::numlit("1"))])])]))]
    #[case("x=rnd();", false, false, Node::none())]
    #[case("x=rnd;", false, false, Node::none())]
    #[case("x=rnd 1);", false, false, Node::none())]
    #[case("x=rnd(1;", false, false, Node::none())]
    #[case("rnd(1);", true, false, Node::none())]
    #[case("x=rnd(a);y=rnd(0);", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::stmt(StmtToken::RndCall, vec![Node::value(ValueToken::strlit("a"))])]), Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("y")), Node::stmt(StmtToken::RndCall, vec![Node::value(ValueToken::numlit("0"))])])]))]
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
    #[case("a[1] = 2;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::expr(ExprToken::ArrayVar, vec![Node::strlit("a"), Node::value(ValueToken::numlit("1"))]), Node::value(ValueToken::numlit("2"))])]))]
    #[case("a = b[2];", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::ArrayVar, vec![Node::strlit("b"), Node::value(ValueToken::numlit("2"))])])]))]
    #[case("abc[5*2+a]=bcd[5<=1/4]/cde[! 0>x];", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::expr(ExprToken::ArrayVar, vec![Node::strlit("abc"), Node::expr(ExprToken::bin_op("+"), vec![Node::expr(ExprToken::bin_op("*"), vec![Node::value(ValueToken::numlit("5")), Node::value(ValueToken::numlit("2"))]), Node::value(ValueToken::strlit("a"))])]), Node::expr(ExprToken::bin_op("/"), vec![Node::expr(ExprToken::ArrayVar, vec![Node::strlit("bcd"), Node::expr(ExprToken::bin_op("<="), vec![Node::value(ValueToken::numlit("5")), Node::expr(ExprToken::bin_op("/"), vec![Node::value(ValueToken::numlit("1")), Node::value(ValueToken::numlit("4"))])])]), Node::expr(ExprToken::ArrayVar, vec![Node::strlit("cde"), Node::expr(ExprToken::bin_op(">"), vec![Node::expr(ExprToken::un_op("!"), vec![Node::value(ValueToken::numlit("0"))]), Node::value(ValueToken::strlit("x"))])])])])]))]
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
    #[case("x=a==b+c[x-y[2]]*+d!=e-f/-g<h+i%+j;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("<"), vec![Node::expr(ExprToken::bin_op("!="), vec![Node::expr(ExprToken::bin_op("=="), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::strlit("b")), Node::expr(ExprToken::bin_op("*"), vec![Node::expr(ExprToken::ArrayVar, vec![Node::strlit("c"), Node::expr(ExprToken::bin_op("-"), vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::ArrayVar, vec![Node::strlit("y"), Node::value(ValueToken::numlit("2"))])])]), Node::expr(ExprToken::un_op("+"), vec![Node::value(ValueToken::strlit("d"))])])])]), Node::expr(ExprToken::bin_op("-"), vec![Node::value(ValueToken::strlit("e")), Node::expr(ExprToken::bin_op("/"), vec![Node::value(ValueToken::strlit("f")), Node::expr(ExprToken::un_op("-"), vec![Node::value(ValueToken::strlit("g"))])])])]), Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::strlit("h")), Node::expr(ExprToken::bin_op("%"), vec![Node::value(ValueToken::strlit("i")), Node::expr(ExprToken::un_op("+"), vec![Node::value(ValueToken::strlit("j"))])])])])])]))]
    #[case("x=a==b+(c*+(d!=e[2*z]-f/-g)<h+i)%+j;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op("=="), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::strlit("b")), Node::expr(ExprToken::bin_op("%"), vec![Node::expr(ExprToken::bin_op("<"), vec![Node::expr(ExprToken::bin_op("*"), vec![Node::value(ValueToken::strlit("c")), Node::expr(ExprToken::un_op("+"), vec![Node::expr(ExprToken::bin_op("!="), vec![Node::value(ValueToken::strlit("d")), Node::expr(ExprToken::bin_op("-"), vec![Node::expr(ExprToken::ArrayVar, vec![Node::strlit("e"), Node::expr(ExprToken::bin_op("*"), vec![Node::value(ValueToken::numlit("2")), Node::value(ValueToken::strlit("z"))])]), Node::expr(ExprToken::bin_op("/"), vec![Node::value(ValueToken::strlit("f")), Node::expr(ExprToken::un_op("-"), vec![Node::value(ValueToken::strlit("g"))])])])])])]), Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::strlit("h")), Node::value(ValueToken::strlit("i"))])]), Node::expr(ExprToken::un_op("+"), vec![Node::value(ValueToken::strlit("j"))])])])])])]))]
    #[case("x=a[x[y[z]]%4]+ +b*c<=d- -e/f>g+-h%i>=j;", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op(">="), vec![Node::expr(ExprToken::bin_op(">"), vec![Node::expr(ExprToken::bin_op("<="), vec![Node::expr(ExprToken::bin_op("+"), vec![Node::expr(ExprToken::ArrayVar, vec![Node::strlit("a"), Node::expr(ExprToken::bin_op("%"), vec![Node::expr(ExprToken::ArrayVar, vec![Node::strlit("x"), Node::expr(ExprToken::ArrayVar, vec![Node::strlit("y"), Node::value(ValueToken::strlit("z"))])]), Node::value(ValueToken::numlit("4"))])]), Node::expr(ExprToken::bin_op("*"), vec![Node::expr(ExprToken::un_op("+"), vec![Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("c"))])]), Node::expr(ExprToken::bin_op("-"), vec![Node::value(ValueToken::strlit("d")), Node::expr(ExprToken::bin_op("/"), vec![Node::expr(ExprToken::un_op("-"), vec![Node::value(ValueToken::strlit("e"))]), Node::value(ValueToken::strlit("f"))])])]), Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::strlit("g")), Node::expr(ExprToken::bin_op("%"), vec![Node::expr(ExprToken::un_op("-"), vec![Node::value(ValueToken::strlit("h"))]), Node::value(ValueToken::strlit("i"))])])]), Node::value(ValueToken::strlit("j"))])])]))]
    #[case("x=a+ +(b*c<=d)- -e/(f>g+-h%i)>=j[-z];", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::bin_op(">="), vec![Node::expr(ExprToken::bin_op("-"), vec![Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::strlit("a")), Node::expr(ExprToken::un_op("+"), vec![Node::expr(ExprToken::bin_op("<="), vec![Node::expr(ExprToken::bin_op("*"), vec![Node::value(ValueToken::strlit("b")), Node::value(ValueToken::strlit("c"))]), Node::value(ValueToken::strlit("d"))])])]), Node::expr(ExprToken::bin_op("/"), vec![Node::expr(ExprToken::un_op("-"), vec![Node::value(ValueToken::strlit("e"))]), Node::expr(ExprToken::bin_op(">"), vec![Node::value(ValueToken::strlit("f")), Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::strlit("g")), Node::expr(ExprToken::bin_op("%"), vec![Node::expr(ExprToken::un_op("-"), vec![Node::value(ValueToken::strlit("h"))]), Node::value(ValueToken::strlit("i"))])])])])]), Node::expr(ExprToken::ArrayVar, vec![Node::strlit("j"), Node::expr(ExprToken::un_op("-"), vec![Node::value(ValueToken::strlit("z"))])])])])]))]
    #[case("print(rnd(readint()==15e3),rnd(rnd(rnd(readint()))));", true, true, Node::program(vec![Node::stmt(StmtToken::Print, vec![Node::stmt(StmtToken::RndCall, vec![Node::expr(ExprToken::bin_op("=="), vec![Node::stmt(StmtToken::ReadCall, vec![]), Node::value(ValueToken::numlit("15e3"))])]), Node::stmt(StmtToken::RndCall, vec![Node::stmt(StmtToken::RndCall, vec![Node::stmt(StmtToken::RndCall, vec![Node::stmt(StmtToken::ReadCall, vec![])])])])])]))]
    #[case("x=a==b+c*+d!=e-/-g<h+i%+j;", false, false, Node::none())]
    #[case("x=a==b+(c*+(d!=e-f/-g)<h+i)%+;", false, false, Node::none())]
    #[case("x=a++b*c<=d- -e x/f>g+-h%i>=j;", false, false, Node::none())]
    #[case("x=a++b*c<=d)- -e/(f>g+-h%i)>=j;", false, false, Node::none())]
    #[case("x=((a[(b[c[(d[((e[f]))])]])]));", true, true, Node::program(vec![Node::stmt(StmtToken::Assn, vec![Node::value(ValueToken::strlit("x")), Node::expr(ExprToken::ArrayVar, vec![Node::strlit("a"), Node::expr(ExprToken::ArrayVar, vec![Node::strlit("b"), Node::expr(ExprToken::ArrayVar, vec![Node::strlit("c"), Node::expr(ExprToken::ArrayVar, vec![Node::strlit("d"), Node::expr(ExprToken::ArrayVar, vec![Node::strlit("e"), Node::value(ValueToken::strlit("f"))])])])])])])]))]
    #[case("x=((a[(b[c[(d[((e[f]))]])])]));", false, false, Node::none())]
    #[case("while((a+b)%d+a()!=1){print();}", true, true, Node::program(vec![Node::stmt(StmtToken::While, vec![Node::expr(ExprToken::bin_op("!="), vec![Node::expr(ExprToken::bin_op("+"), vec![Node::expr(ExprToken::bin_op("%"), vec![Node::expr(ExprToken::bin_op("+"), vec![Node::value(ValueToken::strlit("a")), Node::value(ValueToken::strlit("b"))]), Node::value(ValueToken::strlit("d"))]), Node::stmt(StmtToken::FuncCall, vec![Node::strlit("a")])]), Node::value(ValueToken::numlit("1"))]), Node::program(vec![Node::stmt(StmtToken::Print, vec![])])])]))]
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
