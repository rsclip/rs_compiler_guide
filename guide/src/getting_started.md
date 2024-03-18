# Getting Started
## Setting up your environment
Setting up your development environment varies on your Operating System and IDE. Here, we'll use Windows and VSCode. Assuming those are installed:

1. Install [rustup](https://rustup.rs/) and follow the instructions
2. Verify your installation: `rustc --version` and `cargo --version`
3. Install any IDE plugins you'd find useful
	1. [Code Runner](https://marketplace.visualstudio.com/items?itemName=formulahendry.code-runner), [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) are currently greatly useful. Debugging tools may help too.
4. Create a new project: `cargo new compiler`

I would recommend using git for version control.
### Crates
There are some crates (libraries) we can install to make the development process easier.

**Errors**
[anyhow](https://crates.io/crates/anyhow) is a great crate for propagating errors idiomatically.
[thiserror](https://crates.io/crates/thiserror) will help us to implement our own error enums.
For our error messages, this guide will [codespan-reporting](https://crates.io/crates/codespan_reporting) for its pretty errors and simple interface. Alternatively (and probably better), [ariadne](https://crates.io/crates/ariadne) produces better diagnostics.

**CLI Parsing**
This guide will use [gumdrop](https://crates.io/crates/gumdrop) since our compiler will be incredibly simplistic, and its much faster than [clap](https://crates.io/crates/clap) (although that's a great alternative, albeit relatively slow and large).

**Code Generation**
This guide (un)fortunately won't cover the specific code-generation as it's very out-of-scope and complex, not suitable for a beginner guide.

[Cranelift](https://cranelift.dev/) is a fast compiler backend made in Rust we'll use.
#### Want a library for lexing/parsing?
Although this guide is intended for writing these by hand, you can always rely on more industrial alternatives. I'd recommend using these only if you have written a lexer and parser by hand. Also, the errors may or may not be as pretty and insightful.

- [pest.rs](https://pest.rs/) is a powerful and elegant parser, very helpful for modifying your grammar.
- [chumsky](https://crates.io/crates/chumsky) is a parser with a focus on error recovery, which uses its sister crate [ariadne](https://crates.io/crates/ariadne).
- [logos](https://crates.io/crates/logos) is an incredibly fast lexer you can use if you don't want to tokenize your inputs yourself.
- [combine](https://crates.io/crates/combine) provides parser combinations with zero-copy support, making it very flexible.
#### Not fond of Cranelift?
If you want to look into more conventional Compiler infrastructures, [LLVM](https://llvm.org/) is the perfect candidate. There are multiple Rust bindings, so here's a brief overview:

- [Inkwell](https://crates.io/crates/inkwell) is a safe(?) idiomatic wrapper on LLVM
- [llvm-sys](https://crates.io/crates/llvm-sys) is a direct binding for LLVM
- [Vicis](https://github.com/maekawatoshiki/vicis) if you want to use LLVM IR, *without LLVM*. I found LLVM a pain to setup, so this may be useful.
### File structure
It's important to modularize your codebase so it's easier to use. We'll use a simple structure:

```
compiler/
|-- Cargo.toml
|-- grammar.bnf
+-- src
  +-- ast          // We will store our AST items here
  |-- errors.rs    // Error enum here
  |-- lexer.rs     // Lexical analysis
  |-- main.rs      // Main entry point
  |-- parser.rs    // Our parser
  |-- token.rs     // Our tokens
+-- tests
```

It's important to note that this is **not** the final structure.