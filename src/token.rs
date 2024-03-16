//! Represents a token and their respective information.

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
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
    Plus,       // +
    Minus,      // -
    Star,       // *
    Slash,      // /
    Percent,    // %
    Caret,      // ^
    Bang,       // !
    Colon,      // :
    Semicolon,  // ;
    Comma,      // ,
    Dot,        // .
    Equals,     // =
    Less,       // <
    Greater,    // >
    OpenParen,  // (
    CloseParen, // )
    OpenBrace,  // {
    CloseBrace, // }
    Quote,      // "

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
    Identifier(String),
    IntegerLiteral(i64),
    FloatLiteral(f64),
    BooleanLiteral(bool),

    // Data types
    Int,
    Bool,

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
            _ => TokenKind::Identifier(keyword.to_string()),
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
            TokenKind::OpenParen => write!(f, "("),
            TokenKind::CloseParen => write!(f, ")"),
            TokenKind::OpenBrace => write!(f, "{{"),
            TokenKind::CloseBrace => write!(f, "}}"),
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
            TokenKind::Eof => write!(f, "EOF"),
            TokenKind::Identifier(id) => write!(f, "{}", id),
            TokenKind::IntegerLiteral(_) => write!(f, "IntegerLiteral"),
            TokenKind::FloatLiteral(_) => write!(f, "FloatLiteral"),
            TokenKind::BooleanLiteral(_) => write!(f, "BooleanLiteral"),
        }
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}
