fn normal_match() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
}

fn simple_match() {
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

fn main() {
    // 样板代码
    println!("match的样板代码: {:?}", normal_match());
    println!("if let语法糖: {:?}", simple_match());
}