// 06 - Ownership, Borrowing & Slices
//
// Topics covered:
//   - Ownership rules (each value has exactly one owner)
//   - Move semantics
//   - Clone (deep copy)
//   - Copy types (stack-only)
//   - References and borrowing (&T)
//   - Mutable references (&mut T)
//   - The borrow checker rules
//   - String slices (&str)
//   - Array/slice types (&[T])

fn takes_ownership(s: String) {
    println!("Got ownership of: {s}");
} // `s` is dropped here

fn makes_copy(n: i32) {
    println!("Copied i32: {n}");
} // `n` (a Copy type) is just dropped; original is unaffected

// Borrow a String — caller retains ownership.
fn calculate_length(s: &String) -> usize {
    s.len()
} // `s` goes out of scope, but doesn't drop the data (it's just a reference)

// Mutable borrow — modify without taking ownership.
fn append_exclamation(s: &mut String) {
    s.push_str("!");
}

// Return a slice of the first word in a string.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s // entire string is one word
}

fn sum_slice(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

fn main() {
    // --- Ownership & Move ---
    let s1 = String::from("hello");
    takes_ownership(s1);
    // println!("{s1}"); // compile error: s1 was moved

    let n1 = 42; // i32 implements Copy
    makes_copy(n1);
    println!("n1 still valid: {n1}"); // Copy types are not moved

    // --- Clone (explicit deep copy) ---
    let s2 = String::from("world");
    let s3 = s2.clone();
    println!("s2={s2}, s3={s3}"); // both valid

    // --- Borrowing (immutable reference) ---
    let s4 = String::from("borrowing");
    let len = calculate_length(&s4); // pass a reference
    println!("Length of '{s4}' is {len}"); // s4 still valid

    // Multiple immutable borrows are fine simultaneously.
    let r1 = &s4;
    let r2 = &s4;
    println!("r1={r1}, r2={r2}");

    // --- Mutable reference ---
    let mut s5 = String::from("hello");
    append_exclamation(&mut s5);
    println!("After mutation: {s5}");

    // Only one mutable reference at a time is allowed.
    {
        let r_mut = &mut s5;
        r_mut.push_str("!"); // only r_mut can access s5 here
    } // r_mut goes out of scope here
    println!("After second mutation: {s5}");

    // --- String slices ---
    let sentence = String::from("hello world");
    let word = first_word(&sentence);
    println!("First word: {word}");

    // &str literals are already slices.
    let literal: &str = "hello";
    println!("Literal first word: {}", first_word(literal));

    // --- Array slices ---
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4]; // [2, 3, 4]
    println!("Slice: {:?}", slice);
    println!("Sum of whole array: {}", sum_slice(&arr));
    println!("Sum of slice [1..4]:  {}", sum_slice(slice));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrow_does_not_move() {
        let s = String::from("rust");
        let len = calculate_length(&s);
        assert_eq!(len, 4);
        assert_eq!(s, "rust"); // s is still valid
    }

    #[test]
    fn mutable_borrow_mutates() {
        let mut s = String::from("hello");
        append_exclamation(&mut s);
        assert_eq!(s, "hello!");
    }

    #[test]
    fn clone_is_independent() {
        let s1 = String::from("original");
        let mut s2 = s1.clone();
        s2.push_str(" copy");
        assert_eq!(s1, "original");
        assert_eq!(s2, "original copy");
    }

    #[test]
    fn first_word_single() {
        assert_eq!(first_word("hello"), "hello");
    }

    #[test]
    fn first_word_multiple() {
        assert_eq!(first_word("hello world"), "hello");
    }

    #[test]
    fn sum_of_slice() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(sum_slice(&arr), 15);
        assert_eq!(sum_slice(&arr[1..3]), 5); // 2 + 3
    }
}
