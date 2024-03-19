# Context-free Grammars
Here, we'll discuss the concept of Context-free Grammars (CFGs) and their significance in compiler design. We'll also cover the formal definition of CFGs, metasyntax notations, and designing grammars.

This section is important, as it provides the grammar our parser will be using.

## Introduction to Context-free Grammars
Context-free grammars (CFGs) are a formal way to describe the syntax of programming languages. They are used to define the structure of a language by specifying the rules for constructing valid programs. CFGs are an essential part of the parsing process in compilers, as they provide a systematic way to recognize and analyze the syntactic structure of source code.

It's similar to how a natural language has a grammar, which defines the rules for constructing valid sentences. In the same way, a programming language has a grammar that defines the rules for constructing valid programs.

You would identify that `He. had a hat` is not a valid sentence in English, and in the same way, `int x = 5 +;` is not a valid statement in C. CFGs help us define these rules and constraints.

### What do you mean by "context-free"?
In the context of grammars, "context-free" means that the production of a non-terminal symbol can occur without regard to the surrounding context. This property simplifies parsing, as it allows the recognition of language constructs solely based on the local structure, without considering the broader context in which they appear.

This is a mouthful, but in simple terms, it means that the structure of a construct (i.e. a variable declaration) can be recognized based on its own local structure (i.e. only the variable declaration statement and its components), without needing to consider the broader context (i.e. the outer scope or function).

### Example
Consider a context-free grammar for simple arithemetic expressions (supporting addition).

The product rule for addition may be defined as:
```
// `expr` is defined as `term` followed by `+` followed by `expr`
expr -> expr + term
        | term          // or just `term`
```

The expression can be composed of another expression *inside* the `expr` rule, and the `expr` rule can be used to define the `term` rule. This is the "context-free" property of the grammar. We call this **replacement**.

## Formal Definition of Context-free Grammars
Indeed we need a formal definition of CFGs. It's a bit complex, but it's important to understand the formal definition of CFGs to understand how they are used in compiler design.

CFGs are defined as a 4-tuple \\(G = (N, \sum, P, S)\\), where:
- \\(N\\) is a finite set of non-terminal symbols.
    - "non-terminal" symbols are symbols that can be replaced by a sequence of other symbols. Essentially, the parser can expand these symbols into other symbols.
    - For example, the non-terminal symbol `expr` in the previous example can be expanded into `expr + term` or just `term`.
- \\(\sum\\) is a finite set of terminal symbols, **disjoint** from \\(N\\).
    - "terminal" symbols are symbols that cannot be replaced by other symbols. They are the smallest units of the language.
    - For example, the `+` symbol in the previous example is a terminal symbol.
- \\(P\\) is a finite set of production rules, where each rule is of the form \\(A \rightarrow \alpha\\), where \\(A \in N\\) and \\(\alpha \in (N \cup \sum)^*\\) (a string of symbols from \\((N \cup \sum)^*\\)).
    - Each production rule defines how a non-terminal symbol can be replaced by a sequence of other symbols.
    - For example, the production rule `expr -> expr + term` in the previous example.
    - \\((N \cup \sum)^*\\) denotes the set of all strings of symbols that can be formed from the union of \\(N\\) and \\(\sum\\).
- \\(S\\) is the start symbol, which is a special non-terminal symbol that represents the entire language.
    - It is the symbol from which the derivation of the entire language begins.

Some important points to note:
- Non-terminal symbols represents syntactic categories in the language
- Terminal symbols represent the primary elements of the language
    - Identifiers, keywords, operators, etc.
- Production rules define how non-terminal symbols can be replaced by sequences of other symbols
- The start symbol represents the entire language and is used as the initial symbol for the derivation process

### Example
Let's consider a simple CFG for a very basic arithmetic expression language. The language supports the basic operations of numbers.

Given the CFG \\(G = (N, \sum, P, S)\\), where:
- \\(N = \{\text{Expr}, \text{Term}, \text{Factor}\}\\) are non-terminal symbols
- \\(\sum = \{+, *, (, ), \text{num}\}\\) are terminal symbols
- \\(P\\) is a set of production rules
- \\(S = \text{Expr}\\) is the start symbol

The production rules for the CFG are:
$$\begin{align*}
\text{Expr} &\rightarrow \text{Expr} + \text{Term} \\\\
\text{Expr} &\rightarrow \text{Expr} - \text{Term} \\\\
\text{Expr} &\rightarrow \text{Term} \\\\
\text{Term} &\rightarrow \text{Term} * \text{Factor} \\\\
\text{Term} &\rightarrow \text{Term} \div \text{Factor} \\\\
\text{Term} &\rightarrow \text{Factor} \\\\
\text{Factor} &\rightarrow (\text{Expr}) \\\\
\text{Factor} &\rightarrow \text{num}
\end{align*}$$

Let's parse the arithmetic expression \\(3 \times (4 + 5)\\) using \\(G\\). We'll do it step by step, with indentations to demonstrate the process.

Let the token stream be `"3", "*", "(", "4", "+", "5", ")"`.

- \\(\text{Expr} \Rightarrow \text{Term}\\)
   - \\(\text{Term} \Rightarrow \text{Term} * \text{Factor}\\)
     - \\(\text{Term} \Rightarrow \text{Factor}\\)
       - \\(\text{Factor} \Rightarrow \text{num}\\) applies at \\(3\\)
     - \\(\text{Factor} \Rightarrow (\text{Expr})\\) applies at \\((4 + 5)\\)
       \\(\text{Expr} \Rightarrow \text{Expr} + \text{Term}\\)
         - \\(\text{Expr} \Rightarrow \text{Term}\\)
           - \\(\text{Term} \Rightarrow \text{Factor}\\)
             - \\(\text{Factor} \Rightarrow \text{num}\\) applies at \\(4\\)
         - \\(\text{Term} \Rightarrow \text{Factor}\\)
           - \\(\text{Factor} \Rightarrow \text{num}\\) applies at \\(5\\)
   - \\(\text{Factor} \Rightarrow \text{num}\\) applies at \\(3\\)

The derivation process starts with the start symbol \\(\text{Expr}\\) and proceeds by applying production rules to non-terminal symbols until only terminal symbols remain. This process is called **derivation**.

This would yield the parse tree for the expression \\(3 \times (4 + 5)\\):
   
```
   *
  / \
 3   +
    / \
   4   5
```

Notice how operator precedence is implicitly defined by the grammar. The `*` operator has a higher precedence than the `+` operator, and the parentheses are used to override the precedence.

In compilers, you may either implement operator precedence within the grammar, or during parsing. We'll discuss this in the next section.

## Metasyntax Notations
Metasyntax notations are used to define the syntax of context-free grammars.

### BNF
[Backus-Naur Form (BNF)](https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form) is a notation used to describe the syntax of programming languages (and other formal languages). It's commonly used to define CFGs, as it provides a concise and readable way to express the production rules of a grammar.

In BNF notation, **productions** are used to define rules for constructing valid strings in a language. Each production consists of a **non-terminal symbol** on the left-hand side, followed by the `::=` symbol, and a sequence of **terminal** and/or **non-terminal symbols** on the right-hand side.

For example, the BNF production rule for dates may look like this:
```bnf
<date>   ::= <month> "/"" <day>
<year>   ::= <digit> <digit> <digit> <digit>
<month>  ::= 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12
```

- `<non-terminal>` represents a syntactic category or placeholder symbol
- `::=` is the "is defined as" symbol, otherwise known as the "production" symbol
- `<terminal>` represents a literal symbol or token, otherwise represented as `<expression>`

You may notice some semantic features, such as the pipe `|` denoting "or" and the double quotes `""` denoting a literal symbol. There are other notations, listed below:
- `[]` denotes optionality
   - e.g. `<date> ::= <month> "/" <day> ["/" <year>]`
- `()` denotes grouping
   - e.g. `<expression> ::= "(" <expression> ")" | <term>`
- `*` denotes repetition (0 or more times)
   - e.g. `<identifier> ::= <letter> <letter>*`


### EBNF
[Extended Backus-Naurs Form (EBNF) grammar](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form) is an extension of BNF that adds additional syntactic features to make the notation more expressive and concise. It's commonly used to define the syntax of programming languages and other formal languages.

Here are some of the differences:
- EBNF denotes optional elements by `{}` instead of `[]`
- EBNF extends repetition by introducing the use of `[]` to represent optional repetition, and `+` to represent one or more repetitions
   - e.g. `<block> ::= "{" <statement>* "}"`
   
Typically, you'd use EBNF over BNF as it's more concise and expressive.

As a side-note, you don't need to declare symbols with `<` and `>` in EBNF, but it's a common practice to do so.

## Designing Grammars
   <!-- - Discuss strategies for designing grammars:
     - Top-down vs. bottom-up approach
     - Language features and constraints
     - Hierarchical structure
   - Provide guidelines for creating clear, unambiguous, and concise grammars. -->

There's different strategies for designing grammars for your language.

### Top-down vs. Bottom-up approach
The top-down approach starts with the start symbol \\(S\\) and tries to derive the input string by applying production rules in a leftmost derivation. In simple terms, it starts from the top of the parse tree (i.e. programs, functions) and works its way down.

The bottom-up approach starts with the input string and tries to apply production rules in a rightmost derivation to reach the start symbol \\(S\\). In simple terms, it starts from the bottom of the parse tree (i.e. from tokens, expressions) and works its way up.

### Quick guidelines
When designing grammars, it's important to maximise the usage of the following:
- **Clarity**: The grammar should be clear and easy to understand
- **Consistent naming**: Use consistent naming conventions for non-terminal symbols
- **Unambiguous**: The grammar should be unambiguous, meaning that each string in the language has exactly one valid parse tree
- **Conciseness**: Do not make it overly complex. Keep it simple and concise
- **Modularize**: Similar to programming, modularize your grammar. Break it down into smaller, more manageable parts
- **Document**: Annotate with comments and documentation to explain the purpose of each production rule

## Let's design a grammar!
First let's refresh our memory of the language we're designing a grammar for. Read through [Lexical Analysis - Our Language](./lexical_analysis.md#our-language) to understand the language we're designing a grammar for. Let's see the example code we wrote:
```rust
fn main() -> int {
    let x: int = 5;
    let y: int = 3;
    let z: int = x + y;
    if (z > 5) {            // Branching!
        return z;           // Return z
    } else {
        return 0;           // Return 0
    }
}
```

We can either work top-down, defining the program structure first, or bottom-up, defining the smallest elements first. We'll start with the top-down approach.

We will also be using **EBNF** to define our grammar. Feel free to use a different notation and/or a bottom-up approach.

### High-level constructs
We can start by defining the high-level constructs of the language, such as the program, and the main components of the program. We would like it to be function-based (potentially including structs, enums, etc.).

Let's consider a program to have multiple items, which may be a function, a struct, or an enum. For simplicity, we'll only consider functions for now.

```ebnf
# A program is a sequence of 0 or more items
program           ::= item*

# An item is a function declaration
item              ::= function_decl
```

Now we can define the function declaration. Here's what the declaration looks like:
```rust
fn function_name(param1: type1, param2: type2) -> return_type { ... }
```

It consists of the `fn` keyword, the function name, a list of parameters, and the return type. We'll also include the function body, which we call a "block".

```ebnf
# Create the function declaration. There are optional parameter lists too.
function_decl     ::= "fn" IDENTIFIER "(" {parameter_list} ")" "->" type block

# A parameter list is a sequence of 0 or more parameters
parameter_list    ::= parameter ("," parameter)*

# A parameter is an identifier followed by a type
parameter         ::= IDENTIFIER ":" type
```

It's important to keep track of what you have not defined so far. Here's a list of things we haven't defined:
- `type`
- `block`
- `IDENTIFIER`

### Types
To get parameters and names out of the way, let's create production rules for types.

We don't need to worry about defining the specifics of how an identifier is structured for example, since the lexer has already handled it for us. We can simply refer to it as `IDENTIFIER`.

Types can be simple, like `int` and `bool`, which we call **primitive types**. They can also be more complex, like `struct` and `enum`, which we call **compound types**. For now, we'll only consider primitive types.

```ebnf
type              ::= primitive_type

primitive_type    ::= "int" | "float" | "bool"
```

**Undefined**:
- `block`

### Blocks and Statements
A block is a sequence of statements enclosed in curly braces. A statement can be an expression, a variable declaration, or a control flow statement.

This is straightforward to define:
```ebnf
block             ::= "{" statement* "}"

statement         ::= expression
                    | variable_decl ";"
                    | flow_statement
```

Notice expressions don't end with a semicolon, but variable declarations do.

Expressions are a bit more complex, as they can be composed of other expressions. We'll define them later. Let's focus on variable declarations and control flow statements.

A variable declaration must have a type and an expression. For safety (and to avoid null), we'll require an expression to be assigned to the variable.
```rust
let x: int = 5;
```

```ebnf
variable_decl     ::= "let" IDENTIFIER ":" type "=" expression
```

Control flow statements would consist of a few tokens: `if`, `else`, `return`. This is a bit tricky to design, as it can be recursive. Let's design it!

Here's what a control flow statement looks like:
```rust
if (condition) {
    // if block
} else if (condition) {
    // else if block
} else {
    // else block
}
```

As you see, the `else` token is condition, as is the `else if` token. We can define the `else if` block as a control flow statement, and the `else` block as a block.

```ebnf
flow_statement    ::= "if" expression block {"else" block}
```

We don't know if `expression` would yield a boolean value, but we'll leave that to the semantic analysis phase of the compilation process. We only care about understanding the structure of the language.

One big flaw with this production rule is that it doesn't support an arbritary number of `else if` statements. Let's modify it to support that.

```ebnf
flow_statement    ::= "if" expression block {"else" block | "else" flow_statement}
```
Now, the `else` token can be a block or another `if` statement. We can repeat it as many times as we want.

**Undefined**:
- `expression`

### Expressions
Here's the tough part. Expressions can be composed of other expressions, and can be quite complex. There's various things to support such as binary, unary, and ternary operations, function calls, and more. Similar to the previous expression example, we'll modularize this into smaller parts.

An expression can be either:
- A primary expression (i.e. a literal, a variable, or a function call)
- A unary expression (i.e. a negation, a dereference)
- A binary expression (i.e. an addition, a multiplication)

We call then "unary" and "binary" expressions because they operate on one or two operands, respectively. While we're at it, we'll also define primary expression and literals.

```ebnf
expression        ::= primary_expression
                    | unary_expression
                    | binary_expression

primary_expression ::= literal
                    | IDENTIFIER
                    | "(" expression ")"
                    | function_call

literal           ::= INT | FLOAT | BOOLEAN
```

That leaves us with unary and binary expressions.

Unary expressions consist of a unary operator and an expression. For example, `-4` or `!true`.
```ebnf
unary_expression  ::= "-" expression | "!" expression
```

Binary expressions consist of two expressions and a binary operator. For example, `4 + 5` or `true && false`.
```ebnf
binary_expression ::= expression OPERATOR expression
```

And finally, our last undefined symbol is `function_call`. A function call consists of a function name and a list of arguments.
```ebnf
function_call     ::= IDENTIFIER "(" {expression} ")"
```

### End result
And that's it! We've designed a grammar for our language. Here's the complete grammar so far:
```ebnf
program           ::= item*

item              ::= function_decl
                    # | struct_decl
                    # | enum_decl
                    # | impl_block

function_decl     ::= "fn" IDENTIFIER "(" {parameter_list} ")" "->" type block

parameter_list    ::= parameter ("," parameter)*

parameter         ::= IDENTIFIER ":" type

type              ::= primitive_type
                    # | user_defined_type
                    # | tuple_type
                    # | reference_type
                    # | array_type
                    # | function_type

primitive_type    ::= "int" | "float" | "bool"

block             ::= "{" statement* "}"

statement         ::= expression
                    | variable_decl ";"
                    | flow_statement ";"


expression        ::= primary_expression
                    | unary_expression
                    | binary_expression

primary_expression ::= literal
                    | IDENTIFIER
                    | "(" expression ")"
                    | function_call

literal           ::= INT | FLOAT | BOOLEAN

unary_expression  ::= "-" expression | "!" expression #| "&" expression | "*" expression

binary_expression ::= expression OPERATOR expression

function_call     ::= IDENTIFIER "(" arguments ")"
arguments         ::= expression ("," expression)* | ε

variable_decl     ::= "let" IDENTIFIER ":" type "=" expression

flow_statement    ::= "if" expression block {"else" block}
                    | "return" expression?
```

This is a simple grammar that supports the basic constructs of our language. It's not complete, but it's a good starting point. We can always expand it to support more complex constructs and language features.

# Resources
- [BNF](https://www.ketteringscienceacademy.org/attachments/download.asp?file=1057&type=pdf): The dates example is from this resource.
- [EBNF](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form): Wikipedia's page on EBNF.
- [EBNF](https://www.cl.cam.ac.uk/~mgk25/iso-14977.pdf): The official ISO standard for EBNF.
- [EBNF](https://www.cs.uic.edu/~liub/teach/cs494/ebnf.pdf): A more concise and readable resource on EBNF.
- [Intro to Programming Languages/Grammars](https://en.wikibooks.org/wiki/Introduction_to_Programming_Languages/Grammars): A Wikibooks page on grammars.