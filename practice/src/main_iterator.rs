struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            count: 0,
        }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        }else {
            None
        }
    }
}

fn calling_next_directly() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

fn using_other_iterator_trait_methods() {
    // 0，1，2，3，4，None
    // 1，2，3，4，None
    //       |  a * b
    //       V
    // 0，2，6，12
    //       | x % 3 == 0
    //       V
    // 0, _, 6, 12
    //       | sum()
    //       V
    // 0 + 6 + 12 = 18
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}

fn main() {
    let mut v1 = vec![1,2,3];
    // 不可变引用迭代器，不获取所有权 iter()
    let v1_iter = v1.iter();
    // 可变引用迭代器，不获取所有权: iter_mut()
    let mut v1_iter = v1.iter_mut();
    // 不可变引用迭代器，获取所有权
    let v1_ownership = v1.into_iter();
    // println!("{:?}", v1_ownership); // 这一行编译会报错

    let v1 = vec![1,2,3];
    // 迭代器的使用1：for循环
    for val in v1.iter() {
        println!("Got: {}", val);
    }

    // 迭代器的使用2：sum方法
    let v1 = vec![1,2,3];
    let v1_total: i32 = v1.iter().sum();
    print!("v1_total: {}", v1_total);

    // 迭代器使用3：map()适配器, 并用collect()进行收据，类似Java里的Stream
    let v1 = vec![1,2,3];
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    println!("v1 {:?} map to v2: {:?}", v1, v2);

    // 迭代器使用4：into_iter()、filter()适配器
    let v1 = vec![1,2,3];
    let query = 2;
    let v2: Vec<i32> = v1.into_iter().filter(|x| x == &query).collect();
    println!("v2: {:?} filter by {}", v2, query);

    // 自定义结构体实现Iterator接口
    calling_next_directly();
    //
    using_other_iterator_trait_methods();
}
