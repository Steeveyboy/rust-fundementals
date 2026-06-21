// use std::range
fn main(){
    let mut count: i32 = 10; 
    loop {
        println!("count is {count}");
        count -= 1;
        if count <=0 {
            break
        }
    }

    let mut number = 3;
    while number > 0 {
        println!("INSIDE THE WHILE LOOP {number}");
        number -= 1;
    }

    let a = [1,2,3];

    let mut index = 0;
    while index < a.len() {
        println!("LOOPING THE COLLECTION {}", a[index]);
        index += 1;
    }

    for num in 4..7 {
        println!("{num}!");
    }


}
