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

### Constants

- Constants can be declared using the word `const` and they are ALWAYS read-only
- e.g. `const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;`
- The type of their value MUST be annotated, since it is always known at the time of declaring them
- Can be declared in any scope, even global
  - `const` can be used in the global scope, and `let` can only be used in a function
- Can only be set to a constant expression at the compile=time, not a runtime computation
- Rust’s naming convention for constants is to use all uppercase with underscores between words.

### Shadowing

- A variable if declared with tbe same name, using `let` keyword
- Different than making `mut`
  - `mut` mutates the same variable, this creates a new variable which "shadows" the older one
  - If `mut` variable is declared, it has to be given the value of the same datatype.

# Datatypes

- Rust is a _statically_ typed language, so the compiler has to know the datatype of every variable at **_compile time_**

### Scalar types

- Scalar types represent a single value 
- Typical types are available (`u8` to `u128` and `usize` depending on architecture, similary `i8` to `i128` and `isize`, `bool`, `char`, `f32` i.e. single-precision and `f64` i.e. double precision)
- `char` literals with single quotes, as opposed to string literals, which use double quotes

### Compound types 

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

# Functions

- `fn` keyword for defining a function, curly braces are expected on the same line 
- `snake_case` naming convention for functions

### Parameters

Parameters (or args, used interchangeably) **_MUST_** have types in their function signature

```rust
// Compile time error 
fn another_function(x) {
  println!("The value of x is: {x}");
}

// Compiles OK 
fn yet_another_function(x: i32) {
  println!("The value of x is: {x}");
}
```

### Statements and Expressions 

- Statements: Instructions that perform some action and **do not** return a value
- Expressions: evaluate to a resultant value
- `let y = 6;` is a statement 
- Function definitions are also statements; the entire preceding example is a statement in itself

  ```rust
  fn main() {
    let x = (let y = 6);
  }
  ```

  The `let y = 6` statement does not return a value, so there isn’t anything for x to bind to
- Expressions (like math operations, e.g. `6 + 4`) evaluate to a value

  ```rust
  let y = {
    let x = 3;
    x + 1 // No semicolon, because it will be a statement otherwise 
  }
  ```
  The above code compiles, because `{...}` is a valid expression, and can be assigned to the variable y. 

  > [!CAUTION] 
  > Note that the `x + 1` line doesn’t have a semicolon at the end, which is unlike most of the lines you’ve seen so far. Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.

### Functions with return values 

- Don't have to name return values in the signature, but have to state their type explicitly 
  ```rust
  fn five() -> i32 { 
    5
  }
  ```
- Can return early using the keyword `return` 
- `()` is returned when a function doesn't return an expression (probably similar to `void` or `None`), example below 

  ```rust
  fn main() {
    let x = plus_one(5);
    println!("The value of x is: {x}");
  }

  fn plus_one(x: i32) -> i32 {
      x + 1;
  }
  ```