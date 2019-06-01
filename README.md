# Functional Programming Jargon in Rust

Functional programming (FP) provides many advantages, and its popularity has been increasing as a result.
However, each programming paradigm comes with its own unique jargon and FP is no exception. By providing a glossary,
we hope to make learning FP easier.

Where applicable, this document uses terms defined in the [Fantasy Land spec](https://github.com/fantasyland/fantasy-land)

The goal of this project includes understanding Rust's capability of programming in functional paradigm.

__Table of Contents__
<!-- RM(noparent,notop) -->

## Arity

The number of arguments a function takes. From words like unary, binary, ternary, etc.
This word has the distinction of being composed of two suffixes, "-ary" and "-ity."
Addition, for example, takes two arguments, and so it is defined as a binary function or a function with an arity of two.
Such a function may sometimes be called "dyadic" by people who prefer Greek roots to Latin.
Likewise, a function that takes a variable number of arguments is called "variadic,"
whereas a binary function must be given two and only two arguments, currying and partial application notwithstanding (see below).

```rust
let sum = |a: i32, b: i32| { a + b }; // The arity of sum is 2
```

## Higher-Order Functions (HOF)

A function which takes a function as an argument and/or returns a function.

```rust
let filter = | predicate: fn(&i32) -> bool, xs: Vec<i32> | {
    return xs.into_iter().filter(predicate).collect::<Vec<i32>>();
};
```

```rust
let is_even = |x: &i32| { x % 2 == 0 };
```

```rust
filter(is_even, vec![1, 2, 3, 4, 5, 6]);
```

## Closure

A closure is a scope which retains variables available to a function when it's created. This is important for
[partial application](#partial-application) to work.

```rust
let add_to = | x: i32 | { move | y: i32 | { x + y } };
    

```

We can call `add_to` with a number and get back a function with a baked-in `x`. Notice that we also need to move the
ownership of the y to the internal lambda.

```rust
let add_to_five = add_to(5);
```

In this case the `x` is retained in `add_to_five`'s closure with the value `5`. We can then call `add_to_five` with the `y`
and get back the desired number.

```rust
add_to_five(3); // => 8
```

Closures are commonly used in event handlers so that they still have access to variables defined in their parents when they
are eventually called.

__Further reading__
* [Lambda Vs Closure](http://stackoverflow.com/questions/220658/what-is-the-difference-between-a-closure-and-a-lambda)
* [How do JavaScript Closures Work?](http://stackoverflow.com/questions/111102/how-do-javascript-closures-work)

## Partial Application 

Partially applying a function means creating a new function by pre-filling some of the arguments to the original function.

To achieve this easily, we will be using a [partial application crate](https://crates.io/crates/partial_application)

```rust
#[macro_use]
extern crate partial_application;

fn foo(a: i32, b: i32, c: i32, d: i32, mul: i32, off: i32) -> i32 {
    (a + b*b + c.pow(3) + d.pow(4)) * mul - off
}

let bar = partial!( foo(_, _, 10, 42, 10, 10) );

assert_eq!(
    foo(15, 15, 10, 42, 10, 10),
    bar(15, 15)
); // passes
```

Partial application helps create simpler functions from more complex ones by baking in data when you have it.
Curried functions are automatically partially applied.

__Further reading__
* [Partial Application in Haskell](https://wiki.haskell.org/Partial_application)


## Currying

The process of converting a function that takes multiple arguments into a function that takes them one at a time.

Each time the function is called it only accepts one argument and returns a function that takes one argument until all arguments are passed.


```rust
fn add(x: i32) -> impl Fn(i32)-> i32 {
    return move |y| x + y;
}

let add5 = add(5);
add5(10); // 15
```

__Further reading__
* [Currying in Rust](https://hashnode.com/post/currying-in-rust-cjpfb0i2z00cm56s2aideuo4z)

## Purity

A function is pure if the return value is only determined by its input values, and does not produce side effects.

```rust
let greet = |name: &str| { format!("Hi! {}", name) };

greet("Jason"); // Hi! Jason
```

As opposed to each of the following:

```rust
let name = "Jason";

let greet = || -> String {
    format!("Hi! {}", name)
};

greet(); // String = "Hi! Jason"
```

The above example's output is based on data stored outside of the function...

```rust
let mut greeting: String = "".to_string();

let mut greet = |name: &str| {
    greeting = format!("Hi! {}", name);
};

greet("Jason");

assert_eq!("Hi! Jason", greeting); // Passes
```

... and this one modifies state outside of the function.

## Side effects

A function or expression is said to have a side effect if apart from returning a value,
it interacts with (reads from or writes to) external mutable state.

```rust
use std::time::SystemTime;

let now = SystemTime::now();
```

```rust
println!("IO is a side effect!");
// IO is a side effect!
```

## Idempotent

A function is idempotent if reapplying it to its result does not produce a different result.

```rust
// Custom immutable sort method
let sort = | x: Vec<i32> | -> Vec<i32> {
    let mut cloned_x = x.clone();
    cloned_x.sort();
    return cloned_x;
};
```

Then we can use the sort method like

```rust
let x = vec![2 ,1];
let sorted_x = sort(sort(x.clone()));
let expected = vec![1, 2];
assert_eq!(sorted_x, expected); // passes
```

```rust
let abs = | x: i32 | -> i32 {
    return x.abs();
};

let x: i32 = 10;
let result = abs(abs(x));
assert_eq!(result, x); // passes
```

## Auto currying

## Function Composition

The act of putting two functions together to form a third function where the output of one function is the input of the other.
Below is an example of compose function is Rust.

```rust
macro_rules! compose {
    ( $last:expr ) => { $last };
    ( $head:expr, $($tail:expr), +) => {
        compose_two($head, compose!($($tail),+))
    };
}

fn compose_two<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}
```

Then we can use it like

```rust
let add = | x: i32 | x + 2;
let multiply = | x: i32 | x * 2;
let divide = | x: i32 | x / 2;

let intermediate = compose!(add, multiply, divide);

let subtract = | x: i32 | x - 1;

let finally = compose!(intermediate, subtract);

let expected = 11;
let result = finally(10);
assert_eq!(result, expected); // passes
```

## Continuation

## Point-Free style

Writing functions where the definition does not explicitly identify the arguments used.
This style usually requires currying or other Higher-Order functions. A.K.A Tacit programming.

## Predicate

A predicate is a function that returns true or false for a given value.
A common use of a predicate is as the callback for array filter.

```rust
let predicate = | a: &i32 | a.clone() > 2;

let result = (vec![1, 2, 3, 4]).into_iter().filter(predicate).collect::<Vec<i32>>();

assert_eq!(result, vec![3, 4]); // passes
```
