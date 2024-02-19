use std::fmt::{Debug, Display};
use std::ops::Add;

// Summary 定义了一个特征，他有一个方法 summaries()
pub trait Summary {
    // 这是一个默认实现，类似于 Java 里接口的 default 方法
    fn summaries(&self) -> String {
        String::from("(Read more...)")
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Post {
    title: String,
    author: String,
    content: String,
}

#[allow(unused)]
impl Post {
    pub fn new(title: String, author: String, content: String) -> Post {
        Post { title, author, content }
    }

    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn author(&self) -> &str {
        &self.author
    }
    pub fn content(&self) -> &str {
        &self.content
    }
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
    pub fn set_author(&mut self, author: String) {
        self.author = author;
    }
    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }
}

impl Summary for Post {
    fn summaries(&self) -> String {
        format!("\"{}\" - {}", self.title, self.author)
    }
}

// --------------------------------------------------------------------------------

#[allow(dead_code)]
#[derive(Debug)]
pub struct Weibo {
    username: String,
    content: String,
}

#[allow(unused)]
impl Weibo {
    pub fn new(username: String, content: String) -> Weibo {
        Weibo { username, content }
    }

    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn content(&self) -> &str {
        &self.content
    }
    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }
    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }
}

impl Summary for Weibo {
    fn summaries(&self) -> String {
        format!("{} 发送了微博 {}", self.username, utf8_slice::till(self.content.clone().as_str(), 40))
    }
}

// --------------------------------------------------------------------------------

#[allow(dead_code)]
#[derive(Debug)]
pub struct Tweet {
    username: String,
    content: String,
}

#[allow(unused)]
impl Tweet {
    pub fn new(username: String, content: String) -> Tweet {
        Tweet { username, content }
    }

    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn content(&self) -> &str {
        &self.content
    }
    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }
    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }
}

// 因为 Summary.summaries() 有默认实现，所以这里不实现 summaries() 也不会有问题
impl Summary for Tweet {}

// --------------------------------------------------------------------------------

// notify 接受任何实现了 Summary 特征的实例，其中的关键词是 &impl
#[allow(unused)]
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summaries());
}

// 而 &impl 真正的写法应该是基于泛型的下面这个方法
// 这也是大多数语言习惯的声明方法，接收一个泛型，并且限定这个泛型实现了某个接口，或者继承某个类
#[allow(unused)]
fn notify_post<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summaries());
}

// 对于下面这种写法，参数可以接收不同几种实现了同一个特征的实例
#[allow(unused)]
fn notify_posts(a: &impl Summary, b: &impl Summary) {}

// 而对于下面这种，几个参数都必须是相同的类型
// 两种不同的写法展示了泛型和 &impl 语法糖的区别
#[allow(unused)]
fn notify_plat_post<T: Summary>(a: &T, b: &T) {}

// 多重约束也有两种写法，下面展示的两个方法，他们的参数都需要实现 Summary 和 Debug 两个特征
#[allow(unused)]
fn notify_traits(item: &(impl Summary + Debug)) {}

#[allow(unused)]
fn notify_traits_generics<T: Summary + Debug>(item: &T) {}

// --------------------------------------------------------------------------------

// 如果约束特征变得很复杂，比如
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 { 0 }
// 那么，可以考虑用 where 约束，使得方法签名易读，则可以写成下面的方法
#[allow(unused)]
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    0
}

// --------------------------------------------------------------------------------

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// 实现运算符重载，让 Point 支持加法运算
// 只有 std::ops 里的特征可以进行重载
impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// --------------------------------------------------------------------------------

// Summary: 一个解构实现了多个特征，并且这些实现都有同名同参的方法

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("飞行中");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("起！");
    }
}

impl Human {
    fn fly(&self) {
        println!("摆摆手臂，起飞");
    }
}

// --------------------------------------------------------------------------------

// Summary: 同名关联函数的调用
// 这里引入【完全限定语法】的概念，完全限定语法的定义是：
//
//     <Type as Trait>::function(receiver_if_method, args...)
//
// 这个定义可以囊括关联函数的方法，取决于定义有没有包含 receiver_if_method
// receiver_if_method 则是三种 self，即 self、&self 和 &mut self
//
// 完全限定语法一般不会用到，只有在 rust 不能自动推导出我们的代码意图时才需要限定，典型的例子就是【方法名称重复】

trait Animal {
    fn shout() -> String;
}

struct Dog;

impl Dog {
    fn shout() -> String {
        String::from("bark")
    }
}

impl Animal for Dog {
    fn shout() -> String {
        String::from("dog is barking")
    }
}

// --------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::traits::{Animal, Dog, Human, Pilot, Point, Wizard};

    #[test]
    fn test_add_two_points() {
        let point_a = Point { x: 1, y: 0 };
        let point_b = Point { x: 3, y: -2 };
        let point_merged = point_a + point_b;
        dbg!(point_a);
        dbg!(point_b);
        dbg!(point_merged);
    }

    #[test]
    fn test_fly() {
        // 调用位于不同实现的同名方法
        let person = Human;
        person.fly();
        Pilot::fly(&person);
        Wizard::fly(&person);
    }

    #[test]
    fn test_shout() {
        println!("shout: {}", Dog::shout());
        println!("shout: {}", <Dog as Animal>::shout())
    }
}
