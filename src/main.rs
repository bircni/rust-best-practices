use anyhow::Context;

fn main() {
    println!("Hello, world!");
    println!("My version is {}", env!("CARGO_PKG_VERSION"));
    // if you use a function or macro that is not allowed by clippy,
    // please think about if you can use a different function or macro
    // or if you can use a different approach.
    // if you can't, please add an exception to the clippy lint
    // and add a comment why you need it.
    // for example:

    #[expect(clippy::expect_used, reason = "This is an example")]
    let _ = std::fs::read_to_string("Cargo.toml").expect("Failed to read Cargo.toml");

    let anyhow_result = example_anyhow();
    // Handle the result of the function you could also make the main function return a Result
    // and use the ? operator to propagate the error
    // but in this example we want to handle the error and do not want to hard exit
    // the program if the file does not exist
    match anyhow_result {
        // we do not use the result here so we use _ to ignore the result
        Ok(_) => println!("Success with result"),
        Err(e) => eprintln!("Error: {e}"),
    }
}

/// This is an example of using anyhow
/// It will return an error if the file does not exist
fn example_anyhow() -> anyhow::Result<String> {
    std::fs::read_to_string("Cargo.toml")
        // .context casts the error to anyhow::Error
        // and adds a context message to the error
        // this is useful if you want to add more information to the error
        .context("Failed to read Cargo.toml")
}
