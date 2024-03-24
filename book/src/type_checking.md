# Type Checking
Type Checking is a vital part of the compilation process; it ensures that all types are matching and that the program is semantically correct.

Some languages, like Python, are dynamically typed, meaning that the type of a variable is determined at runtime. This allows for more flexibility but can lead to runtime errors if the types don't match. On the other hand, statically typed languages, like C++, require the type of a variable to be known at compile time. This allows for type checking to be done at compile time, catching errors before the program is run.

We'll be using statically typed languages as our example, but the concepts can be applied to dynamically typed languages as well.

## Type Systems
There are different types of type systems, each with its own set of rules. It's important you know which type system your language uses, as it will affect how you implement type checking.

### Strong vs Weak typing
A language is strongly typed if it enforces strict type checking, meaning that the type of a variable is checked before it's used. Weakly typed languages, on the other hand, allow for more flexibility, often implicitly converting types.

Here's an example of some code which would be valid in a weakly typed language but not in a strongly typed language:

```c
int x = 10;
char y = 'a';
int z = x + y; // Error in a strongly typed language
```

Here we attempt to apply the addition operation between two operands of different types. We cannot add a `char` to an `int` in a strongly typed language, but weakly typed languages (such as Java) apply this conversion implicitly.

### Explicit vs Implicit typing
Languages can also be classified based on how they handle type declarations. In languages with explicit typing, the type of a variable must be explicitly declared. In contrast, languages with implicit typing infer the type of a variable based on its value.

Not all languages strictly fall under these two categories; some may have a mix of both. Rust, for example, is a statically typed language with type inference.

### Type Compatibility and Coercion
Type compatibility refers to whether two types can be used interchangeably. For example, in C, an `int` can be implicitly converted to a `float`, but not the other way around. This is known as type coercion.

<div class="warning">
<strong>Warning</strong>: 
Type coercion can lead to unexpected behavior and bugs, so it's essential to understand how your language handles type compatibility.
</div>

## Type Checking Algorithms
### Hindley-Milner Algorithm
The Hindley-Milner algorithm is a type inference algorithm used in functional programming languages, such as Haskell. It's based on the lambda calculus and is used to infer the most general type of an expression. Our language is not functional, so we won't be covering this algorithm in detail.

This algorithm is mainly able to infer the types of expressions without the need for explicit type annotations (often with no explicit type annotations at all!). 

# Resources
- [Type Checking](https://en.wikipedia.org/wiki/Type_checking)
- [Algorithm W](https://raw.githubusercontent.com/mgrabmueller/AlgorithmW/master/pdf/AlgorithmW.pdf) by Martin Grabmüller
- [Hindley-Milner System](https://pfudke.wordpress.com/2014/11/20/hindley-milner-type-inference-a-practical-example-2/)
- [OCaml's type checker](https://okmij.org/ftp/ML/generalization.html)