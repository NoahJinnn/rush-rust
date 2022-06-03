#[allow(unused)]
pub mod lifetimes {
    use std::fmt::Display;
    // 'a is lifetime annotation constraint that x,y and returned value have the same lifetime
    // the lifetime of the reference returned by the function
    // is the same as the smaller of the lifetimes of the references passed in
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // An instance of ImportantExcerpt can’t outlive the reference it holds in its part field
    struct ImportantExcerpt<'a> {
        part: &'a str,
        // This reference can live for entire duration of the program
        longlive: &'static str,
    }

    impl<'a> ImportantExcerpt<'a> {
        fn notify(&self, msg: &str) -> &str {
            // return type gets lifetime of &self
            self.part
        }
    }

    // Generic type + trait bound + lifetime parameter
    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
