mod collections;
mod concurrency;
mod error_handling;
mod generic;
mod lifetimes;
mod printing;
mod weird_notion;
mod closure;
mod smart_pointer;
mod about_strings;

#[allow(unused)]
fn main() {
    // --- tuple ---
    let tup = (500, 6.4, 1);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let (x, y, z) = tup; // destructing

    // --- array ---
    let a = [1, 2, 3];
    let a = [3; 5]; // [init value, length]
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    // --- expresion ---
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y); // 4

    let number = if true { 5 } else { 6 }; // values in both condition block must be the same type

    'lb1: loop {
        loop {
            if true {
                break; // break inner loop
            }
            if true {
                break 'lb1; // break outer loop
            }
        }
    }

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }
    for number in (1..4).rev() {
        println!("{}!", number);
    }

}
