use std::collections::HashMap;

pub fn vector_demo() {
    let mut emp_v: Vec<u32> = Vec::new(); // Like List in java
    emp_v.push(1);
    let mut v = vec![1, 2, 3];
    println!("{}", &emp_v[0]);
    match v.get(1) {
        Some(sec) => println!("{}", sec),
        None => println!("No sec ele"),
    }
    for val in &mut v {
        *val += 1;
        println!("{}", val);
    }

    for val in v {
        println!("{}", val);
    }

    let s = "test".to_string();
    let mut s1 = String::new();
    s1.push_str("yeh");
    let s2 = s + &s1; // concat
    let s3 = format!("{}-{}", s1, s2); // also
    for c in s3.chars() {
        println!("{}", c);
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value); // field_name, field_value is moved to map

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    println!("{:?}", map);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get("Blue");
    println!("{:?}", score);
}
