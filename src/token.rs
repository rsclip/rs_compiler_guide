//! Represents a token and their respective information.

use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // keywords
    Fn,
    If,
    Else,
    While,
    For,
    Return,
    Let,
    True,
    False,

    // single-character tokens
    Plus,      // +
    Minus,     // -
    Star,      // *
    Slash,     // /
    Percent,   // %
    Caret,     // ^
    Bang,      // !
    Colon,     // :
    Semicolon, // ;
    Comma,     // ,
    Dot,       // .
    Equals,    // =
    Less,      // <
    Greater,   // >
    LParen,    // (
    RParen,    // )
    LBrace,    // {
    RBrace,    // }
    Quote,     // "

    // double-character tokens
    PlusEquals,    // +=
    MinusEquals,   // -=
    StarEquals,    // *=
    SlashEquals,   // /=
    PercentEquals, // %=
    CaretEquals,   // ^=
    BangEquals,    // !=
    LessEquals,    // <=
    GreaterEquals, // >=
    EqualsEquals,  // ==
    Arrow,         // ->

    // literals
    Ident(String),
    IntLiteral(i32),
    FloatLiteral(f32),
    BoolLiteral(bool),

    // Data types
    Int,
    Bool,
    Float,

    // End of file
    Eof,
}

impl TokenKind {
    pub fn operator_from(op: &str) -> TokenKind {
        match op {
            "+" => TokenKind::Plus,
            "-" => TokenKind::Minus,
            "*" => TokenKind::Star,
            "/" => TokenKind::Slash,
            "%" => TokenKind::Percent,
            "^" => TokenKind::Caret,
            "!" => TokenKind::Bang,
            ":" => TokenKind::Colon,
            ";" => TokenKind::Semicolon,
            "," => TokenKind::Comma,
            "." => TokenKind::Dot,
            "=" => TokenKind::Equals,
            "<" => TokenKind::Less,
            ">" => TokenKind::Greater,
            "+=" => TokenKind::PlusEquals,
            "-=" => TokenKind::MinusEquals,
            "*=" => TokenKind::StarEquals,
            "/=" => TokenKind::SlashEquals,
            "%=" => TokenKind::PercentEquals,
            "^=" => TokenKind::CaretEquals,
            "!=" => TokenKind::BangEquals,
            "<=" => TokenKind::LessEquals,
            ">=" => TokenKind::GreaterEquals,
            "==" => TokenKind::EqualsEquals,
            "->" => TokenKind::Arrow,
            _ => panic!("Unknown operator: {}", op),
        }
    }

    pub fn keyword_from(keyword: &str) -> TokenKind {
        match keyword {
            "fn" => TokenKind::Fn,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            "return" => TokenKind::Return,
            "let" => TokenKind::Let,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "int" => TokenKind::Int,
            "bool" => TokenKind::Bool,
            "float" => TokenKind::Float,
            _ => TokenKind::Ident(keyword.to_string()),
        }
    }
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TokenKind::Fn => write!(f, "fn"),
            TokenKind::If => write!(f, "if"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::While => write!(f, "while"),
            TokenKind::For => write!(f, "for"),
            TokenKind::Return => write!(f, "return"),
            TokenKind::Let => write!(f, "let"),
            TokenKind::True => write!(f, "true"),
            TokenKind::False => write!(f, "false"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Percent => write!(f, "%"),
            TokenKind::Caret => write!(f, "^"),
            TokenKind::Bang => write!(f, "!"),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Dot => write!(f, "."),
            TokenKind::Equals => write!(f, "="),
            TokenKind::Less => write!(f, "<"),
            TokenKind::Greater => write!(f, ">"),
            TokenKind::LParen => write!(f, "("),
            TokenKind::RParen => write!(f, ")"),
            TokenKind::LBrace => write!(f, "{{"),
            TokenKind::RBrace => write!(f, "}}"),
            TokenKind::Quote => write!(f, "\""),
            TokenKind::PlusEquals => write!(f, "+="),
            TokenKind::MinusEquals => write!(f, "-="),
            TokenKind::StarEquals => write!(f, "*="),
            TokenKind::SlashEquals => write!(f, "/="),
            TokenKind::PercentEquals => write!(f, "%="),
            TokenKind::CaretEquals => write!(f, "^="),
            TokenKind::BangEquals => write!(f, "!="),
            TokenKind::LessEquals => write!(f, "<="),
            TokenKind::GreaterEquals => write!(f, ">="),
            TokenKind::EqualsEquals => write!(f, "=="),
            TokenKind::Arrow => write!(f, "->"),
            TokenKind::Int => write!(f, "int"),
            TokenKind::Bool => write!(f, "bool"),
            TokenKind::Float => write!(f, "float"),
            TokenKind::Eof => write!(f, "EOF"),
            TokenKind::Ident(id) => write!(f, "{}", id),
            TokenKind::IntLiteral(_) => write!(f, "IntegerLiteral"),
            TokenKind::FloatLiteral(_) => write!(f, "FloatLiteral"),
            TokenKind::BoolLiteral(_) => write!(f, "BooleanLiteral"),
        }
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

impl Hash for TokenKind {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash each variant discriminant and associated value
        match self {
            TokenKind::Fn => "Fn".hash(state),
            TokenKind::If => "If".hash(state),
            TokenKind::Else => "Else".hash(state),
            TokenKind::While => "While".hash(state),
            TokenKind::For => "For".hash(state),
            TokenKind::Return => "Return".hash(state),
            TokenKind::Let => "Let".hash(state),
            TokenKind::True => "True".hash(state),
            TokenKind::False => "False".hash(state),
            TokenKind::Ident(identifier) => identifier.hash(state),
            TokenKind::IntLiteral(integer) => integer.hash(state),
            TokenKind::FloatLiteral(float) => {
                // Hash the float value as a bit pattern
                let bytes = float.to_ne_bytes();
                bytes.hash(state);
            }
            TokenKind::BoolLiteral(boolean) => boolean.hash(state),
            // Handle other variants similarly
            _ => unimplemented!("Hashing not implemented for this variant"),
        }
    }
}