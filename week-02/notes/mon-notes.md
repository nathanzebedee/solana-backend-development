# rust ownership

### memory - heap and stack
**stack**:
* the simplest place to store data
* data stored on stack must have known, fixed size at compile time
* copying items on the stack is fast and (memory) cheap
**heap**:
* used to store more complex, dynamic data structures
* data stored on heap can have unknown size at compile time
* we typically do not want to copy data on the heap - it is very expensive

### clearing memory
**garbage collection**:
newer languages (python, go, etc) have background programs that automatically keep track of and remove items from the stack and heap that are no longer being used.
* pros:
    * easy to use
    * no need to worry about memory management
* cons:
    * programs are slower
    * unable to make optimizations for memory allocation
**manual memory management**:
older languages (c, c++, etc) require you to manually keep track of the memory you have been allocated and free it when you are done with it.
* pros:
    * control over memory management (for optimizations)
* cons:
    * can be hard to use
    * very easy to make critical mistakes
**ownership**:
rust as a very strong compile-time memory safety guarantee, which is achieved through a combination of three features:
* ownership
* borrowing
* lifetimes

### string data types
strings in rust (`String`) are dynamic data types that are stored on the heap. a string is made up of three parts:
* pointer to the memory location (internal buffer) of the string
* length of the string (number of bytes stored in the buffer)
* capacity of the string (the size of the buffer in bytes)

the length will always be <= the capacity. this buffer is *always* stored on the heap.

```
let story = String::from("story");
let len = story.len();
let capacity = story.capacity();
```

we can create a String from a literal and append to it:

```
let mut title = String::from("Solana");
title.push_str(" is a blockchain");
println!("{}", title); // "Solana is a blockchain"
```

### ownership
by placing restrictions on our code, rust gives us control over the lifetime of our data while guaranteeing memory safety.

*ownership* means that a variable owns the data it points to. when a variable goes out of scope, the data it owns is dropped. this is done to prevent dangling pointers.

```
// here, `my_vec` owns the vector
let mut my_vec = vec![1, 2, 3];
```

what it means to own a variable:
* each value is rust has a variable that is called its *owner*
* there can only be one owner at a time
* when the owner goes out of scope, the value will be dropped

```
fn main() {
    { // in this scope, `s` owns the String
        let s = String::from("hello, world");
    } // here, `s` goes out of scope and is dropped
    println!("{}", s); // error: `s` does not exist
}
```

### copying
for simple data types that can exist on the stack (i.e., integers, etc), when we make an assignment we can just copy the value.

```
fn main() {
    let a = 3;
    let b = a;
}
```

types that can be copied:
* all integer types
* the Boolean type `bool` with values `true` and `false`
* all floating point types
* the character type `char`
Tuples, if they only contain types that are also Copy. For example, `(i32, i32)` is Copy, but `(i32, String)` is not.

for more complex data types (such as `String`), we need memory allocated on the heap, and then the ownership rules apply.

### moving ownership
for none copy types (such as `String`), when we make an assignment or set a parameter to a function, we are *moving* the ownership of the data. the source gives up ownership to the destination, and then the source is uninitialized:

```
fn main() {
    let a = vec![1, 2, 3];
    let b = a;
    let c = a; // error: `a` does not have ownership of the vector

    let param = String::from("param");
    takes_ownership(param);
    println!("{}", param); // error: `param` does not have ownership of the String
}

fn takes_ownership(param: String) {
    assert_eq!(param, "param");
}
```

### references
references are a flexible means to help us with ownership and variable lifetimes. they are written as `&T` followed by the variable name (in place of `T`). they are immutable by default, but can be made mutable with `&mut T`.

by using references `&`, we can use a value without taking ownership of it. we are *borrowing* the value. references are essentially pointers to the referenced value in memory.

by using a mutable reference, we can read and modify teh referent.

rules between mutable and immutable references **for a given scope**:
* you can have *either* one mutable reference `&mut T`
* *or* any number of immutable references `&T`

the above rules are meant to prevent *data races* - when we read data as it is being written to. they follow the "multiple reader; single writer" principle.

```
fn main() {
    let mut s = String::from("hello");
    let s1: &mut String = &mut s;
    s1.push_str(", world");
    println!("{}", s1); // "hello, world"
}
```

## de referencing
we can use the `*` operator to de reference a pointer. this is useful when we want to modify the value the pinter is pointing to on the heap.

```
fn main() {
    let a = 3;
    let b = &a;
    assert_eq!(b, a); // false: `&int` is not equal to `int`
    assert_eq!(*b, a); // true: `int` is equal to `int`
}
```

### slices
slices are a way to reference a contiguous sequence of elements in a collection rather than the whole collection. they are written as `&[T]` or `&mut [T]`. slices can be used to borrow a section of a `String`, `Vec`, or array.

slices are similar to arrays, but their size is not known at compile time. slices are *two-word objects*, which contain:
* first word: a pointer to the data
* second word: the length of the slice

```
fn main() {
    let array = [1, 2, 3, 4, 5];
    let slice = &array[1..3];
    assert_eq!(slice, [2, 3]);
}
```

### printing and outputting
* `format!`: write formatted text to a String
* `print!`: same as `format!`, but the text is printed to the console (io::stdout)
* `println!`: same as `print!`, but a newline is appended
* `eprint!`: same as `format!`, but the text is printed to the standard error (io::stderr)
* `eprintln!`: same as `eprint!`, but a newline is appended

use braces `{}` to format the output. the number of arguments must match the number of braces, and are inserted sequentially.

```
println!("{} days in the month of {}", 31, "January");
// "31 days in the month of January"
```

