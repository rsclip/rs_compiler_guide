# Tokens
To implement a lexer, we'll have to go through multiple steps. First, let's declare the different kinds of tokens we might encounter and want to parse. They would serve as the building blocks of the language before it can be understood.

In `token.rs`:

```rust
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
    Plus,      // +
    Minus,     // -
    Star,      // *
    Slash,     // /
    Percent,   // %
    Caret,     // ^
    Bang,      // !
    Colon,     // :
    Semicolon, // ;
    Comma,     // ,
    Dot,       // .
    Equals,    // =
    Less,      // <
    Greater,   // >
    LParen,    // (
    RParen,    // )
    LBrace,    // {
    RBrace,    // }
    Quote,     // "

    // double-character tokens
    PlusEquals,    // +=
    MinusEquals,   // -=
    StarEquals,    // *=
    SlashEquals,   // /=
    PercentEquals, // %=
    CaretEquals,   // ^=
    BangEquals,    // !=
    LessEquals,    // <=
    GreaterEquals, // >=
    EqualsEquals,  // ==
    Arrow,         // ->

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
```

We split these into a few groups:
1. **Keywords**: These are the reserved keywords, e.g. for declaring functions
2. **Single-character tokens**: Self-explanatory.
3. **Multi-character tokens**: Tokens which are multiple (typically two) characters
4. **Datatypes**: The primitive datatypes like `int`
5. **End of File**: This token signifies the end of the token stream.

This is great, but it is important to think ahead. If we encounter an error during the lexing process, unless we keep track of the actual start & end positions (introducing complexity), we won't be able to show the user where the error is.

We will declare a `Token` and `Span` struct which will store both the kind of token it is, and where it spans in the source code:

```rust
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
```

It is important to note they are declared with `pub` to ensure they're accessible outside the module.

Let's also add a helper function which will convert a string to a keyword or identifier; this is because when lexing, we may be unsure if a string is a keyword or not. This will prioritise the keyword so they do not be overwritten by an identifier.

Additionally, we will create a constructor to convert a string to an actual `TokenKind` operator. It is cleaner to keep certain functionalities within this place instead of the lexer since it's more tidy.

```rust
impl TokenKind {
    /// Get operator from a string
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

	/// Get keyword from a string
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
```

This is tons of boilerplate code, but it's important we get this right; we'll be using them throughout the first two stages of the compilation process, until we construct an AST.