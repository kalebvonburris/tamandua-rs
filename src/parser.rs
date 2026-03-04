//! # Parser
//!
//! Predictive recursive-descent parser for the Tamandua language.
//!
//! ## Entry point
//! Call [`Parser::parse`], which returns `(good, done, ast)` — mirroring
//! the Lua `parseit.parse` contract:
//! - `good` — the consumed tokens form a valid program prefix.
//! - `done` — the entire token stream was consumed.
//! - `ast`  — valid only when both flags are `true`.

use crate::lexer::{Lexeme, Lexer};

/// Parses a Tamandua program from source code.
pub fn parse(code: String) -> (bool, bool, Node) {
    let mut lexer = Lexer::new(code);
    let tokens = lexer.lex_input();
    Parser::new(tokens).parse()
}

// ─── AST ─────────────────────────────────────────────────────────────────────

/// A single node in the abstract syntax tree.
///
/// The node kind is *data*, not a type variant — adding new statement or
/// expression forms requires no changes to this struct.
#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub kind: NodeKind,
    pub children: Vec<Node>,
}

/// Coarse structural category of a [`Node`].
///
/// These drive *how* a node is processed, not *which specific* construct it
/// represents — that detail lives in the tag string of each variant.
#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    /// No node; used for error recovery and as a placeholder for missing children.
    None,
    /// Root node of a program or function body.
    Program,
    /// A statement, identified by a string tag (e.g. `"assn"`, `"while"`).
    Stmt(ParseToken),
    /// An expression or operator node, tagged by the operator (e.g. `"+"`).
    Expr(Box<str>),
    /// A leaf: an identifier, literal, or raw lexeme string.
    Value(Box<str>),
}

impl Node {
    /// Constructs a `None` node with no children.
    pub fn none() -> Self {
        Self {
            kind: NodeKind::None,
            children: vec![],
        }
    }

    /// Constructs a `Program` node from a list of statement children.
    pub fn program(stmts: Vec<Node>) -> Self {
        Self {
            kind: NodeKind::Program,
            children: stmts,
        }
    }

    /// Constructs a `Stmt` node with the given tag and children.
    pub fn stmt(tag: ParseToken, children: Vec<Node>) -> Self {
        Self {
            kind: NodeKind::Stmt(tag),
            children,
        }
    }

    /// Constructs an `Expr` node with the given operator tag and children.
    pub fn expr(op: &str, children: Vec<Node>) -> Self {
        Self {
            kind: NodeKind::Expr(op.into()),
            children,
        }
    }

    /// Constructs a leaf `Value` node with no children.
    pub fn value(s: &str) -> Self {
        Self {
            kind: NodeKind::Value(s.into()),
            children: vec![],
        }
    }
}

// ─── Parser ──────────────────────────────────────────────────────────────────

/// Predictive recursive-descent parser for Tamandua.
pub struct Parser {
    /// Token stream produced by the lexer.
    tokens: Vec<(Lexeme, String)>,
    lexstr: String,
    lexcat: Lexeme,
}

impl Parser {
    /// Creates a new parser from a lexer token stream.
    pub fn new(tokens: Vec<(Lexeme, String)>) -> Self {
        Self {
            tokens,
            lexstr: String::new(),
            lexcat: Lexeme::None,
        }
    }

    /// Consumes the next token, updating the current lexeme category and string.
    fn advance(&mut self) {
        if let Some((cat, s)) = self.tokens.pop() {
            self.lexcat = cat;
            self.lexstr = s;
        } else {
            self.lexcat = Lexeme::None;
            self.lexstr.clear();
        }
    }

    /// Checks if the next token matches the expected string, consuming it if so.
    pub fn match_str(&mut self, expected: &str) -> bool {
        if self.lexstr == expected {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Checks if the next token matches the expected lexeme category, consuming it if so.
    pub fn match_cat(&mut self, expected: Lexeme) -> bool {
        if self.lexcat == expected {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Parses the token stream as a Tamandua program.
    ///
    /// Returns `(good, done, ast)`:
    /// - `good` — a valid program prefix was parsed.
    /// - `done` — all tokens were consumed.
    /// - `ast`  — the AST; only meaningful when `good && done`.
    pub fn parse(mut self) -> (bool, bool, Node) {
        let (good, ast) = self.parse_program();

        let done = self.tokens.is_empty();

        (good, done, ast)
    }

    fn parse_program(&mut self) -> (bool, Node) {
        let mut good = false;
        let mut ast = Node::program(vec![]);

        while matches!(
            self.lexstr.as_str(),
            ";" | "print" | "println" | "return" | "++" | "--" | "func" | "if" | "while"
        ) || self.lexcat == Lexeme::Identifier
        {
            let (good, ast2) = self.parse_stmt();

            if !good {
                break;
            }

            ast.children.push(ast2);
        }

        (true, ast)
    }

    pub fn parse_stmt(&mut self) -> (bool, Node) {
        let mut good = true;
        let mut ast = Node::none();

        if self.match_str(";") {
            return (true, Node::stmt(ParseToken::Empty, vec![]));
        }

        (good, ast)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseToken {
    Empty,
    Print,
    Println,
    StrlitOut,
    Return,
    Inc,
    Dec,
    FuncDef,
    FuncCall,
    If,
    While,
    Identifier,
    ChrCall,
    RndCall,
    ReadCall,
    Assn,
    BinOp,
    UnOp,
    NumLit,
    SimpleVar,
    ArrayVar,
}
