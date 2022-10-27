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
enums are a way to define a type by enumerating its possible values. 

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
* `loop` - loops forever
* `while` - loops while a condition is true