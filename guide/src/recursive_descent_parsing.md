# Recursive Descent Parsing
They make these names sound scary for no reason.

## Notable Definitions
Refer to these when needed, but don't memorize them. They are here for reference.

- **Top-down parsing**: A parsing technique that constructs a parse tree from the top and the input is read from left to right.
- **LL(k) parser**: A parser that reads input from left to right, constructs a leftmost derivation of the input, and uses k tokens of lookahead to make parsing decisions.
- **Parse tree**: A tree that represents the syntactic structure of a language construct according to a formal grammar.
- **Abstract Syntax Tree (AST)**: A tree that represents the abstract syntactic structure of a language construct according to a formal grammar.
- **Lookahead**: The number of tokens the parser looks ahead to make parsing decisions.
- **Predictive parsing**: A parsing technique that uses a lookahead of 1 to make parsing decisions.
- **Backtracking**: A technique used in recursive descent parsing to handle ambiguous grammars.
- **Left recursion**: A grammar is left-recursive if it has a non-terminal A that can directly or indirectly derive a string that starts with A itself.
- **Left factoring**: A technique used to eliminate common prefixes in a grammar.
- **Mutually recursive**: A set of procedures that call each other in a circular manner.

### LL(...) Parsing
LL(k) and LL(1) parsing may be confusing at first. Here's a simple explanation:

"LL" stands for "Left-to-right, Leftmost derivation". This means that the parser reads the input from left to right and constructs a leftmost derivation of the input. The "k" represents the number of tokens of lookahead the parser uses to make parsing decisions.

LL(1) parsers are a subset of LL(k) parsers, where k is 1. This means that an LL(1) parser uses a lookahead of 1 to make parsing decisions.

LL(1) is more memory-efficient (particularly a point of concern with RDP) and simpler, but it can only parse a subset of grammars. LL(k) parsers can parse a wider range of grammars, but they are more complex and require more memory.

## Introduction to Recursive Descent Parsing (RDP)
Recursive Descent Parsing is a **top-down parsing technique** that constructs a parse tree (AST) from the top and the input is read from left to right.

In simple terms, a recursive descent parser is a set of mutually recursive procedures that correspond to the grammar rules of the language. Each procedure usually corresponds to a non-terminal symbol (on the left) in the grammar. The parser starts with the start symbol \\(S\\) of the grammar \\(G\\) and uses these procedures to recursively parse the input.

### Evaluation
RDP is a simple and very efficient parsing technique; beyond being easy to implement, it's easy to debug and understand. It's also very flexible and can be used to parse a wide range of grammars, including ambiguous ones (although you should avoid them if possible).

Given its recursive nature, it can be pretty memory-intensive (since it uses the call stack). This worsens if adapted to LL(k) parsing.

## How it works
The parser is a set of **mutually recursive** procedures, in which their names correspond to the non-terminal symbols of the grammar. Each procedure corresponds to a grammar rule and is responsible for parsing the input according to that rule.

The parser starts with the start symbol \\(S\\) of the grammar and uses these procedures to recursively parse the input.

### Example
For example, given the following simple grammar:
$$
\begin{align*}
S &\rightarrow A \\\\
A &\rightarrow b
\end{align*}
$$
The parser would have the following procedures:
```rust,ignore
/// Name corresponds with the left-hand side for S -> aAb
fn parse_S() {
    parse_A();
}

/// Name corresponds with the left-hand side for A -> b
fn parse_A() {
    match next_token() {
        Token::B => consume(),
        _ => panic!("Unexpected token"),
    }
}
```

### Handling Ambiguity
RDP can handle ambiguous grammars by using **backtracking**. When the parser encounters a choice between two or more alternatives, it tries each alternative in turn. If one of the alternatives fails, the parser backtracks to the point where the choice was made and tries the next alternative.

This is particularly useful in scenarios such as parsing expressions, where the parser needs to decide between different operators.

<div class="warning">
    <p><strong>Warning</strong>: Backtracking can be very inefficient and can lead to exponential time complexity in the worst case. It's best to avoid ambiguous grammars if possible.</p>
</div>

### Left Recursion in RDP
RDP cannot handle left-recursive grammars directly. Left recursion is when a non-terminal can directly or indirectly derive a string that starts with itself. For example, the grammar \\(A \rightarrow Aa | b\\) is left-recursive because it can derive strings that start with \\(A\\).

To understand why, consider the following example:
$$
\begin{align*}
A &\rightarrow Aa \ | \ b \\\\
A &\rightarrow b
\end{align*}
$$

If we try to write a recursive descent parser for this grammar, we might end up with an infinite loop. This is because the parser will keep calling the \\(A\\) procedure, which will keep calling itself.

### Left Factoring
To handle left recursion, we can use a technique called **left factoring**. Left factoring is a technique used to eliminate common prefixes in a grammar. It's a simple process that involves creating a new non-terminal for the common prefixes.

Consider the previous example:
$$
\begin{align*}
A &\rightarrow Aa \ | \ b \\\\
A &\rightarrow b
\end{align*}
$$

We can factor out the common prefix \\(A\\) to get:
$$
\begin{align*}
A &\rightarrow bA' \\\\
A' &\rightarrow aA' \ | \ \epsilon
\end{align*}
$$

<!-- Explanation -->
The new non-terminal \\(A'\\) represents the remaining part of the original \\(A\\) rule. The \\(\epsilon\\) represents the empty string, which is used to terminate the recursion.

#### Example
Since this concept was hard for me to understand, I'll provide an example to help you understand it better.

Consider the previous example again:
$$
\begin{align*}
A &\rightarrow bA' \\\\
A' &\rightarrow aA' \ | \ \epsilon
\end{align*}
$$

The parser would have the following procedures:
```rust,ignore
/// Name corresponds with the left-hand side for A -> bA'
fn parse_A() {
    match next_token() {
        Token::B => {
            consume();
            // Call the A' procedure instead of itself
            parse_A_prime();
        }
        _ => panic!("Unexpected token"),
    }
}

/// Name corresponds with the left-hand side for A' -> aA' | ε
fn parse_A_prime() {
    match next_token() {
        Token::A => {
            consume();
            // Call the A' procedure again
            parse_A_prime();
        }
        _ => {}
    }
}
```

The loop is broken by the \\(\epsilon\\) in the \\(A'\\) rule, which stops the recursion:
$$ A' \rightarrow aA' \ | \ \epsilon $$

Keep in mind that this is a simple example. Left factoring can be more complex for larger grammars. You may unknowingly introduce left recursion when factoring out common prefixes, so be careful.

# Resources
- [Recursive Descent Parsing](https://en.wikipedia.org/wiki/Recursive_descent_parser) on Wikipedia
- [RDP Visualizer](https://maeyler.github.io/Automata-2018/cfg/Bilal_RecursiveDescentParser.html) - A visualizer for recursive descent parsers