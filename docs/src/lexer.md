# Implementing a Lexer
Now that we have our tokens declared, we can start implementing our lexer. The lexer will take in a string of source code and output a list of tokens. We'll use a simple state machine to do this, as mentioned in [Finite Automata](./finite_automata.md).

Keep in mind we will not be explicitly storing States and Transitions, but instead it'll be implemented implicitly in the code.

## What are Iterators?
Rust includes the concept of **Iterators**, which essentially allow you to iterate over some sort of collection (e.g. the source code characters) and perform some action on each item. This is a very powerful concept, and we'll use it to our advantage.

We will use the [`Iterator` trait](https://doc.rust-lang.org/std/iter/trait.Iterator.html) to apply the iterator concept to our lexer. As stated, our lexer will be a Deterministic Finite Automaton (DFA). We will use the `Iterator` trait to iterate over the characters of the source code, and the state machine will consume these characters and transition to the next state.

## The Lexer
Let's start by creating a new file called `lexer.rs` in the `src` directory. This file will contain our lexer implementation. We'll start by defining the `Lexer` struct and implementing the `Iterator` trait for it.

```rust,ignore
use anyhow::{anyhow, Error, Result};

use crate::errors::LangError;
use crate::token::{Span, Token, TokenKind};

/// A lexer iterator.
pub struct Lexer<'a> {
    src: &'a str,
    pos: usize,
}
```

The lexer will contain a reference to the source code `src`, and a `pos` field to keep track of the current position in the source code. We would rather use `&'a str` instead of `String` to avoid unnecessary allocations; we won't be modifying the source code, so we don't need to own it.

> **Note**
> 
> Some helper functions (e.g. `advance`, `peek`) are not shown here, but they will be used in the implementation. They are simple and self-explanatory, and you can find them in the full source code.

## DFA Model
Before we implement the `Iterator` trait, we need to understand exactly how iteration will work here.

We will use a simple DFA model to implement our lexer. The DFA will have a set of states, and each state will have a set of transitions to other states. The transitions will be based on the current character being processed.

Consider:

- States: \\( Q = \\{q_{start}, q_{id}, q_{kw}, q_{num}, q_{op}, q_{whitespace}, q_{eof}\\} \\)  
- Alphabet: \\( \sum = \\{\text{Letters}, \text{Digits}, \text{Operators}, \text{Symbols}\\} \\)
    - \\(\text{Operators}\\) represent both single-char and multi-char operators.

Let's derive a transition function! We should define these for each state. We will exclude numbers and identifiers since they were explored in [Finite Automata](./finite_automata.md).

### Tokenizing operators
The major case we will need to handle is tokenizing operators which may or may not be multi-character.

Take `+` and `+=` as an example: the lexer should recognize `+` as an operator token, and `+=` as an operator token as well. The lexer should not tokenize `+` as a single `+` token and `=` as a separate token.

We can define the transition functions for the \\(q_{op}\\) state as follows:

$$\begin{align*} \delta(q_{op}, a) &= \begin{cases} q_{op}^{(1)} & \exists s_i \in \text{ 2-char Operators}: \text{a startswith } s_i \\\\ q_{op} & \exists s_i\in \text{ 1-char Operators}: \text{a startswith } s_i \\\\ \text{null} & \text{otherwise} \end{cases} \\\\ \delta(q_{op}^{(1)}, b) &= \begin{cases} q_{op} & ab \in \text{ 2-char Operators} \\\\ \text{null} & \text{otherwise} \end{cases} \\\\
\end{align*}$$

Essentially, this just means depending on the start of the operator, we will transition to a new state \\(q_{op}^{(1)}\\) (the state for multi-char operators) and then transition back to the \\(q_{op}\\) state if the next character is not part of a multi-char operator.

### Tokenizing comments and whitespace
We will also need to handle comments and whitespace. Typically, we would ignore them unless they hold some significance (e.g. a comment token).

When we transition into the \\(q_{comment}\\) state, we will simply consume until a newline is encountered. We will then transition back to the \\(q_{start}\\) state:

$$\delta(q_{comment}, a) = \begin{cases} q_{comment} & a \neq \text{newline} \\\\ q_{start} & a = \text{newline} \end{cases}$$

The \\(q_{whitespace}\\) state will be similar, but we will transition back to the \\(q_{start}\\) state on encountering a non-whitespace character:

$$\delta(q_{whitespace}, a) = \begin{cases} q_{whitespace} & a = \text{whitespace} \\\\ q_{start} & a \neq \text{whitespace} \end{cases}$$

### Tokening symbols
This is the easiest case to handle. We will simply transition to a final state when we encounter a symbol:

$$\delta(q_{symbol}, a) = \begin{cases} q_{symbol} & \text{if a is a symbol} \end{cases}$$

### Helping DFA realise its over
We will also need to handle the end of the file. We will transition to the \\(q_{eof}\\) state when we reach the end of the file:

$$\delta(q_{eof}, a) = \begin{cases} q_{eof} & \text{EOF} \end{cases}$$

It should simply never transition out of the state, as there are no more characters to process.

## Implementing the Iterator trait
To implement the lexer there is a single method we need to implement: `next`. This method will return the next token in the source code.

```rust,ignore
impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // ...
    }
}
```

At each iteration, we will consume some characters depending on the state, and upon reaching a final state, we will return the token. We will also need to handle the case where the lexer reaches the end of the source code.

First, we should skip any whitespace and comments since they cannot be tokenized into anything meaningful. It'll also be cleaner to move this to a separate `impl` block.

```rust,ignore
impl<'a> Lexer<'a> {
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_comments(&mut self) {
        if self.peek() == Some('/') && self.src[self.pos + 1..].chars().next() == Some('/') { // ridiculously slow :(
            while let Some(ch) = self.peek() {
                if ch == '\n' {
                    break;
                } else {
                    self.advance();
                }
            }
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        self.skip_comments();

        // ...
    }
}
```

Now after transitioning from \\(q_{start}\\) to potentially \\(q_{whitespace}\\) or \\(q_{comment}\\) and back to \\(q_{start}\\), we can start lexing the next token. Rust's `match` keyword is extremely powerful in this instance!

```rust,ignore
        if let Some(ch) = self.peek() {
            // we'll define our transition functions here
            let token = match ch {
                // Transition to q_{op} state
                '+' | '-' | '*' | '/' | '%' | '^' | '=' | '<' | '>' | '!' | '&' | '|' => {
                    self.lex_op()
                }
                // Transition to q_{num} state
                '0'..='9' => self.lex_number(),
                // Transition to q_{id} state. This also includes keywords.
                'a'..='z' | 'A'..='Z' | '_' => self.lex_word(),
                // Transition into symbols state
                '{' => Ok(self.lex_single_char(TokenKind::LBrace)),
                '}' => Ok(self.lex_single_char(TokenKind::RBrace)),
                '(' => Ok(self.lex_single_char(TokenKind::LParen)),
                ')' => Ok(self.lex_single_char(TokenKind::RParen)),
                ';' => Ok(self.lex_single_char(TokenKind::Semicolon)),
                ',' => Ok(self.lex_single_char(TokenKind::Comma)),
                '.' => Ok(self.lex_single_char(TokenKind::Dot)),
                ':' => Ok(self.lex_single_char(TokenKind::Colon)),
                _ => {
                    // return an error including the span
                    Err(anyhow!(LangError::UnexpectedCharacter(
                        ch.to_string(),
                        Span {
                            start: self.pos,
                            end: self.pos + ch.len_utf8()
                        }
                    )))
                }
            };

            Some(token)
        } else {
            None
        }
```

One important note is that whilst it returns `Option<Self::Item>` -- or `Option<Result<Token>>` more specifically -- upon encountering an error we will attempt to recover instead of declaring there is no token there. This is a common practice in lexing and parsing so we can catch multiple errors instead of stopping early.

Next, there's a few transition functions we need to define.

### Transitioning to the \\(q_{op}\\) state (parsing operators)
Recall the transition functions for the \\(q_{op}\\) state:

$$\begin{align*} \delta(q_{op}, a) &= \begin{cases} q_{op}^{(1)} & \exists s_i \in \text{ 2-char Operators}: \text{a startswith } s_i \\\\ q_{op} & \exists s_i\in \text{ 1-char Operators}: \text{a startswith } s_i \\\\ \text{null} & \text{otherwise} \end{cases} \\\\ \delta(q_{op}^{(1)}, b) &= \begin{cases} q_{op} & ab \in \text{ 2-char Operators} \\\\ \text{null} & \text{otherwise} \end{cases} \\\\
\end{align*}$$

Whilst it seems complex, we can implement it in a single function. The steps are simply:
0. Assume that the current character is an operator
1. Keep track of the start index of the operator
2. Check if the next character forms a multi-char operator
    - **If so**, consume the next character and return the multi-char operator token
    - **If not**, return the single-char operator token

Let's implement this in the main `impl` block:

```rust,ignore
    fn lex_op(&mut self) -> Result<Token> {
        // 1. Keep track of the start index of the operator
        let start = self.pos;

        // 2. Check if the next character forms a multi-char operator
        self.advance();
        let mut end = self.pos;

        if let Some(ch) = self.peek() {
            match ch {
                // if so, consume the next character and update end
                '+' | '-' | '*' | '/' | '%' | '^' | '=' | '<' | '>' | '!' | '&' | '|' => {
                    self.advance();
                    end = self.pos;
                }
                // if not, the end is the same as the start
                _ => {}
            }
        } else {
            // Make sure we dont go out of bounds
            return Err(anyhow!(LangError::UnexpectedEOF(Span { start, end })));
        }

        Ok(Token {
            kind: TokenKind::operator_from(&self.src[start..end]),
            span: Span { start, end },
        })
    }
```

Keep in mind we are using the `operator_from` method on `TokenKind` to convert the operator string to a `TokenKind`. We implemented this in the [Tokens](./tokens.md) section.

### Transitioning to the \\(q_{num}\\) state (parsing numbers)
Recall the transition functions for the \\(q_{num}\\) state in the [Finite Automata](./finite_automata.md) section:
$$\delta(q_{num}, a) = \begin{cases}
q_{int} &q = q_{start} \ \land \ a \in \text{Digits} \\\\
q_{decimal} &q = q_{intPart} \ \land \ a \in \text{Decimal Point} \\\\
q_{frac} &q = q_{decimalPoint} \ \land \ a \in \text{DIGITS} \\\\
\text{null} &\text{otherwise}
\end{cases}$$

We'll use this for parsing both integers and floats (in this case, they will be parsed as integers unless a `.` is present).

First, let's get the entire span of the literal, including whether or not it contains a decimal point:

```rust,ignore
    fn lex_number(&mut self) -> Result<Token> {
        let start = self.pos;
        let mut has_decimal = false;

        while let Some(ch) = self.peek() {
            // if it's a number, we keep going
            if ch.is_digit(10) {
                self.advance();
            } else if ch == '.' {
                // we keep going still, but we need to keep track of whether we've seen a decimal point
                if has_decimal {
                    self.advance();
                    let val = &self.src[self.pos - 1..self.pos];

                    // advance until whitespace
                    while let Some(ch) = self.peek() {
                        if ch.is_whitespace() {
                            break;
                        } else {
                            self.advance();
                        }
                    }

                    return Err(anyhow!(LangError::UnexpectedCharacter(
                        val.to_string(),
                        Span {
                            start,
                            end: self.pos
                        }
                    )));
                } else {
                    has_decimal = true;
                    self.advance();
                }
            } else {
                break;
            }
        }
        let end = self.pos;

        // ...
    }
```

Here, we continually advance across the number until we reach a non-digit character. If we encounter a decimal point `.`, we handle it depending on whether we've seen one before:
- If we haven't, we set `has_decimal` to `true` and continue
- If we have, we advance until the end of the entire "number" and return an error

Next, we should check whether the number is a float or not (determined by `has_decimal`), and convert it into the respective `Token`:

```rust,ignore
        if has_decimal {
            // attempt to parse as a float, otherwise return an error
            match value.parse() {
                Ok(f) => Ok(Token {
                    kind: TokenKind::FloatLiteral(f),
                    span: Span { start, end },
                }),
                Err(_) => Err(anyhow!(LangError::InvalidLiteral(
                    value.to_string(),
                    Span { start, end }
                ))),
            }
        } else {
            // attempt to parse as an integer, otherwise return an error
            match value.parse() {
                Ok(i) => Ok(Token {
                    kind: TokenKind::IntLiteral(i),
                    span: Span { start, end },
                }),
                Err(_) => Err(anyhow!(LangError::InvalidLiteral(
                    value.to_string(),
                    Span { start, end }
                ))),
            }
        }
```

### Transitioning to the \\(q_{kw} \bigcup q_{id}\\) state (parsing identifiers)
We're going to group these two states together, since we may not be certain if the identifier is a keyword or not. We'll check this after we've determined the entire span of the word.

Again, let's recall the transition functions for the \\(q_{id}\\) state in the [Finite Automata](./finite_automata.md) section (which we will rename to \\(q_{word}\\) for clarity):

We'll remain in the state \\(q_{word}\\) for any subsequent letters or digits, and transition to the state \\(q_{word}^{(1)}\\) if we encounter a whitespace character. From there, we'll transition into a final state \\(q_{kw}\\) or \\(q_{id}\\) depending on whether the word is a keyword or not.

$$\begin{align*} \delta(q_{word}, a) &= \begin{cases} q_{word} & a \in \text{Letters} \ \bigcup \ \text{Digits} \\\\ q_{word}^{(1)} & a \in \text{Whitespace} \end{cases} \\\\ \delta(q_{word}^{(1)}, b) &= \begin{cases} q_{kw} & ab \in \text{Keywords} \\\\ q_{id} & \text{otherwise} \end{cases} \\\\
\end{align*}$$

Let's implement this in our Lexer! Here's an overview of the process:

0. Assumption: the current character is a letter or underscore (start of word)
1. Keep track of the start index of the word
2. Keep advancing until we reach a non-letter or non-digit character
3. Check if the word is a keyword
    - **If so**, return the keyword token
    - **If not**, return the identifier token

```rust,ignore
    fn lex_word(&mut self) -> Result<Token> {
        // 1. Keep track of the start index of the word
        let start = self.pos;

        // 2. Keep advancing until we reach a non-letter or non-digit character
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }
        let end = self.pos;

        let value = &self.src[start..end];

        // Generate the respective token!
        Ok(Token {
            kind: TokenKind::keyword_from(value),
            span: Span { start, end },
        })
    }
```

Again, we're using the `keyword_from` method on `TokenKind` to convert the word to a `TokenKind`, covered in the [Tokens](./tokens.md) section.

### Transitioning to the \\(q_{symbol}\\) state (parsing symbols)
We leave the easiest for last. We'll simply transition to a final state when we encounter a symbol, since they're guaranteed to be single-characters and final states.

Recall the transition function:

$$\delta(q_{symbol}, a) = \begin{cases} q_{symbol} & \text{if a is a symbol} \end{cases}$$

We can quickly implement this in our Lexer; it will advance past the single-character and return the respective `Token`:

```rust,ignore
    fn lex_single_char(&mut self, kind: TokenKind) -> Token {
        let start = self.pos;
        self.advance();
        let end = self.pos;

        Token {
            kind,
            span: Span { start, end },
        }
    }
```

## Helper methods
If you noticed, the iterator returns `Option<Result<Token>>` which is pretty ugly albeit necessary if we want errors. This also means that we can't use the `collect` method on the iterator, which is a bit of a pain.

It would help a lot to have a method that consumes an iterator, returning a stream of tokens and the errors encountered. Let's create a function `consume_lexer(lexer: Lever) -> (Vec<Token>, Vec<Error>)` to do this.

```rust,ignore
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
```

Very simple and straightforward and makes life a lot easier.

# Resources
- [Iterator trait](https://doc.rust-lang.org/std/iter/trait.Iterator.html): The official documentation for the `Iterator` trait.
- [Lifetimes in Rust](https://doc.rust-lang.org/rust-by-example/scope/lifetime.html)
- [Finite Automata](./finite_automata.md) in this guide!
- [Rust's Strings](https://medium.com/@tarungudipalli/exploring-rusts-string-a-comprehensive-guide-with-examples-25f398ade356): A comprehensive guide to Rust's strings.