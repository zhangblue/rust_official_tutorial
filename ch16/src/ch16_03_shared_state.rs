//! 共享状态并发
//! 共享内存类似于多所有权：多个线程可以同时访问相同的内存位置。第十五章介绍了智能指针如何使得多所有权成为可能，然而这会增加额外的复杂性，因为需要以某种方式管理这些不同的所有者。Rust 的类型系统和所有权规则极大的协助了正确地管理这些所有权。

use std::sync::{Arc, Mutex};
use std::thread;

/// 互斥器一次只允许一个线程访问数据

/// 在单线程中使用锁
fn create_mutex_lock_in_single_thread() {
    let mutex = Mutex::new(5);

    {
        let mut num = mutex.lock().unwrap();
        *num = 6;
    }

    println!("mutex = {:?}", mutex);
}

/// 在多线程中修改同一个内存，此处需要是Arc智能指针，用于在多线程中共享所要修改值的指针。
pub fn create_multiple_thread() {
    // Arc 智能指针是一个原子指针，可以用在多线程中。Rc无法在多线程中使用
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handlers.push(handle);
    }

    for handle in handlers {
        handle.join().unwrap();
    }

    println!("最终的结果为: {}", *counter.lock().unwrap());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_mutex_lock_in_single_thread() {
        create_mutex_lock_in_single_thread();
    }
}