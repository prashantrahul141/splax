use crate::app::app_main::App;
use crate::token::token_main::Token;
use crate::token::token_main::TokenLiterals;
use crate::token::token_types::TokenType;
use std::collections::HashMap;

/// The Lexer struct.
/// converts the entire source input string into tokens.
pub struct Lexer {
    /// Start of the current token being scanned.
    pub start: usize,
    /// Current position of the cursor.
    pub current: usize,
    /// Current line of the cursor.
    pub line: usize,
    /// Total length of the input source string.
    pub len: usize,
    /// Final tokens vector.
    pub tokens: Vec<Token>,
    /// Input source string.
    pub source_string: String,
    /// Input source string in form of a vector of characters.
    pub source_chars: Vec<char>,
    /// contains all reserved keywords.
    pub reserved_keywords: HashMap<String, TokenType>,
}

impl Lexer {
    /// scans individual tokens and add them into lexer's tokens vector.
    pub fn scan_token(&mut self) {
        // consume current char.
        let current_char = self.advance();

        match current_char {
            // single character tokens.
            '(' => self.add_basic_token(TokenType::LeftParen),
            ')' => self.add_basic_token(TokenType::RightParen),
            '{' => self.add_basic_token(TokenType::LeftBrace),
            '}' => self.add_basic_token(TokenType::RightBrace),
            ',' => self.add_basic_token(TokenType::Comma),
            '.' => self.add_basic_token(TokenType::Dot),
            '-' => self.add_basic_token(TokenType::Minus),
            '+' => self.add_basic_token(TokenType::Plus),
            ';' => self.add_basic_token(TokenType::Semicolon),
            '*' => self.add_basic_token(TokenType::Star),
            '%' => self.add_basic_token(TokenType::Mod),

            // multiple character basic tokens.
            // !
            '!' => {
                if self.match_char('=') {
                    // !=
                    self.add_basic_token(TokenType::BangEqual);
                } else {
                    // =
                    self.add_basic_token(TokenType::Bang);
                }
            }

            // =
            '=' => {
                if self.match_char('=') {
                    // ==
                    self.add_basic_token(TokenType::EqualEqual);
                } else {
                    // =
                    self.add_basic_token(TokenType::Equal);
                }
            }

            // <
            '<' => {
                if self.match_char('=') {
                    // <=
                    self.add_basic_token(TokenType::LessEqual);
                } else {
                    // <
                    self.add_basic_token(TokenType::Less);
                }
            }

            // >
            '>' => {
                if self.match_char('=') {
                    // >=
                    self.add_basic_token(TokenType::GreaterEqual);
                } else {
                    // >
                    self.add_basic_token(TokenType::Greater);
                }
            }

            // longer lexemes
            // /
            '/' => {
                if self.match_char('/') {
                    // if the line is infact a comment.
                    // keep consuming characters untill we reach the end of file
                    // or end of the line.
                    while self.look_ahead() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    // we dont add any token for comments BECAUSE they're comments.
                } else {
                    // its a simple slash.
                    self.add_basic_token(TokenType::Slash);
                }
            }

            // ignore these characters, we dont need'em.
            ' ' | '\t' | '\r' => {
                spdlog::trace!("ignoring whitespaces");
            }

            // newlines, we ignore it, but also increase our line count.
            '\n' => {
                self.line += 1;
                spdlog::trace!("found newline, incrementing line number and skipping.");
            }

            // strings.
            '"' => {
                spdlog::trace!("scanning a string token.");
                self.scan_string();
            }

            // all other types of tokens, this includes literal number and identifiers.
            // we also show an error if we found a character we dont recognize.
            _ => {
                // incase the token starts with a number, then its probably a number.
                if Lexer::is_numeric(current_char) {
                    spdlog::trace!("trying to parse a number.");
                    self.scan_number();
                }
                // incase the token starts with a alphabet, then its probably a identifier.
                else if Lexer::is_alpha(current_char) {
                    spdlog::trace!("trying to parse an indentifier.");
                    self.scan_indentifier();
                }
                // it is safe to assume anything else cannot be considered a safe token to parse or interpret.
                else {
                    App::error(
                        self.line,
                        format!("unexpected character : {}", current_char),
                    );
                }
            }
        }
    }

    /// Loops through entire input source string and calls can token until EOF.
    /// returns a vector of all scanned tokens.
    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        spdlog::debug!("scanning tokens");

        // scan individual tokens until EOF.
        while !self.is_at_end() {
            spdlog::trace!("did not reach end, scanning next token.");
            self.start = self.current;
            self.scan_token();
        }

        spdlog::debug!("reached end of file, stopped scanning.");

        // add a EOF token at the end.
        self.add_token(TokenType::Eof, TokenLiterals::Null);

        spdlog::debug!(
            "done scanning tokens, scanned : {} tokens.",
            self.tokens.len()
        );

        spdlog::trace!("{:?}", self.tokens);

        // return ref
        &self.tokens
    }
}
