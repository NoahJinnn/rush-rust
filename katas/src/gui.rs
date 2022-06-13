pub mod gui {
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen<T: Draw> {
        pub components: Vec<Box<T>>,
    }

    impl<T> Screen<T>
    where
        T: Draw,
    {
        pub fn run(&self) {
            for comp in self.components.iter() {
                comp.draw();
            }
        }
    }

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            println!("Drawing Button with width: {} and height: {}", self.width, self.height);
        }
    }

    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }
    
    impl Draw for SelectBox {
        fn draw(&self) {
            println!("Drawing SelectBox with width: {} and height: {}. Options {:?}", self.width, self.height, self.options);
        }
    }
}
