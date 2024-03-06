use std::cell::RefCell;
use std::rc::{Rc, Weak};

mod r#unsafe;

// 主人
struct Owner {
    name: String,
    gadgets: RefCell<Vec<Weak<Gadget>>>,
}

// 工具
struct Gadget {
    id: i32,
    owner: Rc<Owner>,
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {}

#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    use crate::{Gadget, Node, Owner};

    #[test]
    fn tools() {
        // 创建一个 owner 对象，这个 owner 也拥有多个 gadgets
        let gadget_owner: Rc<Owner> = Rc::new(
            Owner {
                name: "Gadget Man".to_string(),
                gadgets: RefCell::new(Vec::new()),
            }
        );

        // 创建 gadgets 并于 owner 关联
        let gadget1 = Rc::new(Gadget { id: 1, owner: gadget_owner.clone() });
        let gadget2 = Rc::new(Gadget { id: 2, owner: gadget_owner.clone() });

        // 为主人更新他拥有的工具
        // 因为之前使用了 Rc，现在必须要降级到 Weak，否则就会循环引用
        // Rc::downgrade 降级成 Weak
        gadget_owner.gadgets.borrow_mut().push(Rc::downgrade(&gadget1));
        gadget_owner.gadgets.borrow_mut().push(Rc::downgrade(&gadget2));

        // 遍历工具
        for gadget_opt in gadget_owner.gadgets.borrow().iter() {

            // gadget_opt 是一个 Weak<Gadget>，因为 weak 指针不保证它所引用的对象仍然存在，
            // 所以需要显式升级（upgrade()）来判断指向的对象是否存在。
            // 当然，如果 upgrade() 返回的 Option 是 None 的话，这个引用原对象就不存在了。
            let gadget = gadget_opt.upgrade().unwrap();

            println!("Gadget {} owned by {}.", gadget.id, gadget.owner.name);
        }

        // 在 main 函数的最后，gadget_owner，gadget1 和 gadget2 都被销毁。
        // 具体是，因为这几个结构体之间没有了强引用（`Rc<T>`），所以，当他们销毁的时候。
        // 首先 gadget2 和 gadget1 被销毁。
        // 然后因为 gadget_owner 的引用数量为 0，所以这个对象可以被销毁了。
        // 循环引用问题也就避免了
    }

    #[test]
    fn tree() {
        // 创建了一个【叶子】
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!("leaf strong: {}, weak: {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

        {
            // 创建一个【枝杈】
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            // 【叶子】还长在【枝杈】上
            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            println!("branch strong: {}, weak: {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
            println!("leaf strong: {}, weak: {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
        }

        // 【叶子】凋落了，离开了【枝杈】
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!("leaf strong: {}, weak: {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    }
}
