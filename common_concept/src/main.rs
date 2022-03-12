mod collections;
mod concurrency;
mod generic;
mod generic_trait_lifetime;

use crate::generic_trait_lifetime::{Summary, Tweet};

#[allow(unused)]
fn main() {
    println!("Hello, world!");
    let a = 4; // dafault to immutable
    let mut a = 3; // mutable variable
    const IM_A_CONSTANT: u32 = 6; // constant

    // --- tuple ---
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup; // destructing
    println!("The value of y is: {}", y);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    // --- array ---
    let a = [1, 2, 3];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // [init value, length]
    let first = a[0];
    let second = a[1];
    // --- expresion ---
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y); // 4
    println!("The value of x is: {}", five()); // 5
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

    println!("Hello wor");
    collections::vector_demo();
    let tweet = Tweet {
        title: "Test".to_string(),
        content: "Test".to_string(),
    };
    println!("{}", tweet.summarize());
}

// Return value function
fn five() -> i32 {
    5 // equivalent to return 5;
}

// Return trait
fn return_summarizable() -> impl Summary {
    Tweet {
        title: "Test".to_string(),
        content: "Test".to_string(),
    }
}

#[derive(Debug)]
struct Obj<'a> {
    a: &'a str,
    b: &'a str,
}
fn print_prac() {
    println!(
        "This is my {last_msg} and {other}",
        last_msg = "last",
        other = "other"
    );
    let obj = Obj { a: "a", b: "b" };

    // Inline
    println!("{:?}", obj);
    // Prettier
    println!("{:#?}", obj);
    let fmt_str = format!("{0:#?} {1}", obj, "b");
}
