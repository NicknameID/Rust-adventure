pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>
}

impl Screen  {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}


struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Draw button!")
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Draw SelectBox!")
    }
}

fn main() {
    let select_box = Box::new(
        SelectBox {
            width: 75,
            height: 10,
            options: vec![
                String::from("Yes"),
                String::from("Maybe"),
                String::from("No")
            ],
        }
    );
    let button = Box::new(
        Button {
            width: 50,
            height: 10,
            label: String::from("OK"),
        }
    );
    let screen = Screen {
        components: vec![select_box, button]
    };
    screen.run();
}
