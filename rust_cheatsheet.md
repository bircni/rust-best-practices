# 🦀 Rust & Cargo Cheatsheet - Tips & Tricks

A quick-reference guide for learning and building in Rust!

## 📦 Cargo Basics

### 🛠 Create a New Project

```bash
cargo new my_project       # Binary (with main.rs)
cargo new --lib my_lib     # Library (with lib.rs)
```

### ▶️ Build & Run

```bash
cargo build                # Compile the project
cargo run                  # Build & run
cargo check                # Check for errors only (faster)
```

### 📦 Manage Dependencies

Edit `Cargo.toml` to add:

```toml
serde = { version = "1", features = ["derive"] }
```

Then run:

```bash
cargo build
```

### 📄 Useful Commands

```bash
cargo update               # Update dependencies
cargo clean                # Remove target directory
cargo doc --open           # Generate and open docs
cargo test                 # Run tests
```

## 🦀 Rust Language Basics

### 📚 Variables & Types

```rust
let x: i32 = 5;            // Immutable
let mut y = 10;            // Mutable
```

### 🚧 Error Handling & Options

```rust
let result = might_fail();
match result {
    Ok(val) => println!("Success: {}", val),
    Err(e) => eprintln!("Error: {}", e),
}

// Shortcut with ?
fn foo() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("file.txt")?;
    Ok(())
}

// Or with anyhow for easier error handling
use anyhow::Result;
fn read_file() -> anyhow::Result<String> {
    let content = std::fs::read_to_string("file.txt")?;
    Ok(content)
}

// Optional values

let maybe_value: Option<i32> = Some(10);
if let Some(value) = maybe_value {
    println!("Value is: {}", value);
} else {
    println!("No value found");
}

// Or optional values
fn a_huge_function(a: Option<String>) -> anyhow::Result<i32> {
    let b = a.context("Expected a string but was none")?;
    // Do something with b
    //....
    // return the result
    Ok(42)
}
// Work with `Option` and `Result` types also works with if let Some/None or Ok/Err patterns
fn do_something() {
    if let Some(value) = get_value() {
        println!("Value: {}", value);
    } else {
        println!("No value found");
    }

    // Or using Result
    if let Ok(result) = get_result() {
        println!("Result: {}", result);
    } else {
        println!("Error occurred");
    }
}
```

### 🔃 Control Flow

```rust
if x > 0 {
    println!("Positive");
} else {
    println!("Non-positive");
}

for i in 0..5 {
    println!("{}", i);
}
```

## 🧰 Common Traits

Traits are like interfaces in Rust, allowing you to define shared behavior.
They can be derived automatically for structs and enums.

### `Debug`

```rust
#[derive(Debug)]
struct MyStruct { a: i32 }
println!("{:?}", MyStruct { a: 42 });
```

### `Clone`, `Copy`, `PartialEq`

```rust
#[derive(Clone, Copy, PartialEq)]
struct Point { x: i32, y: i32 }
```

## 🗃️ Vectors, Strings, HashMaps

### 📦 Vector

```rust
let mut v = vec![1, 2, 3];
v.push(4);
for val in &v {
    println!("{}", val);
}
```

### 🔡 String

```rust
let name = String::from("Rust");
let greeting = format!("Hello, {}!", name);
```

### 🔑 HashMap

```rust
use std::collections::HashMap;
let mut map = HashMap::new();
map.insert("rust", 2025);
if let Some(year) = map.get("rust") {
    println!("Launched in: {}", year);
}
```

## 🔄 Pattern Matching

```rust
match number {
    1 => println!("One"),
    2 | 3 => println!("Two or Three"),
    _ => println!("Other"),
}
```

## 🧪 Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

Run tests:

```bash
cargo test
```

## ⚙️ Useful Crates

| Crate     | Purpose                |
| --------- | ---------------------- |
| `serde`   | JSON (de)serialization |
| `clap`    | CLI argument parsing   |
| `ureq`    | HTTP requests (sync)   |
| `reqwest` | HTTP requests (async)  |
| `anyhow`  | Easy error handling    |

## 🚀 Productivity Tips

- Use `rust-analyzer` in VS Code for IntelliSense
- Use `dbg!(...)` for quick debug prints
- On an error or warning, press `CTRL` + `.` to see quick fixes from `rust-analyzer`

## 🔒 Lifetimes (Quick Hint)

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}
```

## 📎 Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Crates.io](https://crates.io/) — Explore libraries
- [Docs.rs](https://docs.rs/) — Library documentation

Happy hacking! 🦀✨
