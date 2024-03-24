# Parsing Techniques
We've created a lexer to recognize tokens in the input into a stream of tokens, and we've designed a formal grammar for our language.

Next, we will need to write a parser to recognize the structure of the input and build an abstract syntax tree (AST) from it.

This section will cover how parsing works, the different techniques for parsing, and how to implement your own parser.

# Resources
- [Recursive Descent Parsing](https://en.wikipedia.org/wiki/Recursive_descent_parser) on Wikipedia
- [RDP Visualizer](https://maeyler.github.io/Automata-2018/cfg/Bilal_RecursiveDescentParser.html) - A visualizer for recursive descent parsers
- [LL(k) Parsing](https://en.wikipedia.org/wiki/LL_parser) on Wikipedia
- [LL(k) Parsing](https://www.cs.uaf.edu/~cs331/notes/FirstFollow.pdf) on University of Alaska Fairbanks
- [LL(k) Parser](https://github.com/GabrielMajeri/LL-K-Parser): A Python implementation of an LL(k) parser
- [LR Parsing](https://en.wikipedia.org/wiki/LR_parser) on Wikipedia
- [LR Parsing GfG](https://www.geeksforgeeks.org/lr-parser/): GeeksforGeeks article on LR Parsing