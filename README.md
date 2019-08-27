<p align="center"><img src="https://i.imgur.com/pqQLDFz.png" width="30%" /></p>

[![Build Status](https://dev.azure.com/jasonShin91/functional-programming-jargon.rs/_apis/build/status/JasonShin.functional-programming-jargon.rs?branchName=master)](https://dev.azure.com/jasonShin91/functional-programming-jargon.rs/_build/latest?definitionId=3&branchName=master)

The project is a library for functional programming in Rust.

* [fp-core.rs](#fp-corers)
    * [installation](#installation)
* [functional programming jargon in rust](#functional-programming-jargon-in-rust)

# fp-core.rs

A library for functional programming in Rust.

It contains purely functional data structures to supplement the functional programming needs alongside
with the Rust Standard Library.

## Installation 

Add below line to your Cargo.toml

```rust
fp-core = "0.1.8"
```

If you have [Cargo Edit](https://github.com/killercup/cargo-edit) you may simply

```bash
$ cargo add fp-core
```

# functional programming jargon in rust

Functional programming (FP) provides many advantages, and its popularity has been increasing as a result.
However, each programming paradigm comes with its own unique jargon and FP is no exception. By providing a glossary,
we hope to make learning FP easier.

Where applicable, this document uses terms defined in the [Fantasy Land spec](https://github.com/fantasyland/fantasy-land)
and Rust programming language to give code examples.

Also please be mindful that the project sometimes utilise FP related crates that are unfinished but contains
a specific typeclass or data type that is appropriate to give an explanation. Many of them are experimental and
are not suitable for production usage.

__Table of Contents__
<!-- RM(noparent,notop) -->

* [Arity](#arity)
* [Higher-Order Functions (HOF)](#higher-order-functions-hof)
* [Closure](#closure)
* [Partial Application](#partial-application)
* [Currying](#currying)
* [Auto Currying](#auto-currying)
* [Referential Transparency](#referential-transparency)
* [Lambda](#lambda)
* [Lambda Calculus](#lambda-calculus)
* [Purity](#purity)
* [Side effects](#side-effects)
* [Idempotent](#idempotent)
* [Function Composition](#function-composition)
* [Continuation](#continuation)
* [Point-Free Style](#point-free-style)
* [Predicate](#predicate)
* [Contracts](#contracts)
* [Category](#category)
* [Value](#value)
* [Constant](#constant)
* [Variance](#variance)
* [Higher Kinded Type](#higher-kinded-type-hkt)
* [Functor](#functor)
* [Pointed Functor](#pointed-functor)
* [Lifting](#lifting)
* [Equational Reasoning](#equational-reasoning)
* [Monoid](#monoid)
* [Monad](#monad)
* [Comonad](#comonad)
* [Applicative](#applicative)
* [Morphism](#morphism)
  * [Endomorphism](#endomorphism)
  * [Isomorphism](#isomorphism)
  * [Homomorphism](#homomorphism)
  * [Catamorphism](#catamorphism)
  * [Hylomorphism](#hylomorphism)
  * [Anamorphism](#anamorphism)
* [Setoid](#setoid)
* [Ord](#ord)
* [Semigroup](#semigroup)
* [Foldable](#foldable)
* [Lens](#lens)
* [Type Signature](#type-signature)
* [Algebraic data type](#algebraic-data-type)
  * [Sum Type](#sum-type)
  * [Product Type](#product-type)
* [Option](#option)
* [Functional Programming References](#functional-programming-references)
* [Function Programming development in Rust Language](#functional-programming-development-in-rust-language)
* [My thought on this project](#my-thought-on-this-project)

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
    xs.into_iter().filter(predicate).collect::<Vec<i32>>()
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
let add_to = |x: i32| move |y: i32| x + y;
```

We can call `add_to` with a number and get back a function with a baked-in `x`. Notice that we also need to move the ownership of the x to the internal lambda.

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
    move |y| x + y
}

let add5 = add(5);
add5(10); // 15
```

__Further reading__
* [Currying in Rust](https://hashnode.com/post/currying-in-rust-cjpfb0i2z00cm56s2aideuo4z)

## Auto Currying

Transforming a function that takes multiple arguments into one that if given less than its
correct number of arguments returns a function that takes the rest. When the function gets the correct number of
arguments it is then evaluated.

Although Auto Currying is not possible in Rust right now, there is a debate on this issue on the Rust forum:
https://internals.rust-lang.org/t/auto-currying-in-rust/149/22

## Referential Transparency

An expression that can be replaced with its value without changing the behavior of the program is said to be referentially transparent.

Say we have function greet:

```rust
let greet = || "Hello World!";
```

Any invocation of `greet()` can be replaced with `Hello World!` hence greet is referentially transparent.


## Lambda

An anonymous function that can be treated like a value.

```rust
fn  increment(i: i32) -> i32 { i + 1 }

let closure_annotated = |i: i32| { i + 1 };
let closure_inferred = |i| i + 1;
```

Lambdas are often passed as arguments to Higher-Order functions.
You can assign a lambda to a variable, as shown above.

## Lambda Calculus

A branch of mathematics that uses functions to create a [universal model of computation](https://en.wikipedia.org/wiki/Lambda_calculus).

This is in contrast to a [Turing machine](https://www.youtube.com/watch?v=dNRDvLACg5Q), an equivalent model.

Lambda calculus has three key components: variables, abstraction, and application. A variable is just some
symbol, say `x`. An abstraction is sort of a function: it binds variables into "formulae". Applications
are function calls. This is meaningless without examples.

The identity function (`|x| x` in rust) looks like `\ x. x` in most literature (`\` is a Lambda where Latex
or Unicode make it available). It is an abstraction. If `1` were a value we could use, `(\ x. x) 1` would
be an application (and evaluating it gives you `1`).

But there's more...

__Computation in Pure Lambda Calculus__

Let's invent booleans. `\ x y. x` can be true and `\ x y. y` can be false.
If so, `\ b1 b2. b1(b2,(\x y. y))` is `and`. Let's evaluate it to show how:

| `b1` | `b2` | Their `and` |
| --- | --- | --- |
| `\ x y. x` | `\x y. x` | `\x y. x` |
| `\ x y. x` | `\x y. y` | `\x y. y` |
| `\ x y. y` | `\x y. y` | `\x y. y` |
| `\ x y. y` | `\x y. x` | `\x y. y` |

I'll leave `or` as an exercise. Furthermore, `if` can now be implemented: `\c t e. c(t, e)` where `c` is the condition, `t`
the consequent (`then`) and `e` the else clause.

[SICP leaves numbers as an exercise.](https://mitpress.mit.edu/sites/default/files/sicp/full-text/book/book-Z-H-14.html#%_idx_1474)
They define 0 as `\f . \x. x` and adding one as `\n. \f. \x. f(n(f)(x))`.
That isn't even ASCII art, so let's add: `0 + 1`:

```
(\n. \f. \x. f(n(f)(x)))(\f. \x. x) = \f. \x. f((\x'. x')(x)) = \f. \x. f(x)
```

Basically, the number of `f`s in the expression is the number. I'll leave figuring out larger numbers as a exercise.
With patience, you can show that `\f. \x. f(f(x))` is two.  This will help with addition: `\n m. \f. \x. n(m(f)(x))`
should add two numbers. Let's make 4:

```
(\n m. \f. \x. n(f)(m(f)(x)))(\f. x. f(f(x)), \f. \x. f(f(x)))
  = \f. \x. (\f'. \x'. f'(f'(x')))(f)((\f'. \x'. f'(f'(x')))(f)(x))
  = \f. \x. (\x'. f(f(x')))(f(f(x')))
  = \f. \x. f(f(f(f(x))))
```

Multiplication is harder and there's better
[exposition on Wikipedia](https://en.wikipedia.org/wiki/Church_encoding#Calculation_with_Church_numerals).
Another good reference is [on stackoverflow](https://stackoverflow.com/questions/3077908/church-numeral-for-addition).

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
let sort = |x: Vec<i32>| -> Vec<i32> {
    let mut x = x;
    x.sort();
    x
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
    x.abs()
};

let x: i32 = 10;
let result = abs(abs(x));
assert_eq!(result, x); // passes
```

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

At any given point in a program, the part of the code that's yet to be executed is known as a continuation.

```rust
let print_as_string = |num: i32| println!("Given {}", num);

let add_one_and_continue = |num: i32, cc: fn(i32)| {
    let result = num + 1;
    cc(result)
};

add_one_and_continue(1, print_as_string); // Given 2
```

Continuations are often seen in asynchronous programming when the program needs to wait to receive data before it can continue.
 The response is often passed off to the rest of the program, which is the continuation, once it's been received.

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

## Contracts

A contract specifies the obligations and guarantees of the behavior from a function or expression at runtime.
This acts as a set of rules that are expected from the input and output of a function or expression,
and errors are generally reported whenever a contract is violated.

```rust
let contract = | x: &i32 | -> bool {
    x > &10
};

let add_one = | x: &i32 | -> Result<i32, String> {
    if contract(x) {
        return Ok(x + 1);
    }
    Err("Cannot add one".to_string())
};
```

Then you can use `add_one` like

```rust
let expected = 12;
match add_one(&11) {
    Ok(x) => assert_eq!(x, expected),
    _ => panic!("Failed!")
}
```

## Category

A category in category theory is a collection of objects and morphisms between them. In programming, typically types
act as the objects and functions as morphisms.

To be a valid category 3 rules must be met:

1. There must be an identity morphism that maps an object to itself.
    Where `a` is an object in some category,
    there must be a function from `a -> a`.
2. Morphisms must compose.
    Where `a`, `b`, and `c` are objects in some category,
    and `f` is a morphism from `a -> b`, and `g` is a morphism from `b -> c`;
    `g(f(x))` must be equivalent to `(g • f)(x)`.
3. Composition must be associative
    `f • (g • h)` is the same as `(f • g) • h`

Since these rules govern composition at very abstract level, category theory is great at uncovering new ways of composing things.
In particular, many see this as another foundation of mathematics (so, everything would be a category from this view of math).
Various definitions in this guide are related to category theory since this approach applies elegantly to functional programming.

__Examples of categories__

When one specifies a category, the objects and morphisms are essential. Additionally, showing that the rules are met is nice
though usually left to the reader as an exercise.

* Any type and pure functions on the type. (Note, we require purity since side-effects affect associativity (the third rule).)

These examples come up in mathematics:

* 1: the category with 1 object and its identity morphism.
* Monoidal categories: [monoids are defined later](#monoids) but any monoid is a category with 1 object and many morphisms
  from the object to itself. (Yes, there is also a category of monoids -- this is not that -- this example is that any monoid is its own
  category.)

__Further reading__

* [Category Theory for Programmers](https://bartoszmilewski.com/2014/10/28/category-theory-for-programmers-the-preface/)
* [Awodey's introduction](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.211.4754&rep=rep1&type=pdf) for those who like math.

## Value

Anything that can be assigned to a variable.

```rust
let a = 5;
let b = vec![1, 2, 3];
let c = "test";
```

## Constant

A variable that cannot be reassigned once defined.

```rust
let a = 5;
a = 3; // error!
```

Constants are [referentially transparent](#referential-transparency).
That is, they can be replaced with the values that they represent without affecting the result.

## Variance

Variance in functional programming refers to subtyping between more complex types related to subtyping between
their components.

Unlike other usage of variance in [Object Oriented Programming like Typescript or C#](https://medium.com/@michalskoczylas/covariance-contravariance-and-a-little-bit-of-typescript-2e61f41f6f68)
or [functional programming language like Scala or Haskell](https://medium.com/@wiemzin/variances-in-scala-9c7d17af9dc4)

Variance in Rust is used during the type checking against type and lifetime parameters. Here are examples:
- By default, all lifetimes are co-variant except for `'static` because it outlives all others
- `'static` is always contra-variant to others regardless of where it appears or used
- It is `in-variant` if you use `Cell<T>` or `UnsafeCell<T>` in `PhatomData`

**Further Reading**

- https://github.com/rust-lang/rustc-guide/blob/master/src/variance.md
- https://nearprotocol.com/blog/understanding-rust-lifetimes/

## Higher Kinded Type (HKT)

Rust does not support Higher Kinded Types [yet](https://github.com/rust-lang/rust/issues/8922). First of all, HKT is a
type with a "hole" in it, so you can declare a type signature such as `trait Functor<F<A>>`.

Although Rust lacks in a native support for HKT, we always have a walk around called [Lightweight Higher Kinded Type](https://www.cl.cam.ac.uk/~jdy22/papers/lightweight-higher-kinded-polymorphism.pdf)

An implementation example of above theory in Rust would look like below:

```rust
pub trait HKT<A, B> {
    type URI;
    type Target;
}

// Lifted Option
impl<A, B> HKT<A, B> for Option<A> {
    type URI = Self;
    type Target = Option<B>;
}
```

Higher Kinded Type is crucial for functional programming in general.

**Further Reading**

- https://gist.github.com/CMCDragonkai/a5638f50c87d49f815b8
- https://www.youtube.com/watch?v=ERM0mBPNLHc

## Functor

An object that implements a map function which,
while running over each value in the object to produce a new functor of the same type, adheres to two rules:

__Preserves identity__

```
object.map(x => x) ≍ object
```

__Composable__

```
object.map(compose(f, g)) ≍ object.map(g).map(f)
```

(`f`, `g` are arbitrary functions)

For example, below can be considered as a functor-like operation

```rust
let v: Vec<i32> = vec![1, 2, 3].into_iter().map(| x | x + 1).collect();

assert_eq!(v, vec![2, 3, 4]); // passes while mapping the original vector and returns a new vector
```

While leveraging the [HKT implementation](#higher-kinded-type-hkt), You can define a trait that represents Functor like below

```rust
pub trait Functor<A, B>: HKT<A, B> {
    fn fmap<F>(self, f: F) -> <Self as HKT<A, B>>::Target
        where F: FnOnce(A) -> B;
}
```

Then use it against a type such as [Option](#https://doc.rust-lang.org/std/option/index.html) like

```rust
impl<A, B> Functor<A, B> for Option<A> {
    fn fmap<F>(self, f: F) -> Self::Target
        where
            F: FnOnce(A) -> B
    {
        self.map(f)
    }
}

// This conflicts with the above, so isn't tested
// below and is commented out. TODO: add a careful
// implementation that makes sure Optional and this
// don't conflict.
impl<A, B, T> HKT<A, B> for T
where
    T: Sized + Iterator<Item = A>,
    U: Sized + Iterator<Item = B>,
{
    type URI = Self;
    type Target = U;
}

impl<A, B, T> Functor<A, B> for T
where
    T: Iterator<Item = A>,
{
    fn fmap<F>(self, f: F) -> Self::Target
    where
        F: FnOnce(A) -> B,
        A: Sized,
        B: Sized,
    {
        self.map(f)
    }
}

#[test]
fn test_functor() {
    let z = Option::fmap(Some(1), |x| x + 1).fmap(|x| x + 1); // Return Option<B>
    assert_eq!(z, Some(3)); // passes
}
```

__The Underlying Math__

The confusing fact is that functors are morphisms in the category of categories. Really, this means that
a functor from category `C` into `D` preserves properties of the category, so that the data is somewhat
preserved.

Technically, every category has a functor into the simplest (non-empty) category (1): since the category `1` just
has one object and one function, map all the objects and functions in whatever category you start from into the
thing in `1`. So, data isn't quite preserved in a "nice" sense. Such functors are called forgetful sometimes as
they drop structure.

However, less forgetful examples provide more insight and empower useful statements about types.
Unfortunately, these are rather heavy-handed in the mathematics they evoke.


## Pointed Functor

An object with an of function that puts any single value into it.

```rust
#[derive(Debug, PartialEq, Eq)]
enum Maybe<T> {
    Nothing,
    Just(T),
}


impl<T> Maybe<T> {
    fn of(x: T) -> Self {
        Maybe::Just(x)
    }
}
```

Then use it like

```rust
let pointed_functor = Maybe::of(1);

assert_eq!(pointed_functor, Maybe::Just(1));
```

## Lifting

Lifting in functional programming typically means to lift a function into a context (a Functor or Monad).
For example, give a function `a -> b` and lift it into a `List` then the signature would
look like `List[a] -> List[b]`.

**Further Reading**
- https://wiki.haskell.org/Lifting
- https://stackoverflow.com/questions/43482772/difference-between-lifting-and-higher-order-functions
- https://stackoverflow.com/questions/2395697/what-is-lifting-in-haskell/2395956

## Equational Reasoning

When an application is composed of expressions and devoid of side effects, truths about the system can be derived from the parts.

## Monoid

An set with a binary function that "combines" pairs from that set into another element of the set.

One simple monoid is the addition of numbers:

```rust
1 + 1
// i32: 2
```

In this case numbers are the set and `+` is the function.

An "identity" value must also exist that when combined with a value doesn't change it.

The identity value for addition is `0`.

```rust
1 + 0
// i32: 1
```

It's also required that the grouping of operations will not affect the result (associativity):

```rust
1 + (2 + 3) == (1 + 2) + 3
// bool: true
```

Array concatenation also forms a monoid:

```rust
[vec![1, 2, 3], vec![4, 5, 6]].concat();
// Vec<i32>: vec![1, 2, 3, 4, 5, 6]
```

The identity value is empty array `[]`

```rust
[vec![1, 2], vec![]].concat();
// Vec<i32>: vec![1, 2]
```

If identity and compose functions are provided, functions themselves form a monoid:

```rust
fn identity<A>(a: A) -> A {
    a
}
```

`foo` is any function that takes one argument.

```rust
compose(foo, identity) ≍ compose(identity, foo) ≍ foo
```

We can express Monoid as a Rust trait and the type signature would look like below

```rust
use crate::applicative_example::Applicative;

trait Empty<A> {
    fn empty() -> A;
}

trait Monoid<A, F, B>: Empty<A> + Applicative<A, F, B>
where
    F: FnOnce(A) -> B,
{
}
```

According to Fantasy Land Specification, Monoid should implement `Empty` and `Applicative`.

## Monad

A [Monad](https://github.com/fantasyland/fantasy-land#monad) is a trait that implements `Applicative` and `Chain` specifications. `chain` is
like `map` except it un-nests the resulting nested object.

First, `Chain` type can be implemented like below:

```rust
pub trait Chain<A, B>: HKT<A, B> {
    fn chain<F>(self, f: F) -> <Self as HKT<A, B>>::Target
        where F: FnOnce(A) -> <Self as HKT<A, B>>::Target;
}

impl<A, B> Chain<A, B> for Option<A> {
    fn chain<F>(self, f: F) -> Self::Target
        where F: FnOnce(A) -> <Self as HKT<A, B>>::Target {
        self.and_then(f)
    }
}
```

Then `Monad` itself can simply derive `Chain` and `Applicative`

```rust
pub trait Monad<A, F, B>: Chain<A, B> + Applicative<A, F, B>
    where F: FnOnce(A) -> B {}

impl<A, F, B> Monad<A, F, B> for Option<A>
    where F: FnOnce(A) -> B {}

#[test]
fn monad_example() {
    let x = Option::of(Some(1)).chain(|x| Some(x + 1));
    assert_eq!(x, Some(2)); // passes
}
```

`pure` is also known as `return` in other functional languages. `flat_map` is also known as `bind` in other languages.

Importantly, it is worth noting that monads are a rather advanced topic in category theory. In fact, they are called
triples by some as they involve adjoint functors and their unit -- both of which are rare to see in functional programming.
The meme is to think of a monad as a burrito with "pure" being the act of taking a tortilla (the empty burrito) and
adding ingredients using "chain".

The purely mathematical presentation of monads does not look anything like this, but there is an equivalence.

## Comonad

An object that has `extract` and `extend` functions.

```rust
trait Extend<A, B>: Functor<A, B> + Sized {
    fn extend<W>(self, f: W) -> <Self as HKT<A, B>>::Target
    where
        W: FnOnce(Self) -> B;
}

trait Extract<A> {
    fn extract(self) -> A;
}

trait Comonad<A, B>: Extend<A, B> + Extract<A> {}
```

Then we can implement these types for Option

```rust
impl<A, B> Extend<A, B> for Option<A> {
    fn extend<W>(self, f: W) -> Self::Target
    where
        W: FnOnce(Self) -> B,
    {
        self.map(|x| f(Some(x)))
    }
}

impl<A> Extract<A> for Option<A> {
    fn extract(self) -> A {
        self.unwrap() // is there a better way to achieve this?
    }
}
```

Extract takes a value out of a comonad.

```rust
Some(1).extract(); // 1
```

Extend runs a function on the Comonad.

```rust
Some(1).extend(|co| co.extract() + 1); // Some(2)
```

This can be thought of as the reverse of a monad. In fact, this is called
the "dual" in category theory. (Basically, if you know what `A` is, a `coA`
is everything in `A`'s definition with the arrows reversed.)

## Applicative

An applicative functor is an object with an `ap` function. `ap` applies a function in the object to a value in another
object of the same type. Given a pure program `g: (b: A) -> B`, we must lift it to `g: (fb: F<A>) -> F<B>`. In order to achieve
this, we will introduce another [higher kinded type](#higher-kinded-type-hkt), called `HKT3` that is capable of doing this.

For this example, we will use Option datatype.

```rust
trait HKT3<A, B, C> {
    type Target2;
}

impl<A, B, C> HKT3<A, B, C> for Option<A> {
    type Target2 = Option<B>;
}
```

Since Applicative implements Apply for `ap`  and `Pure` for `of` according to [Fantasy Land specification](https://github.com/fantasyland/fantasy-land#applicative)
we must implement the types like below:

```rust

// Apply
trait Apply<A, F, B> : Functor<A, B> + HKT3<A, F, B>
    where F: FnOnce(A) -> B,
{
    fn ap(self, f: <Self as HKT3<A, F, B>>::Target2) -> <Self as HKT<A, B>>::Target;
}

impl<A, F, B> Apply<A, F, B> for Option<A>
    where F: FnOnce(A) -> B,
{
    fn ap(self, f: Self::Target2) -> Self::Target {
        self.and_then(|v| f.map(|z| z(v)))
    }
}

// Pure
trait Pure<A>: HKT<A, A> {
    fn of(self) -> <Self as HKT<A, A>>::Target;
}

impl<A> Pure<A> for Option<A> {
    fn of(self) -> Self::Target {
        self
    }
}

// Applicative
trait Applicative<A, F, B> : Apply<A, F, B> + Pure<A>
    where F: FnOnce(A) -> B,
{
} // Simply derives Apply and Pure

impl<A, F, B> Applicative<A, F, B> for Option<A>
    where F: FnOnce(A) -> B,
{
}
```

Then we can use Option Applicative like this:

```rust
let x = Option::of(Some(1)).ap(Some(|x| x + 1));
assert_eq!(x, Some(2));
```

## Morphism

A function that preserves the structure of its domain.
See [the category definition](#category) from more.

The first few (endomorphism, homomorphism, and isomorphism) are easier to
understand than the rest. The rest require the notion of an F-algebra.
The simpler Haskell declarations are listed in [the wikipedia on paramorphisms](https://en.wikipedia.org/wiki/Paramorphism)
but the notions have yet to be extended to more general category theory.
Briefly, the view of F-algebras is to take the set-theoretic definition of algebraic
objects and redefined them on a purely category theoretic footing: to move the ideas
away from sets containing elements to collections of objects with morphisms.
However, some of the ideas here have yet to be generalised into this movement.

### Endomorphism

A function where the input type is same as the output.

```rust
// uppercase :: &str -> String
let uppercase = |x: &str| x.to_uppercase();

// decrement :: i32 -> i32
let decrement = |x: i32| x - 1;
```

### Isomorphism

A pair of transformations between 2 types of objects that is invertible.

For example, 2D coordinates could be stored as a i32 vector [2,3] or a struct {x: 2, y: 3}.

```rust
#[derive(PartialEq, Debug)]
struct Coords {
    x: i32,
    y: i32,
}

let pair_to_coords = | pair: (i32, i32) | Coords { x: pair.0, y: pair.1 };
let coords_to_pair = | coords: Coords | (coords.x, coords.y);
assert_eq!(
    pair_to_coords((1, 2)),
    Coords { x: 1, y: 2 },
); // passes
assert_eq!(
    coords_to_pair(Coords { x: 1, y: 2 }),
    (1, 2),
); // passes
```

Isomorphisms are critical in making structures identical. Since we know that the struct above is
identical to a pair, all the functions that exist on the pair can exist on the struct. If `f`
is the isomorphism and `g` and endomorphism on the codomain: `f^{-1} g f` would extend `g`
to apply on the domain.

### Homomorphism

A homomorphism is just a structure preserving map. It is the older term of morphism.
In fact, a functor is just a homomorphism between categories as it preserves the original category's structure under the mapping.

```rust
assert_eq!(A::of(f).ap(A::of(x)), A::of(f(x))); // passes
assert_eq!(
    Either::of(|x: &str| x.to_uppercase(x)).ap(Either::of("oreos")),
    Either::of("oreos".to_uppercase),
); // passes
```

### Catamorphism

A `reduceRight` function that applies a function against an accumulator and each value of the array (from right-to-left)
to reduce it to a single value.

```rust
let sum = |xs: Vec<i32>| xs.iter().fold(0, |mut sum, &val| { sum += val; sum });

assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
```

### Anamorphism

An `unfold` function. An `unfold` is the opposite of `fold` (`reduce`). It generates a list from a single value.

```rust
let count_down = unfold((8_u32, 1_u32), |state| {
    let (ref mut x1, ref mut x2) = *state;

    if *x1 == 0 {
        return None;
    }

    let next = *x1 - *x2;
    let ret = *x1;
    *x1 = next;

    Some(ret)
});

assert_eq!(
    count_down.collect::<Vec<u32>>(),
    vec![8, 7, 6, 5, 4, 3, 2, 1],
);
```

### Hylomorphism

The combination of anamorphism and catamorphism.

### Apomorphism

It's the opposite of paramorphism, just as anamorphism is the opposite of catamorphism.
Whereas with paramorphism, you combine with access to the accumulator and what has been accumulated,
apomorphism lets you unfold with the potential to return early.

## Setoid

This is a set with an equivalence relation.

An object that has an `equals` function which can be used to compare other objects of the same type.

It must obey following rules to be `Setoid`

1. `a.equals(a) == true` (reflexivity)
2. `a.equals(b) == b.equals(a)` (symmetry)
3. `a.equals(b)` and `b.equals(c)` then `a.equals(c)` (transitivity)

Make a Vector a setoid:

Note that I am treating `Self` / `self` like `a`.

```rust
trait Setoid {
    fn equals(&self, other: &Self) -> bool;
}

impl Setoid for Vec<i32> {
    fn equals(&self, other: &Self) -> bool {
        self.len() == other.len()
    }
}

assert_eq!(vec![1, 2].equals(&vec![1, 2]), true); // passes
```

In Rust standard library, it already provides [Eq](https://doc.rust-lang.org/std/cmp/trait.Eq.html), which
resembles Setoid that was discussed in this section. Also [Eq](https://doc.rust-lang.org/std/cmp/trait.Eq.html)
has `equals` implementations that covers a range of data structures that already exist in Rust.

## Ord

An object or value that implements Ord specification, also implements [Setoid](#setoid) specification.

The Ord object or value must satisfy below rules for all `a`, `b` or `c`:

1. totality: `a <= b` or `b <= a`
2. antisymmetric: `a <= b` and `b <= a`, then `a == b`
3. transivity: `a <= b` and `b <= c`, then `a <= c`

Rust documentation for Ord can be found here [Ord](https://doc.rust-lang.org/std/cmp/trait.Ord.html)

## Semigroup

An object that has a `combine` function that combines it with another object of the same type.

It must obey following rules to be `Semigroup`

1. `a.add(b).add(c)` is equivalent to `a.add(b.add(c))` (associativity)

```rust
use std::ops::Add;

pub trait Semigroup<M>: Add<M> {
}

assert_eq!(
    vec![1, 2].add(&vec![3, 4]),
    vec![1, 2, 3, 4],
); // passes

assert_eq!(
    a.add(&b).add(&c),
    a.add(&b.add(&c)),
); // passes
```

## Foldable

An object that has a `foldr/l` function that can transform that object into some other type. We are using `rats` to give
an example but the crate only implements `fold_right`.

`fold_right` is equivalent to Fantasy Land Foldable's `reduce`, which goes like:

`fantasy-land/reduce :: Foldable f => f a ~> ((b, a) -> b, b) -> b`

```rust
use rats::foldable::Foldable;
use rats::kind::IntoKind;
use rats::kinds::VecKind;

let k = vec![1, 2, 3].into_kind();
let result = VecKind::fold_right(k, 0, | (i, acc) | i + acc);
assert_eq!(result, 6);
```

If you were to implement `Foldable` manually, the trait of it would look like below

```rust
use crate::hkt::HKT;
use crate::monoid::Monoid;

pub trait Foldable<A, B>: HKT<A, B> {
    fn reduce<F>(b: B, ba: F) -> <Self as HKT<A, B>>::Target
    where
        F: FnOnce(B, A) -> (B, B);

    fn fold_map<M, N, F>(m: M, fa: F) -> M
    where
        M: Monoid<N>,
        F: FnOnce(<Self as HKT<A, B>>::URI) -> M;

    fn reduce_right<F>(b: B, f: F) -> <Self as HKT<A, B>>::Target
    where
        F: FnOnce(A, B) -> (B, B);
}
```

## Lens

A lens is a type that pairs a getter and a non-mutating setter for some other data structure.

```rust
trait Lens<S, A> {
    fn over(s: &S, f: &Fn(Option<&A>) -> A) -> S {
        let result: A = f(Self::get(s));
        Self::set(result, &s)
    }
    fn get(s: &S) -> Option<&A>;
    fn set(a: A, s: &S) -> S;
}

#[derive(Debug, PartialEq, Clone)]
struct Person {
    name: String,
}

#[derive(Debug)]
struct PersonNameLens;

impl Lens<Person, String> for PersonNameLens {
    fn get(s: &Person) -> Option<&String> {
       Some(&s.name)
    }

    fn set(a: String, s: &Person) -> Person {
        Person {
            name: a,
        }
    }
}
```

Having the pair of get and set for a given data structure enables a few key features.

```rust
let e1 = Person {
    name: "Jason".to_string(),
};
let name = PersonNameLens::get(&e1);
let e2 = PersonNameLens::set("John".to_string(), &e1);
let expected = Person {
    name: "John".to_string()
};
let e3 = PersonNameLens::over(&e1, &|x: Option<&String>| {
    match x {
        Some(y) => y.to_uppercase(),
        None => panic!("T_T") // lol...
    }
});

assert_eq!(*name.unwrap(), e1.name); // passes
assert_eq!(e2, expected); // passes
assert_eq!(e3, Person { name: "JASON".to_string() }); // passes
```

Lenses are also composable. This allows easy immutable updates to deeply nested data.

```rust
struct FirstLens;

impl<A> Lens<Vec<A>, A> for FirstLens {
  fn get(s: &Vec<A>) -> Option<&A> {
     s.first()
  }

  fn set(a: A, s: &Vec<A>) -> Vec<A> {
      unimplemented!() // Nothing to set in FirstLens
  }
}

let people = vec![Person { name: "Jason" }, Person { name: "John" }];
Lens::over(composeL!(FirstLens, NameLens), &|x: Option<&String>| {
  match x {
      Some(y) => y.to_uppercase(),
      None => panic!("T_T")
  }
}, people); // vec![Person { name: "JASON" }, Person { name: "John" }];
```

**Further Reading**

- [A Little Lens Starter](https://www.schoolofhaskell.com/school/to-infinity-and-beyond/pick-of-the-week/a-little-lens-starter-tutorial)
- [Monocle Scala](https://scalac.io/scala-optics-lenses-with-monocle/)

## Type Signature

Every function in Rust will indicate the types of their arguments and return values.

```rust
// add :: i32 -> i32 -> i32
fn add(x: i32) -> impl Fn(i32)-> i32 {
    move |y| x + y
}

// increment :: i32 -> i32
fn increment(x: i32) -> i32 {
    x + 1
}
```

If a function accepts another function as an argument it is wrapped in parentheses.

```rust
// call :: (a -> b) -> a -> b
fn call<A, B>(f: &Fn(A) -> B) -> impl Fn(A) -> B + '_ {
    move |x| f(x)
}
```

The letters `a`, `b`, `c`, `d` are used to signify that the argument can be of any type.
The following version of map takes `a` function that transforms `a` value of some type `a` into another type `b`,
an array of values of type `a`, and returns an array of values of type `b`.

```rust
// map :: (a -> b) -> [a] -> [b]
fn map<A, B>(f: &Fn(A) -> B) -> impl Fn(A) -> B + '_ {
    move |x| f(x)
}
```

**Further Reading**

- [Mostly Adequate Guide](https://drboolean.gitbooks.io/mostly-adequate-guide-old/content/ch7.html#tales-from-the-cryptic)
- [What is Hindley-Milner?](https://stackoverflow.com/questions/399312/what-is-hindley-milner/399392#399392)

## Algebraic data type

A composite type made from putting other types together. Two common classes of algebraic types are [sum](#sum-type) and [product](#product-type).

### Sum Type

A Sum type is the combination of two types together into another one.
It is called sum because the number of possible values in the result type is the sum of the input types.

Rust has `enum` that literally represent `sum` in ADT.

```rust
enum WeakLogicValues {
   True(bool),
   False(bool),
   HalfTrue(bool),
}
// WeakLogicValues = bool + otherbool + anotherbool
```

### Product Type

A product type combines types together in a way you're probably more familiar with:

```rust
struct Point {
    x: i32,
    y: i32,
}
// Point = i32 x i32
```

It's called a product because the total possible values of the data structure is the product of the different values.
Many languages have a tuple type which is the simplest formulation of a product type.

See also [Set Theory](https://en.wikipedia.org/wiki/Set_theory)

**Further Reading**

- [ADT in 4 different languages](https://blog.softwaremill.com/algebraic-data-types-in-four-languages-858788043d4e)
- [What are Sum Product and Pi Types](https://manishearth.github.io/blog/2017/03/04/what-are-sum-product-and-pi-types/)

## Option

Option is a [sum type](#sum-type) with two cases often called Some and None.

Option is useful for composing functions that might not return a value.

```rust
let mut cart = HashMap::new();
let mut item = HashMap::new();
item.insert(
    "price".to_string(),
    12
);
cart.insert(
    "item".to_string(),
    item,
);

fn get_item(cart: &HashMap<String, HashMap<String, i32>>) -> Option<&HashMap<String, i32>> {
    cart.get("item")
}

fn get_price(item: &HashMap<String, i32>) -> Option<&i32> {
    item.get("price")
}

```

Use [and_then](https://doc.rust-lang.org/std/option/enum.Option.html#method.and_then) or [map](https://doc.rust-lang.org/std/option/enum.Option.html#method.map) to sequence functions that return Options

```rust
fn get_nested_price(cart: &HashMap<String, HashMap<String, i32>>) -> Option<&i32> {
    return get_item(cart).and_then(get_price);
}

let price = get_nested_price(&cart);

match price {
    Some(v) => assert_eq!(v, &12),
    None => panic!("T_T"),
}
```

`Option` is also known as `Maybe`. `Some` is sometimes called `Just`. `None` is sometimes called `Nothing`.

## Functional Programming references

- [Scala with Cats](https://underscore.io/books/scala-with-cats/)
- [Haskell Programming](http://haskellbook.com/)
- [Category Theory for Programmers](https://bartoszmilewski.com/2014/10/28/category-theory-for-programmers-the-preface/)
- [Category Theory Notes](http://cheng.staff.shef.ac.uk/catnotes/categorynotes-cheng.pdf)

## Functional Programming development in Rust Language

- [Higher Kinded Polymorphism RFC](https://github.com/rust-lang/rfcs/issues/324)
- [Currying in Rust](https://internals.rust-lang.org/t/currying-in-rust/10326)
- [Auto-Currying in Rust](https://internals.rust-lang.org/t/auto-currying-in-rust/149)

## My thought on this project

Please be mindful that the project does not fully cover every single definitions with code examples since it does not worth
investing too much time in a glossary project. I have put extra effort in adding complementary external references that
you can check out for further study.
