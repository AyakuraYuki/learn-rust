fn main() {
    example_slice_string();

    println!();

    example_chinese();

    println!();

    example_convert();

    println!();

    example_push_str();

    println!();

    example_replace();

    println!();

    example_delete();

    println!();

    example_concatenate();
}

fn example_slice_string() {
    let s = String::from("hello, world");
    let hello = &s[..5]; // [0, 5) // 0..5 和 ..5 是等效语句
    let world = &s[7..12]; // [7, 12) // 7..12 和 7.. 在这里是等效语句，因为 s.len() == 12

    println!("length: {}", s.len());
    println!("{}, {}", hello, world);

    let copied_str = &s[..]; // 取得完整的 String 切片

    println!("[..]: {}", copied_str);
}

fn example_chinese() {
    let s = String::from("简体中文");

    println!("length: {}, chars: {}", s.len(), s.chars().count()); // 每个汉字占三个字节

    for c in s.chars() { // chars 遍历 utf8 字符
        println!("{}", c);
    }

    // 非 ASCII 截取需要用 utf8_slice
    let s_chinese_str = utf8_slice::slice(&s[..], 0, 2);

    println!("{}", s_chinese_str);
}

fn example_convert() {
    let s = String::from("hello, world");
    say_hello(&s); // 隐式转换
    say_hello(&s[..]); // 取得完整切片
    say_hello(s.as_str()); // 方法转换
}

fn example_push_str() {
    let mut s = String::from("hello, ");
    println!("before: {}", s);
    s.push_str("rust"); // 追加字符串
    println!("push strings: {}", s);
    s.push('!'); // 追加字符
    println!("then push character: {}", s);
}

fn example_replace() {
    let s = String::from("I am learning rust. It is funny in learning rust.");
    let new_s = s.replace("funny", "hard"); // 因为替换操作不会更改原 String，所以变量 s 不需要使用 mut 关键字修饰
    dbg!(new_s);

    let s = "I am learning rust. It is funny in learning rust.";
    let new_s = s.replacen("rust", "RUST", 1);
    dbg!(new_s);

    let mut s = String::from("Have fun in rust");
    #[allow(unused)]
    {
        let ret = s.replace_range(12..13, "R"); // 因为 replace_range 是对原 String 操作，这个方法只返回单元对象，而不是字符串
    }
    dbg!(s);
}

fn example_delete() {
    // pop 把尾部的字符弹出去（字符操作）
    let mut s = String::from("rust with 中文！");
    dbg!(s.pop());
    dbg!(s.pop());
    dbg!(s);

    // remove 删除给定字节位的字符（字节操作，不是字符操作）
    let mut s = String::from("测试 remove 方法");
    println!("s 占 {} 个字节", std::mem::size_of_val(s.as_str()));
    s.remove(0); // 删除第一个汉字
    // s.remove(1); // 尝试删除第二个汉字？错，这里的 1 代表第二个字节位置！
    s.remove(3); // 删除第二个汉字？错，因为第一个汉字不见了，字符串整体往前移动了 3 个字节位置，这里会删除【试】后面的空格
    dbg!(s);

    // truncate 删除给定字节位往后全部的字符（字节操作，不是字符操作）
    let mut s = String::from("测试 truncate");
    // s.truncate(2); // 危险！truncate 是按照字节处理的，错误的索引位置会产生边界异常
    s.truncate(7);
    dbg!(s);

    // clear 清空字符串，可以看到因为 String 内部是使用 Vec<u8> 封装，可以理解成清空了 Vec
    let mut s = String::from("测试 clear");
    s.clear();
    dbg!(s);
}

fn example_concatenate() {
    let a = String::from("hello, ");
    let b = String::from("world");
    // 这里相当于调用了 std::string 的 add()，这个方法实现给 String，第二个参数是 &str，所以 + 后面的部分要传 &b
    // + 返回的结果是新的字符串，所以原变量不需要用 mut 修饰
    let res = a + &b;
    // dbg! 会归还所有权，但因为 res 没有使用 mut 修饰，需要另外接收
    let res = dbg!(res);
    // 因为 a 的所有权已经被转移，所以不能继续使用，要想继续使用 a，需要在拼接时使用 a.clone() 克隆一份 a
    // dbg!(a);
    let mut res = res.clone();
    // 要想使用 +=，需要保证变量本身是可变的，因为 += 是对原变量的修改
    res += "!!!";
    dbg!(res);

    // format! 宏，利用模板语句构造字符串，因为不会转移原值，所以原值和新字符串都可以继续使用
    let s1 = "hello";
    let s2 = String::from("world");
    let s3 = format!("{}, {}", s1, s2);
    dbg!(s3);
    dbg!(s1);
}

fn say_hello(s: &str) {
    println!("{}", s);
}
