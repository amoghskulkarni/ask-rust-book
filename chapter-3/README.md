# Variables and mutability

- Variables are declared using the `let` keyword
- All variables are immutable by default, can be made mutable using the `mut` keyword
- Program throws a **compile-time error** if you try to mutate an immutable variable

```rust
let x = 5;
println!("The value of x is: {x}");
x = 6; // Throws error - error[E0384]: cannot assign twice to immutable variable `x`
println!("The value of x is: {x}");
```

- A more detailed explanation about any error can be obtained from `rustc`
  - e.g. `rustc --explain E0384`

## Constants

- Constants can be declared using the word `const` and they are ALWAYS read-only
- e.g. `const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;`
- The type of their value MUST be annotated, since it is always known at the time of declaring them
- Can be declared in any scope, even global
  - `const` can be used in the global scope, and `let` can only be used in a function
- Can only be set to a constant expression at the compile=time, not a runtime computation
- Rust’s naming convention for constants is to use all uppercase with underscores between words.

## Shadowing

- A variable if declared with tbe same name, using `let` keyword
- Different than making `mut`
  - `mut` mutates the same variable, this creates a new variable which "shadows" the older one
  - If `mut` variable is declared, it has to be given the value of the same datatype.

# Datatypes

- Rust is a _statically_ typed language, so the compiler has to know the datatype of every variable at **_compile time_**

## Scalar types

- Scalar types represent a single value 
- Typical types are available (`u8` to `u128` and `usize` depending on architecture, similary `i8` to `i128` and `isize`, `bool`, `char`, `f32` i.e. single-precision and `f64` i.e. double precision)
- `char` literals with single quotes, as opposed to string literals, which use double quotes

## Compound types 

- Can represent multiple values, e.g. tuples and arrays 
- Tuples can be destructured, and also `.` notation for accessing tuple elements is allowed (like C structures)
  ```rust
  let tup = (500, 6.4, 1);
  let (x, y, z) = tup; // Destructuring 
  println!("{x.1}") // dot-notation, prints 6.4
  ```
- All elements in an array must have the same data type (i.e. Python like lists are not allowed)
- Arrays cannot grow or shrink dynamically, _vectors_ can. When in doubt, use a vector
- Arrays and their types can be defined with semicolon notation 
  ```rust
  let a: [i32; 5] = [1, 2, 3, 4, 5]  // Each element is of i32 type
  let b: [i32; 5] = [42; 5]  // same as [42, 42, 42, 42, 42]
  ```
- Arrays are useful when you want your data allocated on the stack rather than the heap
- Program "panics" (i.e. throws a runtime error) when an out-of-bound array element is accessed 