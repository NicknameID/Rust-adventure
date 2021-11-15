fn main() {
    // String类型
    let mut s: String = String::from("initial contents");

    // String的slice
    let mut s_slic: &str = "initial contents";

    // string slice 变为 String
    s = s_slic.to_string();

    // String 变为 string slice
    s_slic = &s[..];

    // 更新字符串
    s.push_str("push_str");

    let mut lol_part: String = String::from("lo");
    lol_part.push('p');
    println!("{}", lol_part);

    // 使用 + 运算符
    let s1 = String::from("hello, ");
    let s2 = String::from("world");
    let s3 = s1 + &s2;
    // format! 宏拼接字符串
    let tic = String::from("tic");
    let tac = String::from("toc");
    let toe = String::from("toe");
    let tic_tac_toe = format!("{}-{}-{}", tic, tac, toe);

    // 遍历字符串
    println!("准备遍历字符串, '你好'");
    for c in "你好".chars() {
           println!("{}", c);
    }
    println!("遍历完成");

    // 遍历原始字节
    println!("准备遍历字符串，‘你好’的原始字节");
    for b in "你好".bytes() {
        println!("{}", b);
    }
    println!("准备遍历字符串‘你好’的原始字节完成");
}