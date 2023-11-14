//! 使用线程同时运行代码

use std::thread;
use std::time::Duration;

/// 使用spawn 创建新线程。主线程执行完毕后，无论子线程是否完成，都会结束
fn create_thread_use_spawn() {
    // 创建线程打印数据
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 在主线程中打印数据
    for i in 1..2 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

/// 使用join()函数等待子线程完成
fn wait_thread_use_join() {
    // 创建线程打印数据
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 在主线程中打印数据
    for i in 1..3 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // 使用join()函数等待子线程结束
    handle.join().unwrap();
}

/// 将参数所有权移动至线程闭包中. 使用move关键上下文中的变量的所有权移动到线程中
fn thead_ownership_movement() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_thread_use_spawn() {
        create_thread_use_spawn();
    }

    #[test]
    fn test_wait_thread_use_join() {
        wait_thread_use_join();
    }

    #[test]
    fn test_thead_ownership_movement(){
        thead_ownership_movement();
    }

}