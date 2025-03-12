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

# Constants

- Constants can be declared using the word `const` and they are ALWAYS read-only
- e.g. `const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;`
- The type of their value MUST be annotated, since it is always known at the time of declaring them
- Can be declared in any scope, even global
- Can only be set to a constant expression at the compile=time, not a runtime computation
- Rust’s naming convention for constants is to use all uppercase with underscores between words.

# Shadowing

- A variable if declared with tbe same name, using `let` keyword
- Different than making `mut`
  - `mut` mutates the same variable, this creates a new variable which "shadows" the older one
  - If `mut` variable is declared, it has to be given the value of the same datatype.
