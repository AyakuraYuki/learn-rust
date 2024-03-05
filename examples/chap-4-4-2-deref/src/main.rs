use std::ops::{Deref, DerefMut};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// 要实现 DerefMut 特征，必须先实现 Deref 特征，因为 DerefMut 特征的声明是 `pub trait DerefMut: Deref`
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// --------------------------------------------------------------------------------

fn display(s: &str) {
    println!("{}", s);
}

fn display_mut(s: &mut String) {
    s.push_str(" (wonderful)");
    println!("{}", s);
}

// --------------------------------------------------------------------------------

fn main() {}

#[cfg(test)]
mod test {
    use crate::{display, display_mut, MyBox};

    #[test]
    fn a_sample_of_deref() {
        let x = 5;
        let y = &x;
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn deref_boxed_num_get_sum() {
        let x = Box::new(1);
        let sum = *x + 1;
        println!("{}", sum);
        let sum = *x + 2;
        println!("{}", sum);
    }

    #[test]
    fn test_my_box() {
        let x = MyBox::new(5);
        assert_eq!(5, *x);
    }

    #[test]
    fn continuing_deref() {
        let s = MyBox::new(String::from("hello, world"));
        // 连续的隐式转换，MyBox 被 Deref 成 String，结果发现不能满足 display() 的参数类型，所以继续把 String Deref 成 &str
        display(&s);
        // 假如 Rust 没有提供这种隐式转换，那就要写成下面的写法了，十分难懂
        display(&(*s)[..]);
    }

    #[test]
    fn deref_mut() {
        let mut s = MyBox::new(String::from("hello, world"));
        display_mut(&mut s);
    }
}
