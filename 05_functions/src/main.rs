// 05 - Functions
//
// Topics covered:
//   - Function definitions and calling conventions
//   - Parameters and return values
//   - Expressions vs statements (the last expression is the return value)
//   - Multiple return values via tuples
//   - Closures (anonymous functions that capture their environment)
//   - Higher-order functions: passing and returning functions
//   - `fn` pointers

// Simple function: no parameters, no return value.
fn greet() {
    println!("Hello from a function!");
}

// Function with parameters and an explicit return type.
fn add(a: i32, b: i32) -> i32 {
    a + b // expression (no semicolon) is the implicit return value
}

// Early return with `return`.
fn absolute_value(n: i32) -> i32 {
    if n < 0 {
        return -n;
    }
    n
}

// Returning multiple values via a tuple.
fn min_max(values: &[i32]) -> (i32, i32) {
    let mut min = values[0];
    let mut max = values[0];
    for &v in values.iter() {
        if v < min { min = v; }
        if v > max { max = v; }
    }
    (min, max)
}

// A function that accepts another function as a parameter (function pointer).
fn apply(f: fn(i32) -> i32, value: i32) -> i32 {
    f(value)
}

fn double(x: i32) -> i32 { x * 2 }
fn square(x: i32) -> i32 { x * x }

fn main() {
    greet();

    let sum = add(3, 4);
    println!("3 + 4 = {sum}");

    println!("abs(-7) = {}", absolute_value(-7));
    println!("abs(7)  = {}", absolute_value(7));

    let nums = [3, 1, 4, 1, 5, 9, 2, 6];
    let (min, max) = min_max(&nums);
    println!("min={min}, max={max}");

    // --- Closures ---
    // Closures are anonymous functions that can capture variables from their scope.
    let multiplier = 3;
    let multiply = |x| x * multiplier; // captures `multiplier` by reference
    println!("5 × {multiplier} = {}", multiply(5));

    // Closures with explicit type annotations.
    let add_ten = |x: i32| -> i32 { x + 10 };
    println!("20 + 10 = {}", add_ten(20));

    // Moving ownership into a closure with `move`.
    let greeting = String::from("Hi");
    let say_hi = move || println!("{greeting}, Rustacean!");
    say_hi();

    // --- Higher-order functions ---
    println!("double(5) = {}", apply(double, 5));
    println!("square(5) = {}", apply(square, 5));

    // Using closures with iterator adapters.
    let numbers: Vec<i32> = (1..=5).collect();
    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    let evens: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).cloned().collect();
    println!("Original:  {:?}", numbers);
    println!("Doubled:   {:?}", doubled);
    println!("Evens:     {:?}", evens);

    let total: i32 = numbers.iter().sum();
    println!("Sum 1..=5: {total}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn test_absolute_value() {
        assert_eq!(absolute_value(-5), 5);
        assert_eq!(absolute_value(5), 5);
        assert_eq!(absolute_value(0), 0);
    }

    #[test]
    fn test_min_max() {
        let (min, max) = min_max(&[3, 1, 4, 1, 5, 9]);
        assert_eq!(min, 1);
        assert_eq!(max, 9);
    }

    #[test]
    fn closure_captures_environment() {
        let factor = 4;
        let multiply = |x| x * factor;
        assert_eq!(multiply(5), 20);
    }

    #[test]
    fn higher_order_apply() {
        assert_eq!(apply(double, 6), 12);
        assert_eq!(apply(square, 4), 16);
    }

    #[test]
    fn iterator_map_and_filter() {
        let numbers: Vec<i32> = (1..=5).collect();
        let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6, 8, 10]);

        let evens: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).cloned().collect();
        assert_eq!(evens, vec![2, 4]);
    }
}
