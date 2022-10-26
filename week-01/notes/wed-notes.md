# 10/26/2022

### rust in context with other low-level languages
rust is a general purpose, low-level systems language that prioritizes safety and speed. compared to other popular low-level languages, like c++, rust offers comparable performance, but with a much more robust type system and memory safety. rust is also a compiled language, which means that it is compiled into machine code before it is run. this means that rust is much faster than interpreted languages like python, but it also means that rust is not as flexible as interpreted languages. 

### core features
* memory safety without garbage collection
* concurrency without data races
* abstraction without overhead

### variables
variable bindings are immutable by default, but this can be overridden using the `mut` keyword.

```
let x = 1; // immutable
let mut y = 1 // mutable
```

