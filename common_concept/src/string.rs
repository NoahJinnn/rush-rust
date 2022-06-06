pub mod about_strings {
    fn string_do() {
        let s = "test".to_string();
        let mut s1 = String::new();
        let slice_s = &s[1..2]; // es

        s1.push_str("yeh");
        let s2 = s + &s1; // concat
        let s3 = format!("{}-{}", s1, s2); // also

        for c in s3.chars() {
            println!("{}", c);
        }
    }
}