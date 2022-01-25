fn reference_demo() {
    let s1 = String::from("Ref");
    let len = calculate_length(&s1);
    print!("{}", s1);
    let mut s = String::from("Mut Ref");
    change(&mut s);
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn change(s: &mut String) {
    s.push_str("Will Mut");
}
