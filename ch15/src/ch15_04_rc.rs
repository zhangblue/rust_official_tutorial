//! Rc<T> 引用计数智能指针
//!
//! 为了启用多所有权需要显式地使用 Rust 类型 Rc<T>，其为 引用计数（reference counting）的缩写。引用计数意味着记录一个值的引用数量来知晓这个值是否仍在被使用。如果某个值有零个引用，就代表没有任何有效引用并可以被清理。
//!
//! 注意 Rc<T> 只能用于单线程场景；



use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}


#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::ch15_04_rc::List::{Cons, Nil};

    /// b和c同时都享有a。在创建b和c时，需要使用clone的方式，每次调用 Rc::clone，Rc<List> 中数据的引用计数都会增加，直到有零个引用之前其数据都不会被清理
    #[test]
    fn rc_test_1() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        let b = Cons(3, Rc::clone(&a));
        let c = Cons(4, Rc::clone(&a));
    }

    #[test]
    fn rc_test_2() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("创建a后计数= {}", Rc::strong_count(&a));
        let b = Cons(3, Rc::clone(&a));
        println!("创建b后计数= {}", Rc::strong_count(&a));
        {
            let c = Cons(4, Rc::clone(&a));
            println!("创建c后计数= {}", Rc::strong_count(&a));
        }
        println!("c超出范围后计数= {}", Rc::strong_count(&a));
    }
}


