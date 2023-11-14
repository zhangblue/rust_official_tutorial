//! RefCell<T> 和内部可变性模式
//!
//! 当创建不可变和可变引用时，我们分别使用 `&` 和 `&mut` 语法。对于 `RefCell<T>` 来说，则是 `borrow` 和 `borrow_mut` 方法，这属于 `RefCell<T>` 安全 API 的一部分。`borrow` 方法返回 `Ref<T>` 类型的智能指针，`borrow_mut` 方法返回 `RefMut<T>` 类型的智能指针。这两个类型都实现了 `Deref`，所以可以当作常规引用对待。
//!
//! `RefCell<T>` 记录当前有多少个活动的 `Ref<T>` 和 `RefMut<T>` 智能指针。每次调用 `borrow`，`RefCell<T>` 将活动的不可变借用计数加一。当 `Ref<T>` 值离开作用域时，不可变借用计数减一。就像编译时借用规则一样，`RefCell<T>` 在任何时候只允许有多个不可变借用或一个可变借用。
//!
//! 如果我们尝试违反这些规则，相比引用时的编译时错误，`RefCell<T>` 的实现会在运行时出现 `panic`。


use std::cell::RefCell;
use std::rc::Rc;
use crate::ch15_05_interior_mutability::List::{Cons, Nil};

///! 内部可变形的用例：mock对象

pub trait Messenger {
    fn send(&self, msg: &str);
}

/// 此类需要添加生命周期标记，因为此对象要保证传入的message对象始终有效
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max > 1.0 {
            self.messenger.send("错误：你超过了限制!")
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("紧急警告：你已经用完了90%以上的配额！");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("警告：你已经用完了75%以上的配额！");
        }
    }
}

/// 结合 Rc<T> 和 RefCell<T> 来拥有多个可变数据所有者
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

/// Rc<T> 与 RefCell<T> 共同使用
pub fn rc_and_refcell() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use super::*;

    /// 定义mock的消息发送器
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            // self.sent_messages 中 RefCell 的 borrow_mut 方法来获取 RefCell 中值的可变引用
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        // 调用 RefCell 的 borrow 以获取 vector 的不可变引用
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }

    #[test]
    fn test_rc_and_refcell() {
        rc_and_refcell();
    }
}
