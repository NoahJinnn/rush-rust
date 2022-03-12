#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(unused)]
mod my_generic {

    use std::fmt::{Debug, Display};

    // Function
    fn f<T>(x: T) -> T {
        x
    }

    // Struct
    struct TupleS<T>(T);
    struct S<T> {
        item: T,
    }
    struct DoubleS<T1, T2> {
        item1: T1,
        item2: T2,
    }

    // Trait
    trait Tr<T> {
        fn f(x: T) -> T;
    }
    trait DoubleTr<K, V> {
        fn new(k: K, v: V) -> Self;
        fn f(&self, k: K, v: V) -> (K, V);
    }

    // Implementation
    impl<T> S<T> {
        fn new(x: T) -> S<T> {
            S { item: x }
        }
    }
    impl<Idx, Ele> DoubleTr<Idx, Ele> for DoubleS<Idx, Ele> {
        fn new(index: Idx, element: Ele) -> DoubleS<Idx, Ele> {
            DoubleS {
                item1: index,
                item2: element,
            }
        }

        fn f(&self, k: Idx, v: Ele) -> (Idx, Ele) {
            (k, v)
        }
    }

    // Bound
    trait Barrier {
        fn barrier(&self);
    }

    struct BarrierType();
    impl Barrier for BarrierType {
        fn barrier(&self) {
            println!("Barrier");
        }
    }

    fn b_f<T: Barrier>(x: T) -> T {
        x
    }
    struct b_S<T: Barrier> {
        item: T,
    }
    trait b_Tr<T: Barrier> {
        fn new(x: T) -> Self;
        fn f(x: T) -> T;
    }

    impl<T: Barrier> b_Tr<T> for b_S<T> {
        fn new(x: T) -> Self {
            b_S { item: x }
        }
        fn f(x: T) -> T {
            x
        }
    }

    // Multi bound
    fn compare_prints<T: Debug + Display>(t: &T) {
        println!("Debug: `{:?}`", t);
        println!("Display: `{}`", t);
    }

    // `where` clause
    struct YourType {}
    trait MyTrait<T1, T2> {
        fn print_in_option(&self) {
            println!("yo");
        }
    }
    trait TraitB {}
    trait TraitC {}
    trait TraitE {}
    trait TraitF {}
    /* Expressing bounds with a `where` clause for: impl<A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {} */
    impl<A, D> MyTrait<A, D> for YourType
    where
        A: TraitB + TraitC,
        D: TraitE + TraitF,
    {
    }

    // Associated type
    struct Container(i32, i32);
    trait Contains {
        // Define generic types here which methods will be able to utilize.
        type A;
        type B;

        fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
        fn first(&self) -> i32;
        fn last(&self) -> i32;
    }
    impl Contains for Container {
        // Specify what types `A` and `B` are. If the `input` type
        // is `Container(i32, i32)`, the `output` types are determined
        // as `i32` and `i32`.
        type A = i32;
        type B = i32;

        // `&Self::A` and `&Self::B` are also valid here.
        fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
            (&self.0 == number_1) && (&self.1 == number_2)
        }
        fn first(&self) -> i32 {
            self.0
        }
        fn last(&self) -> i32 {
            self.1
        }
    }
    fn difference<C: Contains>(container: &C) -> i32 {
        container.first() - container.last()
    }

    pub fn use_generic() {
        // Explicit generic call
        let numb = f::<i32>(1);
        let s = f::<S<i32>>(S { item: 1 });

        // Implicit generic call
        let numb = f(1);
        let s = f(S { item: 1 });
        let s = S::new(S { item: 1 });
        let ds = DoubleS::new(1, 1);
        let b_S = b_S::new(BarrierType());

        let string = "words";
        let array = [1, 2, 3];

        // Multi bound demo
        compare_prints(&string);
        // compare_prints(&array); -> Error: array is not sastified Display trait
    }
}
