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

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // equivalent with _ => doSth()
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}