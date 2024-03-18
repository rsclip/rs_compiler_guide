# Lexical Analysis
## Introduction
The primary purpose of lexical analysis is to simplify the subsequent phases of the compiler (the parser!) by providing an unstructured categorized representation of the source code as a stream of **tokens**. This can be done in a few ways.

Regular expressions are patterns used to define the syntax of tokens in a programming language, used by tokenizers/lexers to recognize tokens. This guide will not use regular expressions. This is mainly because it's inefficient (lots of backtracking), is very deterministic, and is difficult to debug and control. I don't like regex either.

Finite automata, including deterministic finite automata (DFAs) and non-deterministic finite automata (NFAs), serve as theoretical models for defining and implementing lexical analyzers.
### Finite Automata
Finite Automata (FAs) are theoretical models we use to implement lexical analyzers (aka lexers, tokenizers). These models define how we transition between states (how we move between the current symbol and the next), allowing the analyzer to recognize tokens by traversing the automaton.

There are two groups we will describe below:
##### Deterministic Finite Automata
DFAs are state machines where, given an input symbol, there is exactly **one next state**. Each token type is represented as a state, and transitions between states are determined by the source code. While the notation may seem scary, re-reading it will help you understand it's just fancy-speak!

Before we recognize some identifiers, there are of-course rules:
- The identifier must start with a letter
	- Typically underscores `_` are allowed but for simplicity, letters
- The identifier does not contain whitespace
- The identifier may contain numbers *after* the first symbol

With these rules, we can construct the transition function, as below.

Consider a simple DFA for recognizing identifiers in a programming language:
- States: \\(Q = \{q_{start}, q_{identifier}\}\\)
- Alphabet: \\(\sum = \\{\text{Letters}, \text{Digits}\\}\\)
- Transition function:
  $$\delta(q, a) = \begin{cases}
q_{identifier} &\text{if} \ q = q_{start} \ \text{and} \ a \in \text{Letters} \\\\
q_{identifier} &\text{if} \ q = q_{identifier} \ \text{and} \ a \in \text{Letters} \ \bigcup \ \text{Digits} \\\\
\text{undefined} &\text{otherwise}
\end{cases}$$
- Input: `"variable123 ..."`

By DFA, we will follow this process:
1. Start in \\(q_{start}\\)
2. Transitions to \\(q_{identifier}\\) on encountering a letter `v`
	1. Stays in \\(q_{identifier}\\) for subsequent letters/digits `ariable123`
3. Reaches end in \\(q_{identifier}\\) at whitespace ` `
4. Recognises `variable123` as an identifier

The transition function $\delta$ determines how the state should transition, depending on the given conditions.
##### Non-Deterministic Finite Automata
NFAs are state machines where, given an input symbol, there can be **multiple possible next states** (as opposed to exactly one in DFAs). We would use these in lexical analysis as a *theoretical basis*, before using DFAs.

Before we recognize some floating-point literals, we must identify the rules:
- Literals must contain digits, and no letters
- Literals may contain a single decimal point `.`
- Literals are defined in two parts: the integer part and the fractional part
	- The integer part represents the whole number section of the literal (i.e. `3`)
	- The fractional part represents the decimal section of the literal (ie. `.14`)

And thus, we can construct a transition function as below!

Consider an NFA for recognizing floating-point literals:
- States: \\(Q = \{q_{start}, q{int}, q_{decimal}, q_{frac}\}\\)
	- \\(q_{int}\\), \\(q_{frac}\\) are the integer and fractional parts respectively
	- \\(q_{decimal}\\) is the presence of a decimal point
- Alphabet: \\(\sum = \\{\text{Digits}, \text{Decimal Point}\\}\\)
	- Keep in mind "Alphabet" is simply the set of parse-able symbols
- Transition function:
  $$\delta(q, a) = \begin{cases}
q_{int} \ \text{if} &q = q_{start} \ \text{and} \ a \in \text{Digits} \\\\
q_{decimal} \ \text{if} &q = q_{intPart} \ \text{and} \ a \in \text{Decimal Point} \\\\
q_{frac} \ \text{if} &q = q_{decimalPoint} \ \text{and} \ a \in \text{DIGITS} \\\\
\text{undefined} &\text{otherwise}
\end{cases}$$
- Input: `"3.14"`

By NFA, we will follow this process:
1. Start in \\(q_{start}\\)
2. Transition to \\(q_{int}\\) for `3`
3. Transition to \\(q_{decimal}\\) for `.`
4. Transition to \\(q_{frac}\\) for `1`
	1. Remain in \\(q_{frac}\\) for `4`
5. End or whitespace is reached
6. Recognises `3.14` as a floating-point literal: `3` for integer part, `14` for fractional part.

Again, the transition function $\delta$ determines how the state should transition, **and** to which state it should transition to, depending on the given conditions.
##### Which Finite Automata do we use?
The clear difference between the two is the DFAs are much more simplistic and efficient, with clear and optimal state transitions. NFAs, however, allow for multiple possible states and are much more flexible.

We will choose to use Deterministic Finite Automata for its simplicity! We will not have an explicit handling on non-determinism, meaning we will not branch or have multiple possible transitions based on an input character. In more simple terms: at each character symbol, we will not consider multiple state possibilities to transition into, but instead exactly one next state.

In a nutshell, we chose to not complicate life more than it already is.
## Implementing a Lexer
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

This is great, but it is important to think ahead. If we encounter an error during the lexing process, unless we keep track of the actual start & end positions (introducing complexity), we won't be able to show the user where the error is. We will declare a `Token` and `Span` struct which will store both the kind of token it is, and where it spans in the source code:

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
