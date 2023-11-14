//! 使用Box<T>指向堆上的数据
//!
//! 使用场景：
//! - 当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候
//! - 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
//! - 当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候
//!
//! Box<T> 实现了 `Deref trait` 和 `Drop trait`
//! - `Deref trait`: 允许 `Box<T>` 值被当作引用对待
//! - `Drop trait`：允许 `Box<T>` 值离开作用域时, box 所指向的堆数据也会被清除


use crate::ch15_01_box::List::{Cons, Nil};

/// 使用box将数据存储在堆上。
pub fn create_in_heap() {
    let b = Box::new(5);
    println!("b = {}", b)
}

/// 在不知道数据大小的情况下，需要使用box将数据存储在堆上。比如这种递归的存储，因为无法预知递归存储的深度。
pub fn create_in_heap2() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

pub enum List {
    Cons(i32, Box<List>),
    Nil,
}


#[cfg(test)]
mod tests {
    use super::*;

    ///
    #[test]
    fn test01() {
        create_in_heap();
    }

    #[test]
    fn test02() {
        create_in_heap2();
    }
}