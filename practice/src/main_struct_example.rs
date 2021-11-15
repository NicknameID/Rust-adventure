// struct example calculate rectangles area

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

struct RectangleTuple(u32, u32);

fn main() {
    let width1 = 30;
    let height1 = 50;
    println!("The area of the rectangle is {} square pixels.", area(width1, height1));
    
    let rectangle = Rectangle {
        width: 40,
        height: 60,
    };
    println!("The rectangle is {:#?}", &rectangle);
    dbg!(&rectangle);
    println!("The area of the rectangle is {} square pixels.", rectangle_area(&rectangle));

    let rectangle_tuple = RectangleTuple(50, 70);
    println!("The area of the rectangle is {} square pixels.", rectangle_tuple_area(&rectangle_tuple));
}

fn area(with: u32, height: u32) -> u32 {
    return with * height;
}

fn rectangle_area(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height; 
}

fn rectangle_tuple_area(rectangle: &RectangleTuple) -> u32 {
    return rectangle.0 * rectangle.1;
}