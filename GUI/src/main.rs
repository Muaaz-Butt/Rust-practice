extern crate GUI;
use GUI::{Draw, Screen, Button};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        //Code to actually draw a select box
    }
}

fn main() {
    let screen = Screen{
        components: vec![
            Box::new(SelectBox {  
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("May be"),
                    String::from("No")
                ],
            }),
            Box::new(Button {
                width: 50, 
                height: 10,
                label: String::from("OK"),
            })
        ]
    };
    screen.run();
}