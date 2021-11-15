use std::collections::HashMap;

fn main() {
    // 手动创建HashMap
    let mut scores: HashMap<String, i32> = HashMap::new();
    // let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("手动创建HashMap {:#?}", scores);

    // 使用两个vector创建HashMap
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let init_scores = vec![10, 50];
    let vector_scores: HashMap<_, _> = teams.iter().zip(init_scores.iter()).collect();
    println!("通过vector创建HashMap {:#?}", vector_scores);


    // hashMap的key和value进入后所有权将发生转移
    let key = String::from("key");
    let value = String::from("Value");
    let mut map = HashMap::new();
    map.insert(key, value);
    // 这里所有权发生
    // println!("{}: {}", key, value);

    println!("访问HashMap中的值");
    let blue_key = String::from("Blue");
    match scores.get(&blue_key) {
        Some(score) => {
            println!("蓝队的分数为: {}", score)
        },
        None => {
            println!("蓝队还没有分数")
        }
    };

    
    println!("遍历HashMap");
    for (key, value) in &scores {
        println!("key={}, value={}", key, value);
    }

    println!("覆盖一个值");
    scores.insert(blue_key.clone(), 15);
    match scores.get(&blue_key) {
        Some(score) => {
            println!("蓝队的分数为: {}", score)
        },
        _ => ()
    };

    println!("插入键值对前检查：只在没有键时插入");
    scores.entry(blue_key.clone()).or_insert(20);
    scores.entry(blue_key.clone()).or_insert(30);

    println!("根据旧值更新一个值");
    let mut map2: HashMap<_, _> = HashMap::new();
    let text = "hello world wonderful world";
    for word in text.split_whitespace() {
        let count = map2.entry(word).or_insert(0);
        *count = *count + 1;
    }
    println!("{:?}", map2);
}