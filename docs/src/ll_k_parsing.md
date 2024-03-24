# LL(k) Parsing
> **❗Not required**
>
> This section is not required for the book, but is included for completeness.

Whilst [Recursive Descent Parsing (RDP)](./recursive_descent_parsing.md) is predominantly LL(1), it's worth noting that LL(k) parsing is a more general form of top-down parsing. The "LL" stands for "Left-to-right, Leftmost derivation", meaning that the parser reads the input from left to right and constructs a leftmost derivation of the input. The "k" represents the number of tokens of lookahead the parser uses to make parsing decisions.

This will not be implemented in this book, so you may skip it. I'd still recommend reading it to understand the differences between LL(1) and LL(k) parsing.

It is important to understanding [CFGs](./context_free_grammars.md) and [RDP](./recursive_descent_parsing.md) before reading this section.

## Introduction to LL(k) Parsing
LL(k) parsers are a superset of LL(k) parsers, where k is 1: whilst LL(1) only use a single look-ahead, LL(k) parsers use k tokens of lookahead to make parsing decisions. This could increase flexibility, but also complexity and memory usage.

Evaluation is similar to the one specified in [RDP](./recursive_descent_parsing.md#evaluation).

## How it works
**Parsing tables** are constructed based on the LL(k) grammar, which includes parsing action and goto tables. These tables are used to determine the next parsing action based on the current input token and lookahead. Simply put, parsing functions use these tables to make parsing decisions. They contain entries that map parsing states, input tokens, and lookahead symbols to to parsing actions or transitions.

**Parsing actions** are the operations that the parser performs when it encounters a specific input token and lookahead symbol. These actions include shifting the input token onto the parsing stack, reducing the stack based on a grammar rule, or accepting the input as a valid parse.

**Goto transitions** are the transitions that the parser makes when it encounters a specific input token and lookahead symbol. These transitions are used to move the parser from one state to another based on the current state and input token.

### Example
Given the following simple grammar:
$$
\begin{align*}
S &\rightarrow A \\\\
A &\rightarrow b
\end{align*}
$$

Let's construct the parsing table step by step.

#### Step 1: Compute FIRST and FOLLOW Sets
"FIRST" and "FOLLOW" sets are used to construct the parsing table. Let's break down that concept.

**FIRST Set**:
- The FIRST set of a non-terminal symbol is the set of terminals that begin the strings derivable from that non-terminal.
    - If the non-terminal can derive a terminal, that terminal is in the FIRST set.
    - If the non-terminal can derive a string that starts with another non-terminal, the FIRST set of that non-terminal is also in the FIRST set of the original non-terminal.
    - Repeat until no new terminals are added to the FIRST set.

**FOLLOW Set**:
- The FOLLOW set of a non-terminal symbol is the set of terminals that can appear immediately to the right of the non-terminal in some sentential form.
    - The end-of-input marker \\(\$\\) is in the FOLLOW set of the start symbol.
    - If there is a production \\(A \rightarrow \alpha B \beta\\), then everything in the FIRST set of \\(\beta\\) except for \\(\epsilon\\) is in the FOLLOW set of \\(B\\).
    - If there is a production \\(A \rightarrow \alpha B\\) or \\(A \rightarrow \alpha B \beta\\) where \\(\beta\\) is nullable, then everything in the FOLLOW set of \\(A\\) is in the FOLLOW set of \\(B\\).

Let's derive the FIRST and FOLLOW sets for the given grammar.

**FIRST Set**:
- \\(FIRST(S) = \{b\}\\), since \\(S \rightarrow A\\) and \\(A \rightarrow b\\).
- \\(FIRST(A) = \{b\}\\), since \\(A \rightarrow b\\).

**FOLLOW Set**:
- \\(FOLLOW(S) = \{\$\}\\), since \\(S\\) is the start symbol.
- \\(FOLLOW(A) = \{\$\}\\), since \\(S \rightarrow A\\).

#### Step 2: Construct the Parsing Table
We'll create a parsing table with rows corresponding to non-terminal symbols, and columns corresponding to terminal symbols (input \\($\\)).

These table entries will help indicate whether to perform a production, or an error action.

| Non-Terminal | Terminal (b) | $ |
|--------------|--------------|---|
| S            | S -> A       |   |
| A            | A -> b       |   |

The parsing table is constructed based on the FIRST and FOLLOW sets, and the grammar rules. The table is used to determine the next parsing action based on the current input token and lookahead.

#### Step 3: Populate the Parsing Table
Let's populate the parsing table based on the FIRST and FOLLOW sets.

For each production \\(A \rightarrow \alpha\\) in the grammar:
- If \\(\epsilon \in FIRST(\alpha)\\), then for each terminal \\(a\\) in \\(FOLLOW(A)\\), add \\(A \rightarrow \alpha\\) to the table entry \\(A, a\\).
    - Otherwise, add \\(A \rightarrow \alpha\\) to the table entry \\(A, a\\) for each terminal \\(a\\) in \\(FIRST(\alpha)\\).
- If \\(\epsilon \in FIRST(\alpha)\\) and \\(\$ \in FOLLOW(A) \\), add \\(A \rightarrow \alpha\\) to the table entry \\(A, \$\\).

The parsing table is populated as follows:

| Non-Terminal | Terminal (b) | $ |
|--------------|--------------|---|
| S            | S -> A       |   |
| A            | A -> b       |   |

Unfortunately no difference, as the grammar is simple, and I don't want to restart that process. The process is still clear though.

## Conclusion
LL(k) is a more general form of top-down parsing, and is much more flexible -- albeit more complex -- than LL(1) parsing. We will not be using it in this book.

# Resources
- [LL(k) Parsing](https://en.wikipedia.org/wiki/LL_parser) on Wikipedia
- [LL(k) Parsing](https://www.cs.uaf.edu/~cs331/notes/FirstFollow.pdf) on University of Alaska Fairbanks
- [LL(k) Parser](https://github.com/GabrielMajeri/LL-K-Parser): A Python implementation of an LL(k) parser