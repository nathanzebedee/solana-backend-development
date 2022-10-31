# 10/31/2022

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
strings in rust (String) are dynamic data types that are stored on the heap. a string is made up of three parts:
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

