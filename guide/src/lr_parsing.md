# LR Parsing
> **❗Not required**
>
> This section is not required for the book, but is included for completeness.

<div class="warning">
    <p><strong style='font-size: 2.5rem;'>This chapter is incomplete</strong><br/></p>
</div>

LR Parsing is a bottom-up parsing technique that is more powerful than LL Parsing. It is used to parse large grammars and is more efficient than LL Parsing. The "L" stands for left-to-right scanning of the input, and the "R" stands for constructing a rightmost derivation in reverse.

This will not be implemented in this book, so you may skip it. Compared to [LL(k) parsers](./ll_k_parsing.md), LR parsers are more powerful and can handle a larger class of grammars. They are also more efficient and can parse larger grammars in \\(O(n)\\) time.

It is important to understand [Recursive Descent Parsing (RDP)](./recursive_descent_parsing.md) and [LL(k) Parsing](./ll_k_parsing.md) before reading this section.

## Introduction to LR Parsing
LR Parsing is pretty much starting with the basic ingredients to bake a cake (flour, eggs, etc.) and then adding them in the right order to make a cake. It uses a different approach: **shift-reduce parsing**.

### How it works
LR Parsing uses a **shift-reduce** approach to parsing. It reads the input from left to right and constructs a rightmost derivation of the input. The parser uses a stack to keep track of the input and the grammar rules that have been applied. The parser then uses a set of parsing actions to shift input tokens onto the stack, reduce the stack based on a grammar rule, or accept the input as a valid parse.

Much like LL(k) Parsing, LR Parsing uses a **parsing table** to determine the next parsing action based on the current input token and lookahead. The parsing table contains entries that map parsing states, input tokens, and lookahead symbols to parsing actions or transitions.

As the parser reads the tokens, it shifts them onto a stack while simultaneously reducing them into larger structures. This process involves comparing the tokens to the rules of the programming language's grammar.

# Resources
- [LR Parsing](https://en.wikipedia.org/wiki/LR_parser) on Wikipedia
- [LR Parsing GfG](https://www.geeksforgeeks.org/lr-parser/): GeeksforGeeks article on LR Parsing