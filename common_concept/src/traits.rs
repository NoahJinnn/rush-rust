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


// Implement ToString trait for every type implement Display trait
// impl<T: Display> ToString for T {
// }