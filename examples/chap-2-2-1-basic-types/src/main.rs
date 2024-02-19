use std::isize;

fn main() {
    println!("i8: {} ~ {}", i8::MIN, i8::MAX);
    println!("i16: {} ~ {}", i16::MIN, i16::MAX);
    println!("i32: {} ~ {}", i32::MIN, i32::MAX);
    println!("i64: {} ~ {}", i64::MIN, i64::MAX);
    println!("i128: {} ~ {}", i128::MIN, i128::MAX);
    println!("isize: {} ~ {}", isize::MIN, isize::MAX);
    println!();
    println!("u8: {} ~ {}", u8::MIN, u8::MAX);
    println!("u16: {} ~ {}", u16::MIN, u16::MAX);
    println!("u32: {} ~ {}", u32::MIN, u32::MAX);
    println!("u64: {} ~ {}", u64::MIN, u64::MAX);
    println!("u128: {} ~ {}", u128::MIN, u128::MAX);
    println!("usize: {} ~ {}", usize::MIN, usize::MAX);
    println!();
    // 下面这里给了基本的使用方法，还有更多使用方法可以参考格式化输出
    println!("dec: {}", 1_145_141_919_810);
    println!("hex: {}", 0xff);
    println!("oct: {}", 0o77);
    println!("bin: {}", 0b11);
    println!("char: {}", b'A');
    println!();
    println!("f32: {} ~ {}", f32::MIN, f32::MAX);
    println!("f64: {} ~ {}", f64::MIN, f64::MAX);
    println!();
    // bitwise
    // 00000010
    let a: i32 = 2;
    // 00000011
    let b: i32 = 3;
    println!("(a & b) is {}", a & b);
    println!("(a | b) is {}", a | b);
    println!("(a ^ b) is {}", a ^ b);
    println!("(!b) is {}", !b);
    println!("(a << b) is {}", a << b);
    println!("(a >> b) is {}", a >> b);
    // 除了【非（!）】运算之外都支持赋值写法，因为 != 被用来判断不等于，同时被写变量需要用 mut 声明
    let mut a = a;
    a <<= b;
    println!("(a << b) is {}", a);
}
