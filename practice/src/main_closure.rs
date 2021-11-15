use std::collections::HashMap;
use std::thread;
use std::time::Duration;

struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: HashMap<String, u32>
}

impl <T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new()
        }
    }

    fn value(&mut self, args: u32) -> u32 {
        let key = args.to_string();
        match self.value.get(&key) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(args);
                self.value.insert(key, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

fn closure_context() {
    let x = 4;
    let equal_to_x = |z| { z == x };
    let y = 4;
    assert!(equal_to_x(y))
}

fn closure_contrxt_fnOnce() {
    let x = vec![1,2,3];
    let equals_to_x = move |z| z == x;
    // println!("can't use x here: {:?}", x);
    let y = vec![1, 2, 3];
    assert!(equals_to_x(y));
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    // 利用闭包的进行惰性求值
    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    // 闭包获取上下文
    closure_context();

    // 闭包从上下文中移动所有权
    closure_contrxt_fnOnce();

}
