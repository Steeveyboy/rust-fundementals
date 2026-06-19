// 04 - Control Flow
//
// Topics covered:
//   - if / else if / else
//   - if as an expression (returns a value)
//   - loop (infinite loop with `break` returning a value)
//   - while
//   - for with ranges and iterators
//   - match (exhaustive pattern matching)
//   - Nested patterns and guards in match

fn main() {
    // --- if / else if / else ---
    let number = 7;
    if number < 5 {
        println!("{number} is less than 5");
    } else if number == 5 {
        println!("{number} is exactly 5");
    } else {
        println!("{number} is greater than 5");
    }

    // if is an expression — it can return a value.
    let description = if number % 2 == 0 { "even" } else { "odd" };
    println!("{number} is {description}");

    // --- loop ---
    let mut count = 0;
    // `loop` returns the value after `break`.
    let result = loop {
        count += 1;
        if count == 5 {
            break count * 2; // break with a value
        }
    };
    println!("Loop result: {result}"); // 10

    // Loop labels let you break out of nested loops.
    'outer: for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 {
                println!("Breaking outer loop at i={i}, j={j}");
                break 'outer;
            }
        }
    }

    // --- while ---
    let mut n = 3;
    while n > 0 {
        println!("while: n={n}");
        n -= 1;
    }

    // --- for with a range ---
    for i in 0..5 {
        print!("{i} ");
    }
    println!(); // newline

    // Inclusive range with `..=`.
    for i in 1..=5 {
        print!("{i} ");
    }
    println!();

    // --- for over a collection ---
    let fruits = ["apple", "banana", "cherry"];
    for fruit in &fruits {
        println!("Fruit: {fruit}");
    }

    // With index using `.iter().enumerate()`.
    for (idx, fruit) in fruits.iter().enumerate() {
        println!("  [{idx}] {fruit}");
    }

    // --- match ---
    let coin_value = 25u32;
    let name = match coin_value {
        1  => "penny",
        5  => "nickel",
        10 => "dime",
        25 => "quarter",
        _  => "unknown",
    };
    println!("Coin: {name}");

    // Match with a guard.
    let num = 13;
    let category = match num {
        n if n < 0  => "negative",
        0           => "zero",
        n if n < 10 => "small positive",
        _           => "large positive",
    };
    println!("{num} is {category}");

    // Match with tuple.
    let pair = (true, 42);
    match pair {
        (true, n) if n > 0 => println!("true and positive: {n}"),
        (true, _)          => println!("true but not positive"),
        (false, _)         => println!("false"),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn if_expression() {
        let x = 4;
        let label = if x % 2 == 0 { "even" } else { "odd" };
        assert_eq!(label, "even");
    }

    #[test]
    fn loop_returns_value() {
        let mut i = 0;
        let v = loop {
            i += 1;
            if i == 3 {
                break i * 10;
            }
        };
        assert_eq!(v, 30);
    }

    #[test]
    fn for_range_sum() {
        let sum: i32 = (1..=10).sum();
        assert_eq!(sum, 55);
    }

    #[test]
    fn match_basic() {
        let x = 2u32;
        let label = match x {
            1 => "one",
            2 => "two",
            _ => "other",
        };
        assert_eq!(label, "two");
    }

    #[test]
    fn match_guard() {
        let x = 15i32;
        let category = match x {
            n if n < 0  => "negative",
            0           => "zero",
            n if n < 10 => "small",
            _           => "large",
        };
        assert_eq!(category, "large");
    }
}
