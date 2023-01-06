# rust continued

### ideomatic rust
the patters you use to design our rust programs are different from those we use in building programs in other languages, such as python or javascript. 

### generics
generics are a way to parameterize across datatypes, such as we do above with Option<T> where T is the parameter that can be replaced with any given datatype (i32, String, f64, etc).

```
struct Point<T> {
    x: T,
    y: T
}

fn main() {
    // x and y are both i32
    let my_point = Point { x: 5, y: 6 };
}
```

in the example above, we are defining a struct with two fields, x and y, that are of the same type, T. We then instantiate the struct with the type i32, which means that x and y are both of type i32. 

that being, if you wanted to declare a struct with two fields of different types, you would do the following:

```
struct Point<T, U> {
    x: T,
    y: U
}

fn main() {
    // x is i32, y is f64
    let my_point = Point { x: 5, y: 6.0 };
}
```

each generic can only be one datatype, so you can't have a struct with two fields of the same type, T, but different types, U.

### enums
enums are a way to define a type by enumerating its possible values. in an enum, we have fixed choices, whereas in a struct, we have a set of fields that can be of any type. 

an enum can be thought of as a flat heirarchy of fixed data structures.

```
enum IpAddrKind {
    V4,
    V6
}
```

### control flow
the `if` keyword is followed by a condition which must evaluate to a `bool`. what is important to note, is that the `if` is actually an **expression**, not a statement -- it returns a value.

```
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);
}
```

however, you must have the same type for both branches of the `if` expression.

### looping
* `loop` - loops forever (until we `break` out of it)
* `while` - loops while a condition is true

### option
the `Option` enum allows us to handle situations where expressions might (or might not) return a value. 

The `Option<T>` enum has to variants:
* `Some` - a tuple struct that wraps a `value` with `T`
* `None` - indicates the lack of a value; substitutes `null` in other languages

the `Option` enum is very useful for avoiding errors that can occur when we try to access a value that doesn't exist.

### result
the `Result` enum is similar to the `Option` enum, but it is used for error handling.

The `Result<T, E>` enum has two variants:
* `Ok` - a tuple struct that wraps a `value` with `T`
* `Err` - a tuple struct that wraps an `error` with `E`

```
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### matching
the `match` expression is used to handle all possible cases of a value. 


```
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    _ => EXPRESSION, // matches all other cases
}
```

it is a flexible way of handling multiple cases, and is similar to a `switch` statement in other languages. in the example above, the `_` is a catch-all pattern that will match all other cases.

```
enum Coin {
    Penny, 
    Quarter
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin:: Quarter => 25
    }
}
```

the keyword `match` is followed by an expression: `coin`. the value of this expression is compared with the patterns in each arm of the `match` expression. if a pattern matches the value, the code associated with that pattern is executed. because each arm is an expression, we can return a value from each arm as well.

### matching with option
in the below example, we are using the `match` expression to handle the `Option` enum. we are using the `Some` and `None` variants to handle the possible cases of the `Option` enum.

if a `i32` is passed into the function, it is iterated by one and wrapped in a `Some` variant. if `None` or any other datatype is passed into the function, `None` is returned.

```
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
            None => None,
            _ => None
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
```

## iterators
the iterator, `.iter()`, in rust is optimized in that is has no effect until it is needed. for this reason, we call iterators *lazy*.

```
let names = vec!["Bob", "Frank", "Ferris"];
let names_iter = names.iter();
```

the above code creates an iterator for us that we can use to iterate through the names use `.next()`.

```
fn iterator_demo() {
    let v1 = vec![1, 2];
    let mut v1_iter = v1.iter();

    assert_eq!(vi_iter.next(), Some(&1));
    assert_eq!(vi_iter.next(), Some(&2));
    assert_eq!(vi_iter.next(), None);
}
```

### vectors
vectors are one of the most useful types of collections in rust. they are similar to arrays, but they can grow or shrink in size. 

we can create a new vector using `Vec::new()` or by using the `vec!` macro.

```
let v: Vec<i32> = Vec::new();
let v: Vec<i32> = vec![1, 2, 3];
```

we do not necessarily need to specify the type when declaring a vector using the `vec!` macro, as the macro allows the compiler to infer the type of the values.

we can retrieve items from the vector by:
* `get` - `v.get(2)` returns `Some(&3)`
* `[]` - `v[2]` returns `3`

if we want to iterate over a vector, we can use the `.iter()` method or a loop:
```
let v = vec![1, 2, 3, 4, 5];

// loop
for i in &v {
    println!("{}", i);
}

// iterator
let mut iterator = v.iter();
println!("{}", iterator.next().unwrap());
```

### shadowing
shadowing is when we declare a new variable with the same name as the previous variable. the first variable is said to be *shadowed* by the new variable.

it is important to note that the variables exist in their own scopes.

```
fn main() {
    let y = 2; // 2
    let y = y * 3; // 6

    {
        let y = y * 2; 
        println!("{}", y); // 12
    }

    println!("{}", y); // 6
}
```

### macros
macros allow us to avoid code duplication. some examples of macros are:
* `vec!`: creates a new vector
* `println!`: prints to the console

we can identify a macro by the `!` at the end of the name.

### hashmaps
where vectors store values by an integer index, `HashMap` stores values by *key*. `HashMap` keys can be a **string**, a **boolean**, a **integer**, or any other type that implements the `Eq` and `Hash` traits. a `HashMap` can grow, and *shrink* if it has excess space.

you can create a `HashMap` using:
* `HashMap::with_capacity(uint)`
* `HashMap::new()`

```
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10); // insert a key-value pair - key: String, value: i32
}
```

you can use an *iterator* (`.iter()` method) to get values from the hashmap:

```
use std::collections::HashMap;

fn main() {
    let mut balances = HashMap::new();
    balances.insert(String::from("USD"), 100); // key-value pair

    for (denom, balance) in balances.iter() {
        println!("{}: {}", denom, balance);
    }
}
```

if we want a *particular* value, we can use the `.get()` method. the `.get()` method returns an `Options<&V>` where `V` is teh type of the value.

```
use std::collections::HashMap;

fn main() {
    let mut balances = HashMap::new();
    balances.insert("USD", 100); // key-value pair

    let denom = String::from("USD");
    let balance = balances.get(&denom).copied().unwrap_or(0);
}
```

### traits
traits are a way of defining the behavior that a type has and can share with other types.

trait definitions are a way to group method signatures together to define a set of behaviors necessary for a particular purpose.

```
pub trait Summary {
    fn summarize(&self) -> String;
}
```

we can **implement a trait** on a type by using the `impl` keyword.

```
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

