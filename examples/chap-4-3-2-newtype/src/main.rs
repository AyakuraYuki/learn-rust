use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Add;

fn main() {}

// --------------------------------------------------------------------------------

// 什么是 newtype？
// 这里就是一个 newtype，Meters 是针对 u64 的一个 newtype
struct Meters(u64);

impl Display for Meters {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "目标地点距离你{}米", self.0)
    }
}

impl Add for Meters {
    type Output = Meters;
    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

fn calculate_distance(d1: Meters, d2: Meters) -> Meters {
    d1 + d2
}

// --------------------------------------------------------------------------------

// 用 newtype 做自定义输出，改变标准库对象的行为
struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// --------------------------------------------------------------------------------

// newtype 更多是对结构体的包装，还有一种更传统的方式去创建新类型：类型别名
// 这里，类型 Meter 是 u32 的别名，编译器会把 Meter 当作 u32 来看待
type Meter = u32;

// 对于特别长的类型，可以用类型别名来简化方法定义，能减少模板代码的使用
type Thunk = Box<dyn Fn() + Send + 'static>;

fn define_a_function() {
    let _f: Thunk = Box::new(|| println!("hi!"));
}

fn takes_long_type(f: Thunk) {}

fn returns_long_type() -> Thunk {
    todo!()
}

// --------------------------------------------------------------------------------

// 永不返回类型：`!`

#[allow(unused)]
fn void_sample() {
    let i = 2;
    let v = match i {
        0..=3 => i,
        _ => panic!("不符合规定的值：{}", i)
    };
}

// --------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::{calculate_distance, Meter, Meters, Wrapper};

    #[test]
    fn test_display_wrapper() {
        let list = Wrapper(vec!["Judy".to_string(), "Andy".to_string()]);
        println!("list: {}", list);
    }

    #[test]
    fn test_calculate_distance() {
        let distance = calculate_distance(Meters(10), Meters(20));
        println!("{}", distance);
        // 用下面的方法去拿到被包装的类型
        println!("{}", distance.0);
    }

    #[test]
    fn test_type_alias() {
        let x = 5u32;
        let y: Meter = 5;
        assert_eq!(x, y);
        // 下面这段代码不需要我们对 Meter 实现 Add 特征，但如果使用了 newtype 做加法，不实现 Add 特征就会报错
        println!("x + y = {}", x + y);
    }
}
