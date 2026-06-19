// 01 - Hello World
//
// Topics covered:
//   - The `main` function as the program entry point
//   - `println!` macro for printing to stdout
//   - `eprintln!` macro for printing to stderr
//   - `format!` macro for building strings
//   - Basic string formatting with `{}`

fn main() {
    // The simplest Rust program: print a line to stdout.
    println!("Hello, world!");

    // `println!` accepts format strings just like Rust's `format!`.
    let name = "Rustacean";
    println!("Hello, {}!", name);

    // Named arguments in format strings.
    println!("Hello, {name}!"); // Rust 1.58+ captured identifier syntax

    // Formatting numbers.
    let answer = 42;
    println!("The answer is {answer}.");

    // Debug formatting with `{:?}` — useful for complex types.
    let numbers = [1, 2, 3];
    println!("Numbers: {:?}", numbers);
    println!("Numbers (pretty): {:#?}", numbers);

    // Print to stderr with `eprintln!`.
    eprintln!("This goes to standard error.");

    // Build a string without printing using `format!`.
    let greeting = format!("Hello from {name}!");
    println!("{greeting}");
}

#[cfg(test)]
mod tests {
    // `format!` is the easiest way to test formatted output without capturing stdout.
    #[test]
    fn format_greeting() {
        let name = "world";
        let result = format!("Hello, {}!", name);
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn format_debug() {
        let nums = [1, 2, 3];
        let result = format!("{:?}", nums);
        assert_eq!(result, "[1, 2, 3]");
    }
}
