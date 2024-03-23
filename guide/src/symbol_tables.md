# Symbol Tables

## Introduction
When performing semantic analysis, you may need to check the validity (and usage) of identifiers like variables and functions. For example, there may be a function call to a function which hasn't been declared yet.

To solve this, we use **Symbol Tables**. These tables store information about identifiers in the program, such as their type, scope, and location. They help the compiler resolve identifiers to their declarations and detect errors like undeclared variables or functions.

## Components of a Symbol Table
A symbol table typically consists of:
- **Entries**: Data structures representing individual symbols (e.g., identifier, type)
- **Scope**: A region of the program where a symbol is visible and accessible
    - It is often hierarchical, with symbol tables containing a reference to the parent scope
- **Operations**: Common operations include insertion, lookup, and deletion of symbols

Let's look at an example function declaration in Python:

```python,ignore
x: int = 10
y: int = 20
z: int = x + y
```

Let's go construct a symbol table step by step.

| Line | Symbol Table |
|------|--------------|
| `x: int = 10` | Add `x: int`: <br/> <table> <thead> <tr> <th>Symbol</th> <th>Type</th> </tr> </thead> <tbody> <tr> <td>x</td> <td>int</td> </tr> </table> <br/> |
| `y: int = 20` | Add `y: int`: <br/> <table> <thead> <tr> <th>Symbol</th> <th>Type</th> </tr> </thead> <tbody> <tr> <td>x</td> <td>int</td> </tr> <tr> <td>y</td> <td>int</td> </tr> </table> <br/> |

`x` and `y` (of type `int`) were declared here, so we insert them directly into the symbol table.

The next line `z: int = x + y` involves an expression with `x` and `y`. We need to check if `x` and `y` are declared before. If they are, we can add `z` to the symbol table.

| Line | Symbol Table |
|------|--------------|
| `z: int = x + y` | 1. Ensure if `x` and `y` are declared. <br/> 2. Add `z: int`: <br/> <table> <thead> <tr> <th>Symbol</th> <th>Type</th> </tr> </thead> <tbody> <tr> <td>x</td> <td>int</td> </tr> <tr> <td>y</td> <td>int</td> </tr> <tr> <td>z</td> <td>int</td> </tr> </table> <br/> |

And since they were declared, we can add `z` to the symbol table! Let's add a new line of code:
```python,ignore
x: int = 10
y: int = 20
z: int = x + y
a: int = b + c  # New line
```


| Line | Symbol Table |
|------|--------------|
| `a: int = b + c` | 1. `b` and `c` are undeclared, raise an error <br/> 2. Add `a: int`: <br/> <table> <thead> <tr> <th>Symbol</th> <th>Type</th> </tr> </thead> <tbody> <tr> <td>x</td> <td>int</td> </tr> <tr> <td>y</td> <td>int</td> </tr> <tr> <td>z</td> <td>int</td> </tr> <tr> <td>a</td> <td>int</td> </tr> </table> <br/> |

Here, `b` and `c` in `a`'s declaration are undeclared, so we raise an error.

You hopefully noticed that we still added `a` to the symbol table. We did this because the error was raised, and it's guaranteed the program will not continue to the next stage of the compilation process. However, since we will continue to look through the program for errors, it's useful to keep track of all the symbols we've seen so far. This is known as **error recovery**.

## Scopes

Hash Tables are a popular choice for implementing symbol tables, due to their \\(O(1)\\) average-case lookup time. Without going into much detail, in a hash table-based symbol table, identifiers are mapped to unique indices using a hash function. Because of this, we can quickly find the symbol's entry in the table.

Let's look at a simplistic implementation in C++. We'll assume that we have the `SymbolEntry` class defined elsewhere.

```cpp
#include <unordered_map>
#include <string>

class SymbolTable {
private:
    std::unordered_map<std::string, SymbolEntry> table;

public:
    void insert(const std::string& name, const SymbolEntry& entry) {
        table[name] = entry;
    }

    SymbolEntry lookup(const std::string& name) {
        return table[name];
    }

    void remove(const std::string& name) {
        table.erase(name);
    }
};
```

This symbol table implementation allows us to determine which variables are declared within the current scope. One problem, however, is that typically within programs you're able to access variables in parent scopes, which this implementation doesn't account for. To handle this, we can use different methods.

We'll cover scope stacks and nested symbol tables, but there are methods. Here are some of the most common ones:
- Dynamic Scoping
- Scope Chains

### Scope Stack
A scope stack is a stack of symbol tables owned by the compiler. It contains all the symbol tables for the scopes \\(S\\) that the compiler has encountered so far. When the compiler enters a new scope, it pushes a new symbol table onto the stack. When it exits the scope, it pops the symbol table off the stack.

Let's see how scope stacks work when analyzing the following Rust code:

```rust
fn main() {
    let x = 10;
    {
        let y = 20;
        println!("{}", x + y);
    }
}
```

Here's how the scope stack would look like:
| Code | Scope Stack | |
|------|-------------|-|
| `fn main()` | \\([S_{\\text{global}}]\\) | The global scope is pushed onto the stack. |
| `{` | \\([S_{\\text{global}}, S_{\\text{main}}]\\) | The `main` scope is pushed onto the stack. |
| `let x = 10;` | \\([S_{\\text{global}}, S_{\\text{main}}]\\) | `x` is added to the `main` scope. |
| `{` | \\([S_{\\text{global}}, S_{\\text{main}}, S_{\text{block}}]\\) | A new block scope is pushed onto the stack. |
| `let y = 20;` | \\([S_{\\text{global}}, S_{\\text{main}}, S_{\text{block}}]\\) | `y` is added to the block scope. |
| `println!("{}", x + y);` | \\([S_{\\text{global}}, S_{\\text{main}}, S_{\text{block}}]\\) | `x` and `y` are looked up in the block scope. |
| `}` | \\([S_{\\text{global}}, S_{\\text{main}}]\\) | The block scope is popped off the stack. |
| `}` | \\([S_{\\text{global}}]\\) | The `main` scope is popped off the stack. |

The line of code `println!("{}", x + y);` involves `x` and `y`. Let's evaluate:

1. We look up `x` in the current scope \\(S_{\text{block}}\\)
    1. `x` is not found in \\(S_{\text{block}}\\). Let's look in the parent scope \\(S_{\text{main}}\\).
    2. `x` is found in \\(S_{\text{main}}\\). We can proceed.
2. We look up `y` in the current scope \\(S_{\text{block}}\\)
    1. `y` is found in \\(S_{\text{block}}\\). We can proceed.

Both `x` and `y` are found, so we can evaluate the expression.

### Nested Symbol Tables (tree structure)
Another approach is to use nested symbol tables, where each symbol table corresponds to a specific scope. When the compiler enters a new scope, it creates a new symbol table and links it to the parent scope's symbol table. This way, the compiler can traverse the symbol tables hierarchically to resolve identifiers.

Let's see how a nested symbol table may operate in the same Rust code:

```rust
fn main() {
    let x = 10;
    {
        let y = 20;
        println!("{}", x + y);
    }
}
```

| Code | Symbol Table Tree |  |
|------|--------------|--------------|
| `fn main()` | \\(S_{\text{global}}\\) | Create global scope |
| `{` | \\(S_{\text{global}} \\\\ \\quad S_{\text{main}} \\) | Create main scope |
| `let x = 10;` | \\(S_{\text{global}} \\\\ \\quad S_{\text{main}} \\) | Add `x` to main scope |
| `{` | \\(S_{\text{global}} \\\\ \\quad S_{\text{main}} \\\\ \\quad \\quad S_{\text{block}} \\) | Create block scope |
| `let y = 20;` | \\(S_{\text{global}} \\\\ \\quad S_{\text{main}} \\\\ \\quad \\quad S_{\text{block}} \\) | Add `y` to block scope |
| `println!("{}", x + y);` | \\(S_{\text{global}} \\\\ \\quad S_{\text{main}} \\\\ \\quad \\quad S_{\text{block}} \\) | Look up `x` and `y` in block scope |
| `}` | \\(S_{\text{global}} \\\\ \\quad S_{\text{main}} \\) | Remove block scope |
| `}` | \\(S_{\text{global}}\\) | Remove main scope |

Let's follow the steps to resolve `x` and `y`:

1. Look up `x` in the block scope
    1. `x` is not found in the block scope. Look up in the parent scope (main scope).
    2. `x` is found in the main scope. Proceed.
2. Look up `y` in the block scope
    1. `y` is found in the block scope. Proceed.

It is very similar to the scope stack approach, but the symbol tables are organized in a tree structure.

### Scope Shadowing
Shadowing is a common practice in programming languages where a variable in an inner scope has the same name as a variable in an outer scope. The variable in the inner scope "shadows" the variable in the outer scope, meaning the inner variable takes precedence. In Rust, it looks like this:

```rust
let x: i32 = 10;
{
    let x: i32 = 20;
    println!("{}", x); // Prints 20
}
println!("{}", x); // Prints 10
```

The inner `x` shadows the outer `x` within the block scope. This is simple to implement with symbol tables by adding a new entry for the shadowed variable in the inner scope.

Variable shadowing is a way to *re-use* a variable, overwriting the previous value, not to be confused with scope shadowing, where a variable in an inner scope hides a variable in an outer scope.

## Symbol Attributes and Metadata
Within a symbol entry, you can store additional information about the symbol. This varies depending on what the symbol is, but common attributes include:
- **Type**: The data type of the symbol (e.g., `int`, `float`, `string`)
- **Scope**: The scope in which the symbol is declared
    - May not be needed if using scope stacks or nested symbol tables
- **Span**: The location of the symbol in the source code (e.g., line number, column number)
- **Modifiers**: Additional information like `const`, `static`, `public`, `private`
    - This can help enforce language-specific rules and optimizations

Using these attributes will help you perform more advanced semantic analysis, such as type checking, scope resolution, and error reporting.