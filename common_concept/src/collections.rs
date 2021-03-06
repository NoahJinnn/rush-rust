#[allow(unused)]
pub mod collections {
    use std::collections::HashMap;

    pub fn vector_do() {
        let mut emp_v: Vec<u32> = Vec::new();
        let mut v = vec![1, 2, 3];

        // Read
        println!("{}", &v[0]);
        match v.get(1) {
            Some(sec) => println!("{}", sec),
            None => println!("No sec ele"),
        }
        // Write
        v.push(4);
        for val in &mut v {
            *val += 1;
            println!("{}", val);
        }
        for val in v {
            println!("{}", val);
        }
        // Utils
        let mut v2 = vec![2, 4, 5, 1];
        v2.sort();
    }

    fn iterator_do() {
        let arr = [1, 2, 3];
        for i in arr.iter() {
            println!("{}", i);
        }
        for i in &arr {
            println!("{}", i);
        } // Slice to iterator implicitly
        for w in arr.windows(2) {
            println!("window {:?}", w);
        } // window [1,2], window [2,3]
        for ch in arr.chunks(2) {
            println!("chunk {:?}", ch);
        } // chunk [1,2], chunk [3]
        let name = ["one", "two", "three"];
        let num = [1,2,3];
        for tpl in name.iter().enumerate() {
            println!("{:?}", tpl);
        } // (0, "one"), (1, "two"), (2, "three")
        for p in name.iter().zip(num.iter()) {
            println!("{:?}", p);
        } // ("one", 1), ("two", 2)

        // Functional programming
        let numbers = vec![1, 2, 3];
        numbers.iter().for_each(|x| println!("{}", x));
        let doubled: Vec<_> = numbers.iter().map(|num| num * 2).collect(); // Need type annotate for Vec, already known i32
        let even: Vec<_> = numbers.iter().filter(|x| *x % 2 == 0).collect();
        let first_even = numbers.iter().find(|x| *x % 2 == 0);
        println!(
            "{:?} {:?} {:?}",
            doubled.get(0),
            even.get(0),
            first_even.unwrap()
        );
    }

    fn map_do() {
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
            println!("{:?}", map.get(key));
        }
    }
}
