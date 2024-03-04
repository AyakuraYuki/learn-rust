fn main() {}

// 下面这种嵌套递归的写法，可以用 Box 指向自身类型，完成从动态类型到固定大小类型的转变
#[allow(dead_code)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// 特征对象，在 examples/chap-2-8-2-traits/src/objects.rs:168 给出了案例，可以前往查看

fn gen_static_str() -> &'static str {
    let mut s = String::new();
    s.push_str("hello, world");
    // Box::leak 消费掉 Box 并且强制目标值从内存中泄漏
    // 这里演示了把一个 String 变成 'static 生命周期的 &str 类型
    //
    // 一个简单的场景：如果需要在【运行时】初始化一个【全局有效】，能活得跟程序一样久的值，就可以用 Box::leak 实现
    Box::leak(s.into_boxed_str())
}

#[cfg(test)]
mod test {
    use crate::gen_static_str;

    #[test]
    fn sample_of_stack_and_heap() {
        // 在栈上创建长度是 1000 的数组
        let arr = [0; 1000];
        // 将 arr 转移给 arr1，但因为 arr 分配在栈上，所以实际上深拷贝了 arr 的数据给 arr1
        let arr1 = arr;
        // 因为 arr 和 arr1 拥有属于各自的栈上数组，所以下面不会报错
        println!("arr: {:?}", arr.len());
        println!("arr1: {:?}", arr1.len());

        // 在堆上创建一个长度是 1000 的数组，用智能指针 Box 指向它
        let arr = Box::new([0; 1000]);
        // 将 arr 转移给 arr1，因为数据在堆上，这里仅拷贝了智能指针的结构体，底层在堆里的数据没有被拷贝
        // 所有权从 arr 转移给了 arr1，arr 不再拥有原来的数据
        let arr1 = arr;
        // 由于 arr 不再持有底层数据的所有权，所以不能输出 arr 的信息
        println!("arr1: {:?}", arr1.len());
        // 这就意味着，下面这行代码会报错，因为所有权变动了
        // println!("arr: {:?}", arr.len());
    }

    #[test]
    fn leak() {
        let s = gen_static_str();
        println!("{}", s);
    }
}
