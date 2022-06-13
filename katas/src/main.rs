// pub mod gui;

pub mod gui;

use std::collections::HashMap;
use gui::*;
fn main() {
    find_median_and_mode();
    convert_word_to_pig_latin();
}

fn find_median_and_mode() {
    let mut numbers = vec![6, 3, 3, 9, 10, 2, 2, 3, 4, 5];
    let mut mode = 0;
    let mut freq = 0;
    let mut cnt_map = HashMap::new();
    numbers.sort();
    let median = numbers[numbers.len() / 2];
    for n in numbers {
        let freq = cnt_map.entry(n).or_insert(0);
        *freq += 1;
    }

    for (k, v) in &cnt_map {
        if *v >= freq {
            freq = *v;
            mode = *k;
        }
    }
    println!("median: {}", median);
    println!("mode: {}", mode);
}

fn convert_word_to_pig_latin() {
    let text = "over test";
    let words = text.split_whitespace();
    for w in words {
        let vowels = ['a', 'i', 'o', 'e', 'u'];
        let first_c = w.chars().nth(0).unwrap();
        if vowels.contains(&first_c) {
            let mut pig_latin = String::new();
            pig_latin.push_str(w);
            pig_latin.push_str("-hay");
            println!("{}", pig_latin);
        } else {
            let mut pig_latin = (&w[1..]).to_string();
            pig_latin.push('-');
            pig_latin.push(first_c);
            pig_latin.push_str("ay");
            println!("{}", pig_latin);
        }
    }
}

