# Your first Rust Project: `rustyweather` 🌦️

This project is a beginner-friendly Rust app that uses public weather APIs to display current weather information for a given city.
Your task will be to extend and enhance the app while learning core Rust concepts.

## ✅ Goals

By completing this project, you'll learn how to:

- Parse command-line arguments with `clap`
- Make HTTP requests using `ureq`
- Deserialize JSON using `serde`
- Work with `Result` and `Option` types
- Handle errors idiomatically in Rust
- Fix the lint issues which are already present in the codebase

## 🚀 Getting Started

1. Install Rust if you haven't yet:  
   <https://www.rust-lang.org/tools/install>

2. Clone this repo:

   ```bash
   git clone https://github.com/yourname/rustyweather
   cd rustyweather
   cargo run -- "London"
   ```

## 🛠️ Tasks

### ✅ Basic Functionality (Already Implemented)

- [x] Parse a city name from the command line
- [x] Query `wttr.in` for weather data
- [x] Print a basic weather report

### 🧠 Your Tasks

Commit each task with a clear message in the conventional commits format.

#### 1. **Add support for optional units (metric/imperial)**

```bash
cargo run -- "New York" --units
```

Use `clap` to support `--units` flag.

#### 2. **Format and pretty-print the weather report**

Instead of raw JSON, parse the output and format it nicely using `serde`.

#### 3. **Add error handling**

- Handle network errors (no internet)
- Handle invalid city names or empty responses

Tip: Use `Result` and `Option` types effectively by using `?` operator for error propagation and `anyhow` for optional values.

#### 4. **Add support for multiple cities**

```bash
cargo run -- "Paris" "Berlin" "Rome"
```

Loop through the list of cities and print each report.

#### 5. **Optional: Add offline caching**

Save the last query to a local file and reuse it if the API fails.

## 🧪 Bonus Ideas

- Add unit tests for your parsing logic
- Add colored output using `colored` crate
- Add emojis based on weather conditions ☀️ 🌧️ ❄️

## 🌐 About `wttr.in`

We use `https://wttr.in/<city>?format=j1`, a free, no-API-key-needed weather endpoint.

Example:

```bash
curl https://wttr.in/London?format=j1
```

This returns structured JSON data with current conditions, forecasts, etc.

## 📦 Crates Used

- [`clap`](https://docs.rs/clap/latest/clap/) – CLI parsing
- [`ureq`](https://docs.rs/ureq/) – HTTP requests
- [`serde`](https://serde.rs/) – JSON parsing
- [`serde_json`](https://docs.rs/serde_json/) – Deserialize JSON

## 🏁 To Run

```bash
cargo run -- "London"
```

Happy Hacking! 🦀
