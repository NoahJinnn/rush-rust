// Use trait bound to PartialOrd trait and Copy trait to specify type with comparable & stack copiable
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut l = list[0];
    for &item in list {
        if item > l {
            l = item;
        }
    }
    l
}

pub trait Summary {
    fn summarize(&self) -> String {
        format!("Read more")
    }
}

pub struct News {
    pub header: String,
    pub body: String,
}

pub struct Tweet {
    pub title: String,
    pub content: String,
}

impl Summary for News {
    fn summarize(&self) -> String {
        format!("{} - {}", self.header, self.body)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} : {}", self.title, self.content)
    }
}

// pub fn notify(item: &impl Summary) {
//     println!("Breaking news: {}", item.summarize());
// }

// Trait bound syntax, equivalent to above
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news: {}", item.summarize());
}

// Implement ToString trait for every type implement Display trait
// impl<T: Display> ToString for T {
// }

// 'a is lifetime annotation constraint that x,y and returned value have the same lifetime
// it means that the lifetime of the reference returned by the longest function
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
}

// This reference can live for entire duration of the program
// let s: &'static str = "I have a static lifetime.";
