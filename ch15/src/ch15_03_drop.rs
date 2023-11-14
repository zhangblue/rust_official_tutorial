//! 使用 Drop Trait 运行清理代码

/// 自定义一个实现了Drop trait的类，当离开作用域时会自动调用drop方法
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("删除了数据：`{}`!", self.data);
    }
}


/// Rust 不允许我们显式调用 drop 因为 Rust 仍然会在 main 的结尾对值自动调用 drop，这会导致一个 double free 错误，因为 Rust 会尝试清理相同的值两次。
/// 如果需要主动调用drop时，应当使用 std::mem::drop 函数
pub fn drop_by_user() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    // 调用 std::mem::drop()
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}

#[cfg(test)]
mod tests {
    use crate::ch15_03_drop::{CustomSmartPointer, drop_by_user};


    #[test]
    fn test_drop() {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers created.");
    }

    /// 强制调用drop方法，可以提早的丢弃某个值
    #[test]
    fn test_drop_2() {
        drop_by_user();
    }
}