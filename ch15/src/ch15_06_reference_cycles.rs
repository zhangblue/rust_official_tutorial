//! 循环引用与内存泄漏


use std::cell::RefCell;
use std::rc::{Rc, Weak};


#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    /// 用于获取List中的第二个值
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            List::Cons(_, item) => Some(item),
            List::Nil => None
        }
    }
}

/// 弱引用
/// 为了使子节点知道其父节点，需要在 Node 结构体定义中增加一个 parent 字段。问题是 parent 的类型应该是什么。我们知道其不能包含 Rc<T>，因为这样 leaf.parent 将会指向 branch 而 branch.children 会包含 leaf 的指针，这会形成引用循环，会造成其 strong_count 永远也不会为 0。
/// 现在换一种方式思考这个关系，父节点应该拥有其子节点：如果父节点被丢弃了，其子节点也应该被丢弃。然而子节点不应该拥有其父节点：如果丢弃子节点，其父节点应该依然存在。
#[derive(Debug)]
struct Node {
    value: i32,
    // 用于子节点关联父节点。因为子节点并不拥有父节点，所以此处
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}


#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};
    use crate::ch15_06_reference_cycles::List::{Cons, Nil};
    use crate::ch15_06_reference_cycles::Node;

    /// 制造循环引用的场景
    #[test]
    fn create_circular_reference() {
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
        println!("a 初始化之后，rc count = {}", Rc::strong_count(&a));
        println!("a 的第二个参数是：{:?}", a.tail());
        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
        println!("b 创建后 a 的 rc count = {}", Rc::strong_count(&a));
        println!("b 初始化后的 rc count = {}", Rc::strong_count(&b));
        println!("b 的第二个参数是：{:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }
        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));
    }

    /// 测试使用弱引用
    #[test]
    fn test_weak_reference() {

        // 创建子节点leaf
        let leaf = Rc::new(Node {
            // 子节点的值为3
            value: 3,
            // 此时子节点的父节点是空的
            parent: RefCell::new(Weak::new()),
            // 子节点的子节点是空的列表
            children: RefCell::new(vec![]),
        });

        println!("leaf 父节点 parent = {:?}", leaf.parent.borrow().upgrade());

        // 创建父节点 branch
        let branch = Rc::new(Node {
            // 父节点的值为5
            value: 5,
            // 父节点的父节点是空的
            parent: RefCell::new(Weak::new()),
            // 父节点的子节点是 leaf
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        // 更改子节点leaf的父节点为branch，使用弱引用
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!("leaf 父节点 parent = {:?}", leaf.parent.borrow().upgrade());
    }

    /// 用于测试弱引用计数器
    #[test]
    fn test_weak_reference_count() {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!(
            "leaf 强引用个数 = {}, 弱引用个数 = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            println!(
                "branch 强引用个数 = {}, 弱引用个数 = {}",
                Rc::strong_count(&branch),
                Rc::weak_count(&branch),
            );

            println!(
                "leaf 强引用个数 = {}, 弱引用个数 = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );
        }

        println!("leaf 父节点 = {:?}", leaf.parent.borrow().upgrade());
        println!(
            "leaf 强引用 = {}, 弱引用 = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
}