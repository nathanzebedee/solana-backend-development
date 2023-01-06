# rust 101

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

### data types
**scalar types**:<br>
a scalar type represents a single value. Rust has four primary scalar types: 
* integers => `u8`, `u32`, `u64`, `usize`
* floating-points => `f32`, `f64`
* booleans => `true`, `false`
* characters => `char`

**compound types**:<br>
a compound type can group multiple values into one type:
* tuples => `(u8, u32, u64)`
* arrays => `[1, 4, 6]`
    * fixed-size list in contiguous memory where objects are the same data type
* structs => `struct User { name: String, age: u8 }`
* vectors => `vec![1, 2, 3]`
    * vectors are resizable arrays
* strings => `&str`, `String`
    * `&str` is a string slice, which is a reference to some UTF-8 encoded string data stored elsewhere (could point to heap, stack, or static memory)
    * `String` is an owned string type, which means it has data allocated on the heap

**functions**:<br>
functions are declared with the `fn` keyword, and follow familiar syntax for parameters and function body:
```
fn my_name(str: &str) -> bool {
    let my_name = "Nathan";
    if str == my_name {
        true 
    } else {
        false
    }
}
```
notice that you do not need a return statement to return a value in rust -- in this case, you simply exclude the `return` keyword and the `;` semicolon

### cargo
cargo is a package manager for rust, similar to npm for javascript. cargo is used to build, test, and publish rust projects. cargo is installed with rustup, which is the recommended way to install rust.

