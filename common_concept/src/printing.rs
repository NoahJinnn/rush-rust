#[allow(unused)]
pub mod printer {
    #[derive(Debug)]
    struct Obj<'a> {
        a: &'a str,
        b: &'a str,
    }
    fn print_prac() {
        println!(
            "This is my {last_msg} and {other}",
            last_msg = "last",
            other = "other"
        );
        let obj = Obj { a: "a", b: "b" };

        // Inline
        println!("{:?}", obj);
        // Prettier
        println!("{:#?}", obj);
        // Get str from printing
        let fmt_str = format!("{0:#?} {1}", obj, "b");
    }
}
