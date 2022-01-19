fn main() {
    let str = String::from("Test"); // str comes into scope

    take_ownership(str); // str's value moves into the function...
                         // ... and so str is no longer valid here, error to use str afterward

    let num = 3; // num comes into scope

    make_copy(num); // num would move into the function, but i32 is Copy, so it's okay to still
                    // use num afterward

    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3

    // --- return multiple values ---
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
} // Here, num goes out of scope, then str. But because str's value was moved, nothing
  // special happens.
  // s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn take_ownership(target: String) {
    println!("{}", target);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn make_copy(target: i32) {
    println!("{}", target)
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
