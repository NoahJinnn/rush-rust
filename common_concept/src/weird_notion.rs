#[allow(unused)]
mod special_syntax {
    use std::num::ParseIntError;

    // New type definition
    struct Years(i64); // so call tuple struct
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
            Ok(n) => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    // Trait object
    trait Duck {
        fn quack(&self);
    }

    impl Duck for i32 {
        fn quack(&self) {
            println!("Quack Int!");
        }
    }

    impl Duck for f32 {
        fn quack(&self) {
            println!("Quack Float!");
        }
    }
    // Box<dyn Duck>, &dyn Duck are trait objects
    fn collect_quack_duck(ducks: Vec<Box<dyn Duck>>) {
        for duck in ducks {
            duck.quack();
        }

        let duck_vec: Vec<&dyn Duck> = vec![&43, &3.4];
        for duck in duck_vec {
            duck.quack();
        }
    }
}
