use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

fn create_thread() -> JoinHandle<()> {
    let handle = thread::spawn(|| {
        for i in 1..100 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle
}

fn thread_join() {
    let handle = create_thread();
    // 使用 join 等待所有线程结束
    handle.join().unwrap();
}

// 线程与 move 闭包
fn thread_closure_move_ownership() {
    // move 关键字强制闭包获取其使用的环境值的所有权。这个技巧在创建新线程将值的所有权从一个线程移动到另一个线程时最为实用
    let v = vec![1, 2, 3];
    // 闭包会自动推测对v的使用是 FnOnce、FnMut、Fn。这里默认是Fn既保守的不可变引用，
    // 但因为v是定义在闭包的外部线程即主线程中，无法估计子线程的执行时间所以无法判断v的引用是否一直有效。
    // 所以这里应该使用move指令，告诉编译器使用FnOnce这个trait
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    // drop(v);
    handle.join().unwrap();
}

fn main() {
    // 单自定义创建的线程的理论执行时间大于主线程的执行时间，当主线程执行完成后程序会退出，手动创建的线程不一定能执行完成
    // create_thread();

    // 使用 join() 等待子线程完成后再退出程序
    // thread_join();

    thread_closure_move_ownership();
}
