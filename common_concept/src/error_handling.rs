#[allow(unused)]
mod errr {
    use std::{error::Error, num::ParseIntError, fmt::{Display, Formatter}};
    
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

    fn handle_result(msg: Result<&str, Box<dyn Error>>) -> Result<&str, Box<dyn Error>> {
        let pan_m = msg.as_ref().unwrap(); // panic if Err
        let err_m = msg?; // return Err if Err
        return Ok(err_m);
        // Result's map & combinator
        fn multiply(first: &str, sec: &str) -> Result<i32, ParseIntError> {
            first.parse::<i32>().and_then(|first_number| {
                sec.parse::<i32>()
                    .map(|second_number| first_number * second_number)
            })
        }
    }

    fn handle_multi_err_types() {
        fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
            vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n))
        }

        // When we care about None case
        // fn double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
        //     let opt = vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n));

        //     opt.map_or(Ok(None), |r| r.map(Some))
        // }

        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        println!("The first doubled is {:?}", double_first(numbers)); // Some(Ok(84)) || Ok(Some(84))
        println!("The first doubled is {:?}", double_first(empty)); // None || Ok(None)
        println!("The first doubled is {:?}", double_first(strings)); // Some(Err(ParseIntError { kind: InvalidDigit })) || Err(ParseIntError { kind: InvalidDigit })
    }

    // Custom Error
    #[derive(Debug, Clone)]
    struct DoubleError;
    type CusResult<T> = Result<T, DoubleError>;
    impl Display for DoubleError {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "invalid first item to double")
        }
    }

    impl Error for DoubleError {
        fn description(&self) -> &str {
            "invalid first item to double"
        }
    }

    // Dynamic Error, error type is determined at runtime, use when you don't know the error type at compile time
    type DynResult<T> = Result<T, Box<dyn Error>>;
}
