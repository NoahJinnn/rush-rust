#[allow(unused)]
mod errr {

    #[derive(Debug)]
    enum Food {
        Apple,
        Carrot,
        Potato,
    }

    #[derive(Debug)]
    struct Peeled(Food);
    #[derive(Debug)]
    struct Chopped(Food);
    #[derive(Debug)]
    struct Cooked(Food);

    fn handle_option(msg: Option<&str>) -> Option<&str> {
        panic!("For unrecoverable err!!!!");
        let pan_m = msg.unwrap(); // panic if None
        let cus_pan_m = msg.expect("custom msg for panic"); // panic if None
        let default_m = msg.or_else(|| Some("default msg")); // return default msg if None
        let none_m = msg?; // return None if no Some
        // Option's map & combinator
        let chopped = Some(Food::Apple);
        chopped.map(|food| Cooked(food));
        fn process(food: Option<Food>) -> Option<Food> {
            // Cookings food
            food.map(|f| Peeled(f))
                .map(|Peeled(f)| Chopped(f))
                .map(|Chopped(f)| Cooked(f))
                .and_then(|Cooked(f)| Some(f))
            }
    }

    fn handle_result() {}
}
