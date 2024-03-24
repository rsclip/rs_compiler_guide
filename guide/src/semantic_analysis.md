# Semantic Analysis

Semantic Analysis is a stage in the compilation process where the compiler checks the program for semantic errors; they're errors that occur when the program is syntactically correct but the meaning of the program is incorrect. For example, if you try to add a number to a string, the parser will not catch this error during parsing because the syntax is correct. However, the program will not run correctly because you cannot add a number to a string.

This code will be contained in the module `src/semantic_analysis`, so be sure to create that directory and add a file `mod.rs` to it.

> **Note**
>
>  Before implementing, it's advisable to adjust your error module to accept **warnings** too, since we will be analyzing the code for both errors and warnings.
> 
> We will not cover it here.

# Resources
- [Symbol Table - Wikipedia](https://en.wikipedia.org/wiki/Symbol_table)
- [Compiler Design: Symbol Table - GeeksforGeeks](https://www.geeksforgeeks.org/symbol-table-compiler/)
- [Symbol Table Lecture Slides](https://home.adelphi.edu/~siegfried/cs372/372l3.pdf) at Adelphi University
- [Symbol Table - Tutorialspoint](https://www.tutorialspoint.com/compiler_design/compiler_design_symbol_table.htm)
- [Scope Stacks - Persistant Array](https://en.wikipedia.org/wiki/Persistent_array)
- [Type Checking](https://en.wikipedia.org/wiki/Type_checking)
- [Algorithm W](https://raw.githubusercontent.com/mgrabmueller/AlgorithmW/master/pdf/AlgorithmW.pdf) by Martin Grabmüller
- [Hindley-Milner System](https://pfudke.wordpress.com/2014/11/20/hindley-milner-type-inference-a-practical-example-2/)
- [OCaml's type checker](https://okmij.org/ftp/ML/generalization.html)
- [Box](https://doc.rust-lang.org/std/boxed/struct.Box.html)
- [Memory Stack and Heap](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/the-stack-and-the-heap.html)