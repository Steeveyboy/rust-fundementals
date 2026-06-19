// 07 - Structs
//
// Topics covered:
//   - Defining and instantiating structs
//   - Field init shorthand
//   - Struct update syntax
//   - Tuple structs
//   - Unit-like structs
//   - Methods and associated functions with `impl`
//   - Derived traits: Debug, Clone, PartialEq

// A standard struct with named fields.
#[derive(Debug, Clone, PartialEq)]
struct Rectangle {
    width: f64,
    height: f64,
}

// `impl` block: methods operate on instances of the struct.
impl Rectangle {
    // Associated function (no `self`) — acts like a constructor / static method.
    fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height } // field init shorthand (same name)
    }

    fn square(size: f64) -> Self {
        Rectangle { width: size, height: size }
    }

    // Method — takes `&self` (immutable borrow of the instance).
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn is_square(&self) -> bool {
        (self.width - self.height).abs() < f64::EPSILON
    }

    // Takes `&mut self` to mutate the instance in place.
    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }
}

// A tuple struct — useful for newtype wrappers.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Meters(f64);

#[derive(Debug, Clone, Copy, PartialEq)]
struct Kilograms(f64);

impl Meters {
    fn value(self) -> f64 { self.0 }
}

// A unit-like struct (no fields) — often used as a marker type.
#[derive(Debug)]
struct Marker;

// A more complex struct example.
#[derive(Debug, Clone, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Self {
        Person { name: name.to_string(), age }
    }

    fn greeting(&self) -> String {
        format!("Hi, I'm {} and I'm {} years old.", self.name, self.age)
    }
}

fn main() {
    // --- Creating and using a struct ---
    let rect1 = Rectangle::new(10.0, 5.0);
    println!("rect1 = {:?}", rect1);
    println!("  area      = {}", rect1.area());
    println!("  perimeter = {}", rect1.perimeter());
    println!("  is_square = {}", rect1.is_square());

    let sq = Rectangle::square(4.0);
    println!("sq = {:?}, is_square={}", sq, sq.is_square());

    // --- Mutable struct + method mutation ---
    let mut rect2 = rect1.clone();
    rect2.scale(2.0);
    println!("rect2 after scale(2): {:?}", rect2);

    // --- Struct update syntax ---
    let rect3 = Rectangle {
        width: 20.0,
        ..rect1 // copy remaining fields from rect1
    };
    println!("rect3 (update syntax): {:?}", rect3);

    // --- Tuple struct ---
    let distance = Meters(42.5);
    println!("distance = {:?}, value = {}", distance, distance.value());
    // The type system prevents mixing up Meters and Kilograms at compile time:
    let _mass = Kilograms(70.0);

    // --- Unit-like struct ---
    let _marker = Marker;
    println!("marker = {:?}", _marker);

    // --- Person struct ---
    let alice = Person::new("Alice", 30);
    println!("{}", alice.greeting());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rectangle_area() {
        let r = Rectangle::new(4.0, 5.0);
        assert!((r.area() - 20.0).abs() < f64::EPSILON);
    }

    #[test]
    fn rectangle_perimeter() {
        let r = Rectangle::new(3.0, 4.0);
        assert!((r.perimeter() - 14.0).abs() < f64::EPSILON);
    }

    #[test]
    fn square_detection() {
        assert!(Rectangle::square(5.0).is_square());
        assert!(!Rectangle::new(3.0, 4.0).is_square());
    }

    #[test]
    fn scale_mutates() {
        let mut r = Rectangle::new(2.0, 3.0);
        r.scale(2.0);
        assert!((r.width  - 4.0).abs() < f64::EPSILON);
        assert!((r.height - 6.0).abs() < f64::EPSILON);
    }

    #[test]
    fn struct_update_syntax() {
        let r1 = Rectangle::new(10.0, 5.0);
        let r2 = Rectangle { width: 20.0, ..r1.clone() };
        assert!((r2.width  - 20.0).abs() < f64::EPSILON);
        assert!((r2.height -  5.0).abs() < f64::EPSILON);
    }

    #[test]
    fn tuple_struct_value() {
        let m = Meters(3.5);
        assert!((m.value() - 3.5).abs() < f64::EPSILON);
    }

    #[test]
    fn person_greeting() {
        let p = Person::new("Bob", 25);
        assert_eq!(p.greeting(), "Hi, I'm Bob and I'm 25 years old.");
    }
}
