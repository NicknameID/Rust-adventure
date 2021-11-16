use std::ops::Deref;
use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Box<List>),
    Nil,
}

// 自定义智能指针
struct MyBox<T>(T);

impl <T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl <T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox with data")
    }
}

fn main() {
    // 在堆上申请空间
    let x = 5;
    let b = MyBox::new(x);
    println!("MyBox created.");
    println!("b = {}", *b);

    let list = Cons(1, Box::new(Cons(2,
                                     Box::new(Cons(3,
                                                   Box::new(Nil))))));

    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
}
