struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    w: u16,
    h: u16,
}

impl Rectangle {
    fn area(&self) -> u16 {
        self.h * self.w
    }

    fn square(size: u16) -> Rectangle {
        Rectangle { w: size, h: size }
    }
}

fn struct_demo() {
    let u1 = User {
        email: String::from("aa@t.com"),
        username: String::from("aa"),
        active: false,
        sign_in_count: 0,
    };

    let email = String::from("bb@t.com");
    let mut mu1 = User {
        email,
        username: String::from("aa"),
        active: false,
        sign_in_count: 0,
    };
    mu1.email = String::from("test");

    let u3 = User {
        email: String::from("cc@t.com"),
        ..u1 // u1 no longer be used, bc name value is move into u3
    };

    struct Color(i32, i32, i32); // tuple struct
    let black = Color(0, 0, 0);

    struct AlwaysEqual; // unit-like struct
    let subject = AlwaysEqual;

    // Rectangle calculation
    let ar = Rectangle { w: 10, h: 30 };
    println!("{:#?}", Rectangle::square(10));
}