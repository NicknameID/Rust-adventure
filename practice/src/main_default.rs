// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {}{}", value, unit_label);
// }
fn takes_ownership(some_str: String) {
    println!("{}", some_str);
}
fn main() {
    // println!("Hello, world!");
    // print_labeled_measurement(5, 'h');
    // let mut hello = String::from("Hello");
    // let world = String::from("Wrodl");

    // hello += &world;
    // hello.push_str(&world);

    // println!("{}", hello);

    let mut x = 5;
    let y = x;
    println!("x={}, y={}", x, y);
    x += 1;
    println!("x={}, y={}", x, y);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1={},s2={}", s1, s2);


    takes_ownership(s1);
}
