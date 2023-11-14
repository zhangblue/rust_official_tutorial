//! 使用消息传递在线程间传送数据
//! 多线程好像只能使用main函数启动才能看出效果，使用单元测试进行测试时，貌似是穿行的，并没有多线程

use std::sync::mpsc;
use std::thread;
use std::time::Duration;


/// 创建channel，发送并接收单个数据
fn create_channel_and_send_single_value() {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let value = String::from("hi");
        // send() 夺取了value的所有权
        sender.send(value).unwrap();
    });
    let received = receiver.recv().unwrap();
    println!("Got: {}", received);
}

/// 发送多个值
pub fn create_channel_and_send_multiple_value() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let values = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for value in values {
            tx.send(value).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

/// 创建线程，并使用复制出多个发送者
pub fn create_channel_and_copy_multiple_sender() {
    // 创建发送着与消费者
    let (tx, rx) = mpsc::channel();

    // 克隆发送者
    let tx1 = tx.clone();

    // 开启线程1，使用发送者1进行发送
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

    // 开启线程2，使用发送者2进行发送
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

    // 主线程中使用接受者接收数据
    for received in rx {
        println!("Got: {}", received);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_channel() {
        create_channel_and_send_single_value();
    }

    #[test]
    fn test_create_channel_and_send_multiple_value() {
        create_channel_and_send_multiple_value();
    }
}