# Parsing and Syntax Analysis
Now that we have a stream of tokens, we can start to analyze the syntax of the program. This is the process of parsing the tokens to determine the structure of the program. The result of this process is an abstract syntax tree (AST), which is a tree representation of the program's structure.

This chapter will cover the following topics:
- ASTs and their uses
- How grammars work, and how to use them
- Parsing techniques
- How to implement a parser

# Resources
- [Abstract Syntax Trees - Wikipedia](https://en.wikipedia.org/wiki/Abstract_syntax_tree)
- [ASTs vs CSTs](https://eli.thegreenplace.net/2009/02/16/abstract-vs-concrete-syntax-trees)
- [Rust Syn crate](https://docs.rs/syn/1.0.72/syn/)
- [Tree Data Structures](https://en.wikipedia.org/wiki/Tree_(data_structure))
- [Tree Traversal](https://en.wikipedia.org/wiki/Tree_traversal)
- [DFS, BFS](https://www.cs.cornell.edu/courses/cs2110/2017sp/online/dfs/dfs01.html)
- [BNF](https://www.ketteringscienceacademy.org/attachments/download.asp?file=1057&type=pdf): The dates example is from this resource.
- [EBNF](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form): Wikipedia's page on EBNF.
- [EBNF](https://www.cl.cam.ac.uk/~mgk25/iso-14977.pdf): The official ISO standard for EBNF.
- [EBNF](https://www.cs.uic.edu/~liub/teach/cs494/ebnf.pdf): A more concise and readable resource on EBNF.
- [Intro to Programming Languages/Grammars](https://en.wikibooks.org/wiki/Introduction_to_Programming_Languages/Grammars): A Wikibooks page on grammars.
- [Recursive Descent Parsing](https://en.wikipedia.org/wiki/Recursive_descent_parser) on Wikipedia
- [RDP Visualizer](https://maeyler.github.io/Automata-2018/cfg/Bilal_RecursiveDescentParser.html) - A visualizer for recursive descent parsers
- [LL(k) Parsing](https://en.wikipedia.org/wiki/LL_parser) on Wikipedia
- [LL(k) Parsing](https://www.cs.uaf.edu/~cs331/notes/FirstFollow.pdf) on University of Alaska Fairbanks
- [LL(k) Parser](https://github.com/GabrielMajeri/LL-K-Parser): A Python implementation of an LL(k) parser
- [LR Parsing](https://en.wikipedia.org/wiki/LR_parser) on Wikipedia
- [LR Parsing GfG](https://www.geeksforgeeks.org/lr-parser/): GeeksforGeeks article on LR Parsing