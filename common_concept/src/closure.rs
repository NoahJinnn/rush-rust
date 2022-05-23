#[allow(unused)]
pub mod closure {
    use std::{thread, time::Duration};

    pub struct Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        pub calculation: T,
        pub value: Option<u32>,
    }

    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(calc: T) -> Self {
            Cacher {
                calculation: calc,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

    fn generate_workout(intensity: u32, random_number: u32) {
        let mut expensive_result = Cacher::new(|num| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        });

        if intensity < 25 {
            println!("Today, do {} pushups!", expensive_result.value(intensity));
            println!("Next, do {} situps!", expensive_result.value(intensity));
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!(
                    "Today, run for {} minutes!",
                    expensive_result.value(intensity)
                );
            }
        }
    }

    fn functional_programming() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let v_map: Vec<i32> = v.iter().map(|x| x * 2).collect();
        let v_filter: Vec<&i32> = v.iter().filter(|&x| *x % 2 == 0).collect();
        let find = v.iter().find(|&x| *x == 4).unwrap();
        let pair_sum = v.into_iter().reduce(|base, next| base + next);
    }
}
