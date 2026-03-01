//! # Lexer

use std::ops::RangeInclusive;

/// A state machine lexer that takes an input
/// block of code and returns a `Vec` of ordered tuple pairs.
pub struct Lexer {
    /// The entirety of the given code block.
    code: Vec<char>,
    /// The current string literal of the [`Lexeme`] being lexed.
    current_lexeme: String,
    /// The category of the current [`Lexeme`] being lexed.
    current_category: Lexeme,
    /// The index into the `code` `Vec`. Can never be `>= code.len()`.
    index: usize,
    /// The current [`State`] of the lexer.
    state: State,
    /// Storage for the [`Lexeme`]s found.
    found_lexems: Vec<(Lexeme, String)>,
}

/// The `State` of a [`Lexer`].
#[derive(Debug, PartialEq)]
pub(crate) enum State {
    Start,
    Letter,
    Num,
    Str,
    Scientific,
    Operator,
    Comment,
    End,
}

/// A given `Lexeme` according to the lexical specification.
#[derive(Debug, PartialEq, Clone)]
pub enum Lexeme {
    None,
    /// See [`KEYWORDS`].
    Keyword,
    /// A `String` that represents either a variable name
    /// or function name.
    Identifier,
    /// A literal numerical value, such as `1`, `2.5`, or `123.4E10`.
    NumericLiteral,
    /// A literal [`String`] value, matched with `/("."|'.')/`.
    StringLiteral,
    /// An operator used by [`NumericLiterals`](NumericLiteral) or
    /// variables.
    Operator,
    /// A possible piece of fluff, such as a semicolor (`';'`),
    /// open/close braces (`'{'`/`'}'`).
    Punctuation,
    /// Some sort of broken state/[`Lexeme`] with an attached
    /// [error](LexingError).
    Malformed(LexingError),
}

#[derive(Debug, PartialEq, Clone)]
pub enum LexingError {
    NonLegalChar,
    NoLexemeFound,
    ImproperNumericLiteral,
    MalformedStringLiteral,
}

/// All legal [`char`]s for the lexical specification.
///
/// These are ASCII `' '`` to ASCII `'~'`.
///
/// ```
/// use undefined_rs::lexer;
/// assert_eq!(' '..='~', lexer::LEGAL_CHARACTERS);
/// ```
pub const LEGAL_CHARACTERS: RangeInclusive<char> = ' '..='~';

/// All legal [`Keywords`](Lexeme::Keyword).
///
/// ```
/// use undefined_rs::lexer;
/// assert_eq!(&["begin", "end", "print"], lexer::KEYWORDS);
/// ```
pub const KEYWORDS: &[&str] = &[
    "chr", "else", "elsif", "func", "if", "print", "println", "readint", "return", "rnd", "while",
];

/// All legal [`Operators`](Lexeme::Operator).
/// ```
/// use undefined_rs::lexer;
///
/// let operators = &[
///   "+", "-", "*", "/", "++", "--", ".", "=", "==", "+=", "-=", "*=", "/=",
/// ];
///
/// assert_eq!(operators, lexer::OPERATORS);
/// ```
pub const OPERATORS: &[&str] = &[
    "+", "-", "*", "/", "=", "==", "!=", "<", "<=", ">", ">=", "!", "&&", "||", "]", "[", "++",
    "--", "%",
];

impl Lexer {
    /// Creates a new [`Lexer`]
    ///
    /// Call [`Lexer::lex_input`] to get the desired output `Vec` of [`Lexeme`]s.
    pub fn new(s: String) -> Self {
        Self {
            code: s.chars().collect(),
            current_lexeme: String::new(),
            current_category: Lexeme::None,
            index: 0,
            state: State::Start,
            found_lexems: Vec::new(),
        }
    }

    /// Lexes the current `self.code` `String` into a `Vec<(Lexeme, String)>`.
    ///
    /// ```
    /// use undefined_rs::lexer::{
    ///     Lexer,
    ///     Lexeme,
    ///     NumericLiteral,
    /// };
    ///
    /// let mut lexer = Lexer::new("2".to_string());
    ///
    /// let expected_output = vec![
    ///     (Lexeme::NumericLiteral(NumericLiteral::Integer),
    ///     "2".to_string()),
    /// ];
    ///
    /// assert_eq!(lexer.lex_input(), expected_output);
    /// ```
    pub fn lex_input(&mut self) -> Vec<(Lexeme, String)> {
        while self.index <= self.code.len() {
            self.handle_state();
        }

        self.found_lexems.clone()
    }

    /// Maps a given [`State`] to its related handler function.
    fn handle_state(&mut self) {
        match self.state {
            State::Start => self.handle_start(),
            State::Letter => self.handle_letter(),
            State::Num => self.handle_num(),
            State::Str => self.handle_str(),
            State::Scientific => self.handle_scientific(),
            State::Operator => self.handle_operator(),
            State::Comment => self.handle_comment(),
            State::End => self.handle_end(),
        }
    }

    /// Handles the state: [`State::Start`]
    ///
    /// Maps the given `curr_char` to a state for relevant lexing.
    fn handle_start(&mut self) {
        match self.curr_char() {
            'a'..='z' | 'A'..='Z' | '_' => {
                self.add_one();
                self.state = State::Letter;
            }
            '0'..='9' => {
                self.add_one();
                self.state = State::Num;
            }
            '+' | '-' | '*' | '/' | '=' | '!' | '<' | '>' | '&' | '|' | ']' | '[' | '%' | '#'
            | '.' => {
                // Comment - NOT an Operator
                if self.curr_char() == '#' {
                    self.drop_one();
                    self.state = State::Comment;
                    return;
                }

                self.add_one();
                self.state = State::Operator;
            }
            '"' | '\'' => {
                self.add_one();
                self.state = State::Str;
            }
            '\n' | ' ' => self.drop_one(),
            _ => {
                if LEGAL_CHARACTERS.contains(&self.curr_char()) {
                    self.add_one();
                    self.current_category = Lexeme::Punctuation;
                    self.state = State::End;
                } else {
                    self.add_one();
                    self.current_category = Lexeme::Malformed(LexingError::NonLegalChar);
                    self.state = State::End;
                }
            }
        };
    }

    /// Handles the state: [`State::Comment`]
    ///
    /// All comments start with `'#'` and continue until a newline or the end of the file.
    fn handle_comment(&mut self) {
        if self.curr_char() == '\n' || self.curr_char() == '\0' {
            self.drop_one();
            self.state = State::Start;
        } else {
            self.drop_one();
        }
    }

    /// Handles the state: [`State::Letter`]
    fn handle_letter(&mut self) {
        if self.curr_char().is_ascii_alphanumeric() || self.curr_char() == '_' {
            self.add_one();
        } else {
            if KEYWORDS.contains(&self.current_lexeme.as_str()) {
                self.current_category = Lexeme::Keyword;
            } else {
                self.current_category = Lexeme::Identifier;
            }
            self.state = State::End;
        }
    }

    /// Handles the state: [`State::NumericLiteral`]
    ///
    /// This is triggered after seeing a number while not in the
    /// [State::Letter](State::Letter) state. Usually lexes out a [`Lexeme::NumericLiteral`].
    fn handle_num(&mut self) {
        match self.curr_char() {
            '0'..='9' | '.' => self.add_one(),
            'e' | 'E' => {
                // Potential Scientific notation
                // Check for '+'
                if self.next_char() == '+' {
                    if self
                        .code
                        .get(self.index + 2)
                        .unwrap_or(&'\0')
                        .is_ascii_digit()
                    {
                        self.add_one();
                        self.add_one();
                        self.state = State::Scientific;
                    } else {
                        self.state = State::End;
                    }
                } else if self.next_char().is_ascii_digit() {
                    self.add_one();
                    self.add_one();
                    self.state = State::Scientific;
                }
            }
            _ => {
                self.current_category = Lexeme::NumericLiteral;
                self.state = State::End;
            }
        }
    }

    /// Handles the state: [`State::Str`]
    fn handle_str(&mut self) {
        let quote_type = self.current_lexeme.chars().next().unwrap();

        while self.curr_char() != quote_type && self.curr_char() != '\0' {
            self.add_one();
        }

        if self.curr_char() == '\0' {
            self.current_category = Lexeme::Malformed(LexingError::MalformedStringLiteral);
        } else {
            self.add_one();
            self.current_category = Lexeme::StringLiteral;
        }

        self.state = State::End;
    }

    /// Handles the state: [`State::Operator`]
    ///
    /// See [`OPERATORS`] for all valid operators.
    fn handle_operator(&mut self) {
        println!("In operator state with lexeme: {}", self.current_lexeme);
        let mut op = self.current_lexeme.clone();

        op.push(self.curr_char());

        if OPERATORS.contains(&op.as_str()) {
            self.add_one();
            self.current_category = Lexeme::Operator;
            self.state = State::End;
            return;
        }

        // +/- state
        if self.current_lexeme == "+" || self.current_lexeme == "-" {
            // +/-[0-9]
            if self.curr_char().is_ascii_digit() {
                self.add_one();
                self.state = State::Num;
            }
            // +/-.[0-9]
            else if self.curr_char() == '.' && self.next_char().is_ascii_digit() {
                self.add_one();
                self.state = State::Num;
            }
            // +/- only
            else {
                self.current_category = Lexeme::Operator;
                self.state = State::End;
            }
        } else if self.current_lexeme == "." {
            // .[0-9]
            if self.curr_char().is_ascii_digit() {
                self.add_one();
                self.state = State::Num;
            } else {
                self.current_category = Lexeme::Operator;
                self.state = State::End;
            }
        } else {
            self.current_category = Lexeme::Operator;
            self.state = State::End;
        }

        println!(
            "Exiting operator state with lexeme: {} in state {:?} and category {:?}",
            self.current_lexeme, self.state, self.current_category
        );
    }

    /// Handles the state: [`State::NumericLiteral(NumericLiteral::Scientific)`]
    ///
    /// All scientific literals must be in the form `/[0-9]*.?[0-9]+(e|E)-?[0-9]+/`.
    fn handle_scientific(&mut self) {
        while self.curr_char().is_ascii_digit() {
            self.add_one();
        }

        self.current_category = Lexeme::NumericLiteral;
        self.state = State::End;
    }

    /// Handles the state: [`State::End`]
    ///
    /// Pushes the `self.current_lexeme` and `self.current_category` onto
    /// the `self.found_lexems` [`Vec`].
    fn handle_end(&mut self) {
        self.found_lexems
            .push((self.current_category.clone(), self.current_lexeme.clone()));

        self.current_lexeme = String::new();
        self.current_category = Lexeme::None;
        self.state = State::Start;
    }

    /// Pushes the next `char` in `self.code` to `self.CurrentLexeme`
    /// and iterates `self.index`.
    fn add_one(&mut self) {
        let c = self.curr_char();

        self.current_lexeme.push(c);
        self.index += 1;
    }

    /// Iterates `self.index`, effectively dropping the next `char`
    /// of `self.code`.
    fn drop_one(&mut self) {
        self.index += 1;
    }

    /// Gets the `char` at `self.code[self.index]`.
    fn curr_char(&mut self) -> char {
        *self.code.get(self.index).unwrap_or(&'\0')
    }

    /// Gets the `char` at `self.code[self.index + 1]`
    fn next_char(&mut self) -> char {
        *self.code.get(self.index + 1).unwrap_or(&'\0')
    }
}
