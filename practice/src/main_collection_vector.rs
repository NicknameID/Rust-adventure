fn main() {
    let mut v_new: Vec<i32> = Vec::new();
    let mut v_macro = vec![1,2,3,4];

    // 推入元素
    v_new.push(5);

    // v_macro中第三个元素的引用
    let third_ref: &i32 = &v_macro[2];
    // v_macro中第三个元素进行拷贝
    let third_copy: i32 = v_macro[2];

    match v_macro.get(2) {
        Some(third_ref) => println!("The third element is {}", third_ref),
        None => println!("There is no third element."),
    }

    let v_new_first = &v_new[0];
    println!("The first element of v_new is: {}", v_new_first);
    v_new.push(6);
    let v_new_first = &v_new[0];
    println!("The first element of v_new is: {} after changed", v_new_first);

    // 遍历vec
    for i in &v_macro {
        println!("{}", i);
    }
    // 遍历修vector
    for i in &mut v_macro {
        *i = *i + 1;
    }
    for i in &v_macro {
        println!("{}", i);
    }


    // 使用枚举存储不同类型的值
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];
}