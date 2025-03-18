# Ownership 

### Concept 

- A discipline that ensures **safety** of Rust programs 
- Rust’s goal is to compile programs into efficient binaries that require as few runtime checks as possible, hence the rules of ownership
- Ownership rules avoid scenarios like accessing a variable before it is defined (the scenarios where the resultant behavior of the program is unreliable)
- Ownership is a model of thinking about _memory_, which is not too high level (like "memory is a thing that holds my data") and not too low level (like "memory is a location in my physical RAM which I get after using `malloc`")
- Variables live in "stack" and Boxes live in "heap"
  ```rust
  // Variable a contains an array of 1 million values
  // and it is copied in the second line
  let a = [0; 1_000_000];
  let b = a;

  // Here Box is used to contain the same values
  // and now b = a does not copy the data since it lives in heap
  // a in this case is pointer
  let a = Box::new([0; 1_000_000]);
  let b = a;
  ```
- Rust **does not** have `free` to manually deallocate memory from heap 
- How Rust decides to deallocate a `Box`?
  - When a `Box` is allocated to a variable, that variable (pointer) is said to **own** the box
  - e.g.
    ```rust
    let a = Box::new(5);
    let b = a 
    ```
  - It is incorrect to free the `Box` referenced by `a` and `b` both when the go out of scope, because `free` should be happening only once (since there's only 1 box)
  - Hence it is said that "Box's ownership is moved from a to b" in above snippet 
  - If a variable owns a box, when Rust deallocates the variable’s frame, then Rust deallocates the box’s heap memory
  - In above snippet, if the programmer tries to access the box through `a`, Rust borrow checker gives error saying the ownership has moved
  - At any given point of time, there's only one variable that "owns" the data 
- Rust data structures like `Vec`, `String` and `HashMap` use `Box`s underneath

##### A little more involving example 

```rust
fn main() {
  let first = String::from("Ferris"); // L1
  let full = add_suffix(first); // L4 
  println!("{full}");
}

fn add_suffix(mut name: String) -> String {
  /* L2 */ name.push_str(" Jr."); // L3
  name
}
```

- At L1, the string “Ferris” has been allocated on the heap. It is owned by `first`.
- At L2, the function `add_suffix(first)` has been called. This moves ownership of the string from `first` to `name`. The string data is not copied, but the pointer to the data _is_ copied.
- At L3, the function `name.push_str(" Jr.")` resizes the string’s heap allocation. This does three things. 
  - First, it creates a new larger allocation. 
  - Second, it writes “Ferris Jr.” into the new allocation. 
  - Third, it frees the original heap memory. `first` now points to deallocated memory.
- At L4, the frame for `add_suffix` is gone. This function returned `name`, transferring ownership of the string to `full`.

Rust borrow checker (BC) will throw error if we try to access `first` after calling `add_suffix`. This happens because `first` now points to deallocated i.e. invalid memory. It is ok for `first` to exist, it's just not ok to _use_ `first` in the program thereafter. 

This is called **"Moved heap data principle"** - if a variable `x` moves ownership of heap data to another variable `y`, then `x` cannot be used after the move.

Cloning avoid moves.
- Following doesn't produce an error from BC
  ```rust
  fn main() {
    let first = String::from("Ferris");
    let first_clone = first.clone(); 
    let full = add_suffix(first_clone);
    println!("{full}, originally {first}"); // first still exists at this level 
  }

  fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
  }
  ```

