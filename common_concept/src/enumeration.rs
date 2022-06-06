#[derive(Debug)]
enum IpAddr {
    V4(u16, u16, u16, u16),
    V6(String),
}

impl IpAddr {
    fn call(&self) {
        println!("{:#?}", self)
    }
}

fn enum_demo() {
    let ipa = IpAddr::V6(String::from("Test"));
    ipa.call();
}
