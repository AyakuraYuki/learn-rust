#[allow(dead_code)]
#[derive(Debug)]
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    // new 方法往往是用于初始化实例，它是 Circle 的关联函数，它的第一个参数不是 self。
    // Rust 中有一个约定俗成的规则，使用 new 来作为构造器的名称，出于设计上的考虑，Rust 特地没有用 new 作为关键字。
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle { x, y, radius }
    }

    // Circle 结构体的方法，&self 表示借用当前实例（可以理解是 Python 中类的实例本身，又或者 Java 中的 this）
    // 关于 self, &self, &mut self
    //   1. self 表示实例所有权转移到方法中，这种形式很少用
    //   2. &self 表示不可变借用，按写 golang 的经验 &self 代表不带指针的 receiver
    //   3. &mut self 表示可变借用，是标准的 Java 的 this，也是 golang 中带指针的 receiver
    // 结构体方法里的逻辑也要遵守所有权规则，一些质量不好的代码，会在结构体方法里【转移】外面传来的变量的所有权，从而造成意料外的结果
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

fn main() {
    let circle = Circle::new(1f64, 1f64, 2f64);
    println!("area: {}", circle.area());

    let block = Rectangle::new(10, 10);
    println!("area: {}", block.area());
}
