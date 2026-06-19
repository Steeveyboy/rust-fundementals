// 08 - Enums, Option & Result
//
// Topics covered:
//   - Defining enums with and without data
//   - Pattern matching on enums with `match`
//   - `if let` and `while let` for concise matching
//   - The `Option<T>` enum (`Some` / `None`)
//   - The `Result<T, E>` enum (`Ok` / `Err`)
//   - Common combinators: `unwrap`, `expect`, `map`, `unwrap_or`, `?` operator
//   - Recursive data structure using `Box<T>`

// A simple enum — variants can be unit, tuple, or struct-like.
#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East  => Direction::West,
            Direction::West  => Direction::East,
        }
    }
}

// An enum whose variants carry data.
#[derive(Debug)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius }           => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height }   => 0.5 * base * height,
        }
    }

    fn name(&self) -> &str {
        match self {
            Shape::Circle { .. }    => "Circle",
            Shape::Rectangle { .. } => "Rectangle",
            Shape::Triangle { .. }  => "Triangle",
        }
    }
}

// --- Option<T> helpers ---

fn find_first_even(numbers: &[i32]) -> Option<i32> {
    numbers.iter().find(|&&x| x % 2 == 0).copied()
}

fn double_if_some(opt: Option<i32>) -> Option<i32> {
    opt.map(|x| x * 2)
}

// --- Result<T, E> helpers ---

#[derive(Debug, PartialEq)]
enum ParseError {
    Empty,
    InvalidNumber(String),
}

fn parse_positive(s: &str) -> Result<u32, ParseError> {
    if s.is_empty() {
        return Err(ParseError::Empty);
    }
    s.trim().parse::<u32>().map_err(|_| ParseError::InvalidNumber(s.to_string()))
}

// Using the `?` operator to propagate errors.
fn parse_and_double(s: &str) -> Result<u32, ParseError> {
    let n = parse_positive(s)?; // returns early with Err if parse fails
    Ok(n * 2)
}

// A simple recursive linked list using `Box<T>` to avoid infinite size.
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn new() -> Self { List::Nil }

    fn prepend(self, value: i32) -> Self {
        List::Cons(value, Box::new(self))
    }

    fn sum(&self) -> i32 {
        match self {
            List::Nil          => 0,
            List::Cons(v, rest) => v + rest.sum(),
        }
    }
}

fn main() {
    // --- Basic enum usage ---
    let dir = Direction::North;
    println!("Direction: {:?}, Opposite: {:?}", dir, dir.opposite());

    // --- Enum with data ---
    let shapes: Vec<Shape> = vec![
        Shape::Circle { radius: 3.0 },
        Shape::Rectangle { width: 4.0, height: 5.0 },
        Shape::Triangle { base: 6.0, height: 8.0 },
    ];
    for s in &shapes {
        println!("{}: area = {:.4}", s.name(), s.area());
    }

    // --- Option ---
    let nums = [1, 3, 5, 4, 7];
    match find_first_even(&nums) {
        Some(n) => println!("First even: {n}"),
        None    => println!("No even number found"),
    }

    // `if let` is syntactic sugar for matching a single pattern.
    if let Some(n) = find_first_even(&[1, 3, 5]) {
        println!("Found even: {n}");
    } else {
        println!("No even found in [1, 3, 5]");
    }

    // Option combinators.
    println!("double_if_some(Some(5))  = {:?}", double_if_some(Some(5)));
    println!("double_if_some(None)     = {:?}", double_if_some(None));
    println!("unwrap_or: {}", find_first_even(&[1, 3]).unwrap_or(-1));

    // --- Result ---
    match parse_positive("42") {
        Ok(n)  => println!("Parsed: {n}"),
        Err(e) => println!("Error: {:?}", e),
    }
    match parse_positive("abc") {
        Ok(n)  => println!("Parsed: {n}"),
        Err(e) => println!("Error: {:?}", e),
    }
    println!("parse_and_double(\"7\") = {:?}", parse_and_double("7"));
    println!("parse_and_double(\"\")  = {:?}", parse_and_double(""));

    // --- Recursive list ---
    let list = List::new().prepend(3).prepend(2).prepend(1);
    println!("List: {:?}", list);
    println!("Sum:  {}", list.sum());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_opposite() {
        assert_eq!(Direction::North.opposite(), Direction::South);
        assert_eq!(Direction::East.opposite(),  Direction::West);
    }

    #[test]
    fn shape_areas() {
        let c = Shape::Circle { radius: 1.0 };
        assert!((c.area() - std::f64::consts::PI).abs() < 1e-10);

        let r = Shape::Rectangle { width: 3.0, height: 4.0 };
        assert!((r.area() - 12.0).abs() < f64::EPSILON);

        let t = Shape::Triangle { base: 6.0, height: 4.0 };
        assert!((t.area() - 12.0).abs() < f64::EPSILON);
    }

    #[test]
    fn find_even_some() {
        assert_eq!(find_first_even(&[1, 3, 4, 5]), Some(4));
    }

    #[test]
    fn find_even_none() {
        assert_eq!(find_first_even(&[1, 3, 5]), None);
    }

    #[test]
    fn double_if_some_variants() {
        assert_eq!(double_if_some(Some(6)), Some(12));
        assert_eq!(double_if_some(None), None);
    }

    #[test]
    fn parse_positive_ok() {
        assert_eq!(parse_positive("10"), Ok(10));
    }

    #[test]
    fn parse_positive_empty() {
        assert_eq!(parse_positive(""), Err(ParseError::Empty));
    }

    #[test]
    fn parse_positive_invalid() {
        assert!(matches!(parse_positive("xyz"), Err(ParseError::InvalidNumber(_))));
    }

    #[test]
    fn parse_and_double_propagates() {
        assert_eq!(parse_and_double("5"), Ok(10));
        assert_eq!(parse_and_double(""), Err(ParseError::Empty));
    }

    #[test]
    fn list_sum() {
        let list = List::new().prepend(3).prepend(2).prepend(1);
        assert_eq!(list.sum(), 6);
    }
}
