fn main() {
    example_basic_for_iter();
    println!();
    example_loop_structs();
    println!();
    example_loop_with_index();
    println!();
}

fn example_basic_for_iter() {
    // range 表达式在这两个 for loop 的区别是，0..5 不会走到第六个元素【5】，但 0..=5 会，因为后者指明了需要走到【5】
    for i in 0..5 {
        print!("{} ", i); // Output: "0 1 2 3 4 "
    }
    println!();
    for i in 0..=5 {
        print!("{} ", i); // Output: "0 1 2 3 4 5 "
    }
    println!();
}

fn example_loop_structs() {
    let array: [Foo; 5] = std::array::from_fn(|i| Foo { name: format!("foo_{}", i) });
    // 对结构体数组的遍历，需要使用引用，否则每个元素的所有权会被转移到 for 循环里，导致后面的代码不能继续使用 array
    for foo in &array {
        print!("{} ", foo.name); // Output: "foo_0 foo_1 foo_2 foo_3 foo_4 "
    }
    println!();
    println!("{:?}", array);
    // 对于可变借用，需要对 &mut array 迭代，并且 array 本身需要用 mut 修饰
    let mut array: [Foo; 5] = std::array::from_fn(|i| Foo { name: format!("foo_{}", i) });
    for foo in &mut array {
        foo.name = format!("new_{}", foo.name);
    }
    println!("{:?}", array);
}

fn example_loop_with_index() {
    let numbers: [i32; 10] = std::array::from_fn(|i| (i + 1) as i32);
    for (index, value) in numbers.iter().enumerate() {
        println!("第 {} 个数字是 {}", index, value);
    }
}

#[derive(Debug)]
struct Foo {
    name: String,
}
