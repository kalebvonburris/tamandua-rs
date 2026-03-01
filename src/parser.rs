//! # Parser

use crate::lexer::Lexeme;

struct AST {
    category: ASTCategory,
    value: Option<String>,
    tree: Option<Vec<AST>>,
}

enum ASTCategory {
    Program,
}

pub struct Parser {
    code: Vec<(Lexeme, String)>,
}

const STATEMENT_KETWORDS: &[&str] = &[
    ";", "print", "println", "return", "++", "--", "func", "if", "while",
];

impl Parser {
    pub fn new(code: Vec<(Lexeme, String)>) -> Self {
        Self { code }
    }

    pub fn parse(&mut self) -> Option<AST> {
        let mut ast = AST {
            category: ASTCategory::Program,
            value: None,
            tree: None,
        };

        for (lexeme, string) in self.code.iter() {
            if STATEMENT_KETWORDS.contains(&string.as_str()) || *lexeme == Lexeme::Identifier {}
        }

        None
    }
}
