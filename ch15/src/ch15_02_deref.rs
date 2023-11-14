//! 通过 Deref trait 将智能指针当作常规引用处理
//!
//! Deref 强制转换如何与可变性交互
//!
//! 类似于如何使用 Deref trait 重载不可变引用的 * 运算符，Rust 提供了 DerefMut trait 用于重载可变引用的 * 运算符。
//!
//! Rust 在发现类型和 trait 实现满足三种情况时会进行 Deref 强制转换：
//! - 当 T: Deref<Target=U> 时从 &T 到 &U。
//! - 当 T: DerefMut<Target=U> 时从 &mut T 到 &mut U。
//! - 当 T: Deref<Target=U> 时从 &mut T 到 &U。
//!
//! 头两个情况除了第二种实现了可变性之外是相同的：第一种情况表明如果有一个 &T，而 T 实现了返回 U 类型的 Deref，则可以直接得到 &U。第二种情况表明对于可变引用也有着相同的行为。
//!
//! 第三个情况有些微妙：Rust 也会将可变引用强转为不可变引用。但是反之是 不可能 的：不可变引用永远也不能强转为可变引用。因为根据借用规则，如果有一个可变引用，其必须是这些数据的唯一引用（否则程序将无法编译）。将一个可变引用转换为不可变引用永远也不会打破借用规则。将不可变引用转换为可变引用则需要初始的不可变引用是数据唯一的不可变引用，而借用规则无法保证这一点。因此，Rust 无法假设将不可变引用转换为可变引用是可能的。


use std::ops::Deref;

/// 自定义MyBox 用于模拟实现Box<T>的功能
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T; // 定义了用于此 trait 的关联类型。关联类型是一个稍有不同的定义泛型参数的方式

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


#[cfg(test)]
mod tests {
    use crate::ch15_02_deref::MyBox;

    /// 普通的解引用测试
    #[test]
    fn normal_dereference() {
        let x = 5;
        let y = &5;

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    /// Box<T> 上使用的解引用运算符
    /// 将 y 设置为一个指向 x 值拷贝的 Box<T> 实例，而不是指向 x 值的引用
    #[test]
    fn box_dereference() {
        let x = 5;
        let y = Box::new(5);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    /// 使用MyBox
    #[test]
    fn my_box_dereference() {
        let x = 5;
        let y = MyBox::new(5);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
}