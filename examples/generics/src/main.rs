// add 声明为接收两个同类型的参数，计算他们的算数和
// 类型允许是支持 Add 特性的
#[allow(unused)]
fn add<T: std::ops::Add<Output=T>>(a: T, b: T) -> T {
    a + b
}

#[allow(unused)]
fn sub<T: std::ops::Sub<Output=T>>(a: T, b: T) -> T {
    a - b
}

#[allow(unused)]
fn largest<T: PartialOrd>(list: &[T]) -> &T
{
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item
        }
    }
    largest
}

#[allow(dead_code)]
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mix<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point { x: self.x, y: other.y }
    }
}

// 对于 Point<f32, f32>，也就是使用 f32 作为 x 和 y 的类型的 Point，会有特定的方法
// 其他类型的 Point 实例不具备这里面实现的方法
#[allow(unused)]
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 这里的泛型定义出现了 const N
// 这是一个 const 泛型，一个基于值的泛型参数，在这个方法中用来替代数组的长度
#[allow(unused)]
fn display_array<T: std::fmt::Debug, const N: usize>(array: &[T; N]) {
    println!("{:?}", array);
}

fn main() {
    println!("{}", add(1, 2));
    println!("{}", add(1u64, 2u64));
    println!("{}", add(1.0f32, 2.5f32));

    println!("{}", largest(&[1, 6, 4, 2, 3, 7, 2, 3]));
    println!("{}", largest(&['y', 'm', 'c', 'a']));
    println!("{}", largest(&["hello".to_string(), "world".to_string()]));

    let point_a = Point { x: 5, y: 11.4 };
    let point_b = Point { x: "hello", y: 'w' };
    let point_mixed = point_a.mix(point_b);
    println!("{:?}", point_mixed);

    display_array(&['y', 'm', 'c', 'a']);
}
