use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

fn send_once_producer() -> Receiver<String> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
    rx
}

fn send_multiple_producer() -> Receiver<String> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    rx
}

fn block_channel(receive: &Receiver<String>) {
    // rx.recv()方法会阻塞等待发送端的发送数据。如果收到消息会返回Ok, 如果收到Err表示通道发送端关闭
    let received = receive.recv().unwrap();
    println!("Got: {}", received);
}

fn unblock_channel(receive: &Receiver<String>) {
    // rx.try_recv()方法不会阻塞，会返回一个Result，Ok表示有消息，Err()表示没有消息
    loop {
        match receive.try_recv() {
            Ok(received) => {
                println!("Got: {}", received);
                break;
            },
            Err(_) => {
                println!("nothing got");
                continue
            }
        }
    }
}

fn iter_receiver(rx: Receiver<String>) {
    for receive in rx {
        println!("Got: {}", receive)
    }
}

fn multiple_sender() {
    // 多个生产者往同一个接收器channel发送
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    // tx1发送者线程
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    // tx发送者线程
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });


    for received in rx {
        println!("Got: {}", received);
    }
}

fn main() {
    let send_once_rx = send_once_producer();
    // 阻塞
    block_channel(&send_once_rx);
    // 非阻塞
    unblock_channel(&send_once_rx);

    // 使用迭代器消费channel的rx
    let send_multiple_rx = send_multiple_producer();
    iter_receiver(send_multiple_rx);

    // 多个发送者的情况
    multiple_sender();

}
