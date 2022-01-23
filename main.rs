mod collections;
mod generic_trait_lifetime;
use generic_trait_lifetime::{Summary, Tweet};

fn main() {
    println!("Hello wor");
    collections::vector_demo();
    let tweet = Tweet {
        title: "Test".to_string(),
        content: "Test".to_string(),
    };
    println!("{}", tweet.summarize());
}

// Return trait
fn return_summarizable() -> impl Summary {
    Tweet {
        title: "Test".to_string(),
        content: "Test".to_string(),
    }
}
