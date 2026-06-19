// 02 - Variables, Mutability, Constants & Shadowing
//
// Topics covered:
//   - `let` bindings (immutable by default)
//   - `let mut` for mutable variables
//   - `const` for compile-time constants
//   - Shadowing — rebinding a name in the same scope
//   - Type annotations

// Constants are evaluated at compile time, must have a type annotation,
// and can be declared in any scope (including global scope).
const MAX_SCORE: u32 = 100;
const PI_APPROX: f64 = 3.14159;

fn main() {
    // --- Immutable binding ---
    let x = 5;
    println!("x = {x}");
    // x = 6; // compile error: cannot assign twice to immutable variable

    // --- Mutable binding ---
    let mut counter = 0;
    counter += 1;
    counter += 1;
    println!("counter = {counter}");

    // --- Constants ---
    println!("MAX_SCORE = {MAX_SCORE}");
    println!("PI_APPROX = {PI_APPROX}");

    // --- Shadowing ---
    // You can re-declare a variable with the same name; the new binding
    // *shadows* the old one.  Unlike `mut`, shadowing can also change the type.
    let value = 10;
    println!("value (before shadow) = {value}");
    let value = value * 2; // shadow: new immutable binding
    println!("value (after shadow)  = {value}");

    // Shadowing with a type change.
    let spaces = "   "; // &str
    let spaces = spaces.len(); // usize — different type, same name
    println!("spaces = {spaces}");

    // --- Explicit type annotations ---
    let typed: i32 = -42;
    let float: f64 = 2.718;
    println!("typed = {typed}, float = {float}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constants_have_correct_values() {
        assert_eq!(MAX_SCORE, 100);
        assert!((PI_APPROX - 3.14159).abs() < f64::EPSILON);
    }

    #[test]
    fn mutable_counter() {
        let mut counter = 0u32;
        counter += 1;
        counter += 1;
        assert_eq!(counter, 2);
    }

    #[test]
    fn shadowing_changes_value() {
        let value = 10;
        let value = value * 2;
        assert_eq!(value, 20);
    }

    #[test]
    fn shadowing_changes_type() {
        let spaces = "   ";
        let spaces = spaces.len();
        assert_eq!(spaces, 3);
    }
}
