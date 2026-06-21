

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {

    let truncate = -5.5 / 3.0;
    println!("Truncate: {truncate}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {y}");



    let mut mutable_array: [i32; 4] = [1, 2, 3, 4];
    mutable_array[0] = 5;
    println!("The first number is: {}", mutable_array[0]);


    let mut first_entry: i32 = 11; //#[allow(unused_assignments)] 
    let second_entry: i32 = 2;
    let immutable_array = [&first_entry, &second_entry]; // first_entry is mutable, but when we reference it in the array, it becomes immutable
    println!("Array: {:?}", immutable_array);

        

    // first_entry = 22;   // Because first_entry is borrowed as immutable in the array, this will cause a compile-time error;
    // println!("Array: {:?}", immutable_array);


    let immutable_entry: i32 = 111;
    let mut mutable_array = [&immutable_entry, &second_entry]; // immutable_entry is immutable, but when we reference it in the array, it becomes mutable
    println!("Array: {:?}", mutable_array);

    mutable_array[0] = &222; // This is allowed because mutable_array is mutable, even though immutable_entry is immutable
    mutable_array[1] = &333;
    println!("Array: {:?}", mutable_array);
    println!("Immutable entry: {}", immutable_entry); // Immutable entry remains unchanged, demonstrating that the array holds references to the values, not the values themselves

    let x = plus_one(5);
    println!("5 plus one is: {x}");

}



