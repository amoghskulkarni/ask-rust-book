# Guessing game (Basic app)

### Learnings

- `cargo new guessing_game` creates the crate for the project (notice the `snake_case`, and not `kebab-case`. `snake_case` is the Rust naming convention)
- `std::io` module has to be explicitly imported, because it is not in the _prelude_. A programs _prelude_ is a set of items defined in the standard library that are brought into the scope of every program.
- `let` _defines_ variables, `mut` keyword makes a variable _mutable_. Variables are defined immutable by default.
- `::new()` syntax
  - The `::` syntax indicates that `new` is an _associated function_ of the `String` type.
  - An associated function is a function that’s implemented on a type, in this case _String_.
- Could have written `std::io::stdin` instead of `use std::io` + `io::stdin`, both valid
  - The `stdin` function returns an instance of `std::io::Stdin`
  - It is a type that represents a handle to the standard input to the terminal
- `&` for passing the reference of variable to functions
  - References are by default immutable, have to use `&mut` for making them mutable
- Basic error handling
  - `.expect()` here prints an error statement if the operation fails
  - `Ok` and `Err` are the two possible values of the _enum_ `Result`, and they're called its _variants_
  - If `Ok` is returned, message is not displayed. If `Err` is the `Result`'s variant, error message is displayed
- String formatting
  - `println!("You guessed: {}", guess);`
  - Another way - `println!("x = {x} and y + 2 = {}", y + 2);`
- Dependencies of a crate
  - Defined in `[dependencies]` section in `Cargo.toml`
  - Cargo understands [SemVer](https://semver.org/).
    - The specifier `0.9.0` is actually shorthand for `^0.9.0`, i.e. get me the highest patch of the same version (note that the meaning of `^` is different than the one in `package.json` in Node)
    - Project will remain at `0.9.0` until you explicitly upgrade, thanks to the [Cargo.lock](./guessing_game/Cargo.lock) file. Improves reproducibility of the code.
  - `cargo build` downloads deps from [Crates.io](https://crates.io/) registry and installs
  - `cargo update` updates lockfile and `Cargo.toml` file to include the latest patch of the given major version and minor subversion
- Generating random number
  - `rand::thread_rng().gen_range(1..=100);`
  - What is `::Rng` in `use rand::Rng`??
    - The `Rng` _trait_ defines methods that random number generators implement, and this _trait_ must be in scope to use those methods
    - Now we call the `gen_range` method on the random number generator, since this method is defined by the `Rng` trait
    - How to know which traits to use?
      - Each crate has documentation with instructions for using it
      - `cargo doc --open` opens docs locally in the browser. This creates a `/target/doc` directory that contains all documentation in html format.
      - Click `rand` in the left sidebar to know more about traits of `rand` module
  - `start..=end` is for inclusive range (i.e. `[start, end]`) and `start..end` is for exclusive range (i.e. `(start, end)`)
- `match` expressions and _arms_
  - `match` are switch-like statements, which accept a literal value and multiple _arms_ (options to be matched against)
  - This is done by something called "pattern matching"
  - `match guess.cmp(&secret_number) {...}` block in the code
- `expect(...)` method
  - Allows us to give a error message if something fails
  - e.g. `stdin().read_line(&mut input_var).expect("failed to read input")` will print that error log if something fails
- Shadowing
  - A variable of the same name as that of an already existing one can be declared, and the newest value will take precedence
  - Even if `let mut guess = String::new();` this is written before, we can write `let guess = guess.trim().parse::<u32>()`, last immutable declaration will take precedence
- `loop` keyword
  - creates infinite loop by default
  - wrap the main logic in it to keep asking for a number
- Error handling
  - `match` expr can be used to catch Error states like so
    ```rust
    let guess: u32 = match guess.trim().parse() {
              Ok(num) => num,
              Err(_) => continue,
          };
    ```
  - another option is to use `expect()` method to print, but still default error message of the library will be shown
  - `match` with `continue` for `Err(_)` variant for guess variable will ensure nothing is executed in error case (i.e. our empty error handler gets executed, and program crashes gracefully)
