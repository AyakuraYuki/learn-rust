fn main() {
    // 声明了一个固定长的，有线性顺序的月份名称的数组
    // 一个完整的数组声明，其类型部分应该是这样的格式：[<type>; <length>]
    // 但是，因为存在类型匹配，长度也是可以知道的，所以一般可以省略类型声明
    let months: [&str; 12] = [
        "January", "February", "Match", "April", "May", "June",
        "July", "August", "September", "October", "November", "December"
    ];

    // 一个快捷的创建方法，重复某个元素 n 次，可以用下面的语句创建
    // 下面示范了创建一个包含 10 个 0 的数组，数组的类型是 [i32; 10]
    let _numbers = [0; 10];

    // 下面是一个访问数组的示例
    // 我们以 months 为例，注意 months 的元素类型是 &str，直接拿取不会产生所有权转移
    let jan = months[0];
    let feb = months[1];
    dbg!(jan, feb);

    // 对于一个结构体，不能像 _numbers 那样写来创建结构体数组，考虑使用 std::array::from_fn 走函数返回来批量创建
    let array: [Foo; 5] = std::array::from_fn(|i| Foo { name: format!("foo_{}", i) });
    println!("{:?}", array);
}

#[allow(dead_code)]
#[derive(Debug)]
struct Foo {
    name: String,
}
