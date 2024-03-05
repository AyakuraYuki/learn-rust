struct HasDrop1;

impl Drop for HasDrop1 {
    fn drop(&mut self) {
        println!("dropping HasDrop1");
    }
}

struct HasDrop2;

impl Drop for HasDrop2 {
    fn drop(&mut self) {
        println!("dropping HasDrop2");
    }
}

struct HasTwoDrops {
    one: HasDrop1,
    two: HasDrop2,
}

impl Drop for HasTwoDrops {
    fn drop(&mut self) {
        println!("dropping HasTwoDrops");
    }
}

struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        println!("dropping Foo");
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::{Foo, HasDrop1, HasDrop2, HasTwoDrops};

    #[test]
    fn dropping() {
        // drop 顺序
        // Foo -> HasTwoDrops -> [ HasDrop1 -> HasDrop2 ]
        // 变量级别按照逆序释放
        // 结构体内按声明顺序释放
        let _x = HasTwoDrops {
            two: HasDrop2,
            one: HasDrop1,
        };
        let _foo = Foo;
        println!("running");
    }
}
