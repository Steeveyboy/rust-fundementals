// 03 - Data Types
//
// Topics covered:
//   - Integer types: i8/u8 … i128/u128, isize/usize
//   - Floating-point types: f32, f64
//   - Boolean type: bool
//   - Character type: char (Unicode scalar value)
//   - Tuple type
//   - Array type (fixed-length, stack-allocated)
//   - String slice (&str) vs owned String

fn main() {
    // --- Integer types ---
    let a: i32 = -100;
    let b: u32 = 200;
    let c: i64 = 9_000_000_000; // underscores improve readability
    let hex: u8 = 0xFF;
    let binary: u8 = 0b1010_0101;
    let octal: u8 = 0o77;
    println!("Integers: a={a}, b={b}, c={c}");
    println!("Literals: hex={hex}, binary={binary}, octal={octal}");

    // Integer overflow (checked at runtime in debug builds, wraps in release).
    let max_u8: u8 = u8::MAX;
    println!("u8::MAX = {max_u8}");

    // --- Floating-point ---
    let f1: f32 = 3.14;
    let f2: f64 = 2.718_281_828; // f64 is the default
    println!("f32={f1:.4}, f64={f2:.9}");

    // --- Boolean ---
    let t: bool = true;
    let f: bool = false;
    println!("bool: t={t}, f={f}, and={}", t && f);

    // --- Char (Unicode scalar value, 4 bytes) ---
    let letter: char = 'R';
    let emoji: char = '🦀';
    println!("char: letter={letter}, emoji={emoji}");

    // --- Tuples (heterogeneous, fixed-length) ---
    let tup: (i32, f64, bool) = (500, 6.4, true);
    let (x, y, z) = tup; // destructuring
    println!("Tuple destructure: x={x}, y={y}, z={z}");
    println!("Tuple index access: tup.0={}", tup.0);

    // --- Arrays (homogeneous, fixed-length, stack-allocated) ---
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let zeros = [0; 3]; // [0, 0, 0]
    println!("Array: {:?}", arr);
    println!("Zeros: {:?}", zeros);
    println!("arr[2] = {}", arr[2]);
    println!("Array length = {}", arr.len());

    // --- &str vs String ---
    let s_literal: &str = "I am a string slice"; // baked into binary
    let s_owned: String = String::from("I am an owned String");
    let s_concat = format!("{} | {}", s_literal, s_owned);
    println!("{s_concat}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn integer_arithmetic() {
        let a: i32 = 10;
        let b: i32 = 3;
        assert_eq!(a + b, 13);
        assert_eq!(a - b, 7);
        assert_eq!(a * b, 30);
        assert_eq!(a / b, 3); // integer division truncates
        assert_eq!(a % b, 1);
    }

    #[test]
    fn float_precision() {
        let f: f64 = 0.1 + 0.2;
        assert!((f - 0.3).abs() < 1e-10);
    }

    #[test]
    fn tuple_destructure() {
        let tup = (1, 2.0_f64, true);
        let (a, b, c) = tup;
        assert_eq!(a, 1);
        assert!((b - 2.0).abs() < f64::EPSILON);
        assert!(c);
    }

    #[test]
    fn array_indexing() {
        let arr = [10, 20, 30, 40, 50];
        assert_eq!(arr[0], 10);
        assert_eq!(arr[4], 50);
        assert_eq!(arr.len(), 5);
    }

    #[test]
    fn string_types() {
        let slice: &str = "hello";
        let owned: String = String::from("hello");
        assert_eq!(slice, owned.as_str());
    }
}
