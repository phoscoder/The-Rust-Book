
use trait_objects::{Draw, Screen, Button};


struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("yes"),
                    String::from("maybe"),
                    String::from("no"),
                ],
            }),
        ],
    };

    screen.run();
}
