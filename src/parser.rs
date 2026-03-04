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

use crate::lexer::Lexeme;

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
    /// Root node of a program or function body.
    Program,
    /// A statement, identified by a string tag (e.g. `"assn"`, `"while"`).
    Stmt(Box<str>),
    /// An expression or operator node, tagged by the operator (e.g. `"+"`).
    Expr(Box<str>),
    /// A leaf: an identifier, literal, or raw lexeme string.
    Value(Box<str>),
}

impl Node {
    /// Constructs a `Program` node from a list of statement children.
    pub fn program(stmts: Vec<Node>) -> Self {
        Self { kind: NodeKind::Program, children: stmts }
    }

    /// Constructs a `Stmt` node with the given tag and children.
    pub fn stmt(tag: &str, children: Vec<Node>) -> Self {
        Self { kind: NodeKind::Stmt(tag.into()), children }
    }

    /// Constructs an `Expr` node with the given operator tag and children.
    pub fn expr(op: &str, children: Vec<Node>) -> Self {
        Self { kind: NodeKind::Expr(op.into()), children }
    }

    /// Constructs a leaf `Value` node with no children.
    pub fn value(s: &str) -> Self {
        Self { kind: NodeKind::Value(s.into()), children: vec![] }
    }
}

// ─── Parser ──────────────────────────────────────────────────────────────────

/// Predictive recursive-descent parser for Tamandua.
pub struct Parser {
    /// Token stream produced by the lexer.
    tokens: Vec<(Lexeme, String)>,
    /// Index of the current (not yet consumed) token.
    pos: usize,
}

impl Parser {
    /// Creates a new parser from a lexer token stream.
    pub fn new(tokens: Vec<(Lexeme, String)>) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Parses the token stream as a Tamandua program.
    ///
    /// Returns `(good, done, ast)`:
    /// - `good` — a valid program prefix was parsed.
    /// - `done` — all tokens were consumed.
    /// - `ast`  — the AST; only meaningful when `good && done`.
    pub fn parse(mut self) -> (bool, bool, Option<Node>) {
        todo!()
    }
}
