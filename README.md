# Rust Dependency Demo

This project demonstrates how to create a Rust library and use it as a dependency in another Rust project.

## Project Structure

```
demo_deps/
├── my_lib/          # Library crate
│   ├── src/
│   │   └── lib.rs   # Library implementation
│   └── Cargo.toml   # Library configuration
├── demo_app/        # Application crate
│   ├── src/
│   │   └── main.rs  # Application using the library
│   └── Cargo.toml   # Application configuration (depends on my_lib)
└── README.md        # This file
```

## Library Features (`my_lib`)

The library provides three main modules:

### Calculator Module
- `add(left, right)` - Addition
- `multiply(left, right)` - Multiplication  
- `power(base, exponent)` - Exponentiation

### String Utilities Module
- `reverse_string(input)` - Reverse a string
- `count_words(input)` - Count words in a string
- `to_title_case(input)` - Convert to title case

### Greeting Function
- `greet(name)` - Generate a personalized greeting

## Running the Demo

1. **Build the library:**
   ```bash
   cd my_lib
   cargo build
   ```

2. **Run library tests:**
   ```bash
   cd my_lib
   cargo test
   ```

3. **Build and run the demo application:**
   ```bash
   cd demo_app
   cargo run
   ```

## How the Dependency Works

The `demo_app` depends on `my_lib` using a path dependency defined in `demo_app/Cargo.toml`:

```toml
[dependencies]
my_lib = { path = "../my_lib" }
```

This tells Cargo to look for the `my_lib` crate in the relative path `../my_lib`.

## Alternative Dependency Methods

Instead of path dependencies, you could also use:

1. **Git dependency:**
   ```toml
   my_lib = { git = "https://github.com/username/my_lib.git" }
   ```

2. **Published crate (from crates.io):**
   ```toml
   my_lib = "0.1.0"
   ```

3. **Local registry or private registry**

## Development Notes

- The library uses Rust 2024 edition
- All functions are properly tested
- The code follows Rust best practices
- Both projects can be built and run independently
