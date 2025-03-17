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

# Comments

Usual C-style comment syntax.

- `//` for single-line comment
- `/* */` for block comments 
- Docstring comments are supported 
  ```rust
  /*
   * This is a docstring comment
   */
  fn some_function () {
    // ...
  }
  ```

# Control flow 

### Branching (`if` expressions)

- Note: `if` is not a statment, it's an **expression** 
- Optional `else` expression
- No paranthesis around the condition
  ```rust
  fn main() {
    let number = 3;

    if number < 5 {
      println!("condition was true");
    } else {
      println!("condition was false");
    }
  }
  ```
- Condition expression **MUST** be a bool, no "truthy value" business
  ```rust
  fn main () {
    let number = 5;

    // Gives error, number is not a boolean value 
    if number {
      println!("Some number");
    } 
  }
  ```
- `else if` can chain to make an `if..else` ladder
  ```rust
  if number < 5 {
    println!("Less than 5");
  } else if number == 5 {
    println!("Equal to 5");
  } else {
    println("Greater than 5");
  }
  ```
- Since `if` is an expression, you can use it with `let` statement to assign a value to a variable conditionally
  ```rust
  fn main() {
    let condition = true;
    let number1 = if condition { 5 } else { 6 };
    // Following gives error, because 
    // let number = if condition { 5 } else { "Not 5" };

    println!("The value of number is: {number}");

    // The following code is equivalent to the above statement 
    let number2;
    if condition {
      number2 = 1;
    } else {
      number2 = 2;
    }
  }
  ```

### Loops 

- Three types of looping with `loop`, `while` and `for`.
- `loop` executes a block infinitely untill interrupted (e.g. using `ctrl-c`)
- Can be broken out of using `break` 
- `continue` skips over any remaining code in the running iteration and goes to the next iteration
- `break` and `continue` always apply to the innermost loop 
- Returning values with loops with `break` 

  ```rust
  let mut counter = 0;

  let result = loop {
    counter += 1;
    if counter == 10 {
      break counter * 2;
    }
  };

  println!("The result is {result}");
  ```
  While `break` only exits the current loop, `return` always exits the current **function**.

##### Nested loops and breaks 
- Loop labels can be optionally used in case a nested loop wants to break its outer loop (the loop that wraps it)

  ```rust
  let mut count = 0;
  'counting_up: loop {
    println!("count = {count}");
    let mut remaining = 10;

    loop {
      println!("remaining = {remaining}");
      if remaining == 9 {
        break;
      }
      if count == 2 {
        break 'counting_up; // Specifically breaks the outer loop
      }
      remaining -= 1;
    }

    count += 1;
  }
  println!("End count = {count}");
  ```

  prints following 

  ```shell
  count = 0
  remaining = 10
  remaining = 9
  count = 1
  remaining = 10
  remaining = 9
  count = 2
  remaining = 10
  End count = 2
  ```

##### `while`
- `while` is a combination of `loop`, `if`, `else` and `break`
- "Loop on a condition and keep checking if it is true - if it is true run 'this', else run 'that' and break"

##### `for`
- Used to iterate over a collection
- Can be done with `loop` and `while`, but efficient with `for`
- Iterating on a collection with `for` is less error-prone
  ```rust
  let a = [10, 20, 30, 40, 50]

  // using while works, but potentially panics if array length changes  
  let mut i = 0;
  while i < 5 {
    println!("Value: {a[index]}")
  }

  // This is better  
  for element in a {
    println!("Value: {element} ")
  } 
  ```