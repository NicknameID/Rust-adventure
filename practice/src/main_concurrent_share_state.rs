use std::ops::Deref;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

fn first_try_mutex() {
    let m = Mutex::new(5);
    println!("m = {:?}", m);
    {
        let mut  num = m.lock().unwrap();
        *num = 6;

        // mutexGuard是一个智能指针，实现Deref和Drop，在离开作用域后会自动释放锁
    }
    println!("m = {:?}", m);
}


fn share_mutex_between_threads() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn main() {
    // first_try_mutex();
    share_mutex_between_threads();
}
