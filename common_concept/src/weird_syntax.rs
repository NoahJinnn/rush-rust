mod special_syntax {
    use std::num::ParseIntError;

    // New type definition
    struct Years(i64);
    fn demonstrate() {
        let years = Years(42);
        let years_as_primitive_1: i64 = years.0; // Tuple
        let Years(years_as_primitive_2) = years; // Destructuring   
    }

    // Result Alias
    // Define a generic alias for a `Result` with the error type `ParseIntError`.
    type AliasedResult<T> = Result<T, ParseIntError>;
    fn print(result: AliasedResult<i32>) {
        match result {
            Ok(n)  => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }
}
