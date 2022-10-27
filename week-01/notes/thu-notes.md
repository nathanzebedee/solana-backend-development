# 10/27/2022

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
```
fn main() {
    fn plus_one(x: Option<i32>) {
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

in the above example, we are using the `match` expression to handle the `Option` enum. we are using the `Some` and `None` variants to handle the possible cases of the `Option` enum.