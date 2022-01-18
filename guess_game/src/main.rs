use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    loop {
        println!("Guess!");
        println!("Please input your guesses:");
        let secret_numb = rand::thread_rng().gen_range(1..101);
        let mut g = String::new();
        io::stdin() // Return an instance std::io::Stdin
            .read_line(&mut g) // Read stdin into g, return io::Result
            .expect("Fail to read"); // Print "Fail to read" when there's problem while reading stdin
        println!("Your guess: {}", g);
        let g: u32 = match g.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match g.cmp(&secret_numb) {
            Ordering::Less => println!("Smaller"),
            Ordering::Greater => println!("Bigger"),
            Ordering::Equal => {
                println!("Win");
                break;
            }
        }
    }
}
