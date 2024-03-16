//! Defines the lexer iterator.
//! Parses a string into a sequence of tokens.

use anyhow::{anyhow, Error, Result};

use crate::token::{Span, Token, TokenKind};
use crate::errors::LangError;

/// A lexer iterator.
pub struct Lexer<'a> {
    src: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer from a string.
    pub fn new(src: &'a str) -> Self {
        Self { src, pos: 0 }
    }

    /// Peek at the next character
    fn peek(&self) -> Option<char> {
        self.src[self.pos..].chars().next()
    }

    /// Advance the lexer by one character
    fn advance(&mut self) {
        if let Some(ch) = self.peek() {
            self.pos += ch.len_utf8();
        }
    }

    /// Skip whitespace
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Skip comments
    fn skip_comments(&mut self) {
        if self.peek() == Some('/') && self.src[self.pos + 1..].chars().next() == Some('/') {
            while let Some(ch) = self.peek() {
                if ch == '\n' {
                    break;
                } else {
                    self.advance();
                }
            }
        }
    }

    /// Lex a number
    /// Supports integers and floats
    fn lex_number(&mut self) -> Result<Token> {
        let start = self.pos;
        let mut has_decimal = false;
        while let Some(ch) = self.peek() {
            if ch.is_digit(10) {
                self.advance();
            } else if ch == '.' {
                if has_decimal {
                    self.advance();
                    let val = &self.src[start..self.pos];
                    let span = Span { start, end: self.pos };
                    
                    // advance until whitespace
                    while let Some(ch) = self.peek() {
                        if ch.is_whitespace() {
                            break;
                        } else {
                            self.advance();
                        }
                    }

                    return Err(anyhow!(
                        LangError::UnexpectedCharacter(
                            val.to_string(),
                            span
                        )
                    ));
                } else {
                    has_decimal = true;
                    self.advance();
                }
            } else {
                break;
            }
        }
        let end = self.pos;
        
        let value = &self.src[start..end];
        if value.is_empty() {
            return Err(anyhow!(
                LangError::ExpectedAnyToken {
                    expected: vec![TokenKind::IntegerLiteral(0), TokenKind::FloatLiteral(0.0)],
                    found: "Empty literal".to_string(),
                    span: Span { start, end },
                }
            ));
        }
    
        if has_decimal {
            match value.parse() {
                Ok(f) => Ok(Token {
                    kind: TokenKind::FloatLiteral(f),
                    span: Span { start, end },
                }),
                Err(_) => Err(anyhow!(
                    LangError::InvalidLiteral(
                        value.to_string(),
                        Span { start, end }
                    )
                )),
            }
        } else {
            match value.parse() {
                Ok(i) => Ok(Token {
                    kind: TokenKind::IntegerLiteral(i),
                    span: Span { start, end },
                }),
                Err(_) => Err(anyhow!(
                    LangError::InvalidLiteral(
                        value.to_string(),
                        Span { start, end }
                    )
                )),
            }
        }
    }
    
    /// Lex an operator (may or may not be multi-char)
    /// Always advances the lexer past the operator
    fn lex_op(&mut self) -> Result<Token> {
        let start = self.pos;
        self.advance();
        let mut end = self.pos;

        if let Some(ch) = self.peek() {
            match ch {
                '+' | '-' | '*' | '/' | '%' | '^' | '=' | '<' | '>' | '!' | '&' | '|' => {
                    self.advance();
                    end = self.pos;
                },
                _ => {}
            }
        } else {
            return Err(anyhow!(
                LangError::UnexpectedEOF(Span { start, end })
            ));
        }

        Ok(Token {
            kind: TokenKind::operator_from(&self.src[start..end]),
            span: Span { start, end },
        })
    }

    /// Lex a word
    /// May be a keyword or an identifier
    fn lex_word(&mut self) -> Result<Token> {
        let start = self.pos;
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }
        let end = self.pos;

        let value = &self.src[start..end];
        if value.is_empty() {
            return Err(anyhow!(
                LangError::UnexpectedCharacter(
                    value.to_string(),
                    Span { start, end }
                )
            ));
        }

        Ok(Token {
            kind: TokenKind::keyword_from(value),
            span: Span { start, end },
        })
    }

    /// Lex a single character token
    fn lex_single_char(&mut self, kind: TokenKind) -> Token {
        let start = self.pos;
        self.advance();
        let end = self.pos;
        Token { kind, span: Span { start, end } }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        self.skip_comments();

        if let Some(ch) = self.peek() {
            let token = match ch {
                '+' | '-' | '*' | '/' | '%' | '^' | '=' | '<' | '>' | '!' | '&' | '|' => self.lex_op(),
                '0'..='9' => self.lex_number(),
                'a'..='z' | 'A'..='Z' | '_' => self.lex_word(),
                '{' => Ok(self.lex_single_char(TokenKind::OpenBrace)),
                '}' => Ok(self.lex_single_char(TokenKind::CloseBrace)),
                '(' => Ok(self.lex_single_char(TokenKind::OpenParen)),
                ')' => Ok(self.lex_single_char(TokenKind::CloseParen)),
                ';' => Ok(self.lex_single_char(TokenKind::Semicolon)),
                ',' => Ok(self.lex_single_char(TokenKind::Comma)),
                '.' => Ok(self.lex_single_char(TokenKind::Dot)),
                _ => {
                    // try ident/keyword
                    if ch.is_alphabetic() {
                        self.lex_word()
                    } else {
                        Err(anyhow!(
                            LangError::UnexpectedCharacter(
                                ch.to_string(),
                                Span { start: self.pos, end: self.pos + ch.len_utf8() }
                            )
                        ))
                    }
                }
            };
            
            Some(token)
        } else {
            None
        }
    }
}

/// Consumes and partitions the Lexer iterator into 2 `vec`s for tokens and errors.
pub fn consume_lexer(lexer: Lexer) -> (Vec<Token>, Vec<Error>) {
    let mut tokens = Vec::new();
    let mut errors = Vec::new();
    for token in lexer {
        match token {
            Ok(token) => tokens.push(token),
            Err(err) => errors.push(err),
        }
    }
    (tokens, errors)
}

#[cfg(test)]
mod tests {
    use codespan_reporting::files::SimpleFiles;

    use super::*;
    use crate::errors::ErrorReporter;

    #[test]
    fn lex_general() {
        let src = "if name == 335.333 { return true; }";
        let lexer = Lexer::new(src);
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(tokens.len(), 9);
        println!("{:#?}", tokens);
    }

    #[test]
    fn number_error() {
        let src = "if name == 335.333.2 { return true; }";
        let lexer = Lexer::new(src);

        let mut files = SimpleFiles::new();
        let file_id = files.add("test", src);
        
        let (tokens, errors) = consume_lexer(lexer);
        
        assert_eq!(tokens.len(), 8);
        assert_eq!(errors.len(), 1);

        let reporter = ErrorReporter::with_files(files);

        // display anyhow errors
        for err in errors {
            reporter.report(file_id, &err).expect("Failed to report error");
        }
    }
}