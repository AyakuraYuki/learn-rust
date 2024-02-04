fn main() {}

#[cfg(test)]
mod test {
    #[test]
    fn sample_usages() {
        println!("hello, world"); // 标准输出
        println!("hello, {}", "world"); // 填充字符串输出，适用于实现 std::fmt::Display 特征的类型
        println!("the number is {}, ", 1); // 填充数字输出
        println!("{:?}", (3, 4)); // 调试输出，适用于实现 std::fmt::Debug 特征的类型，还有一种写法是 {:#?}，更好的格式化
        println!("{value}", value = 5); // 填充命名占位符
        println!("{} {}", 1, 2); // 多个参数的输出
        println!("{:04}", 42); // 填充前缀 0 的输出，这个用例是在数字前面加`0`以补足 4 位字符串长度
    }

    #[test]
    fn print_and_format() {
        let s = "hello";
        println!("{}, world", s); // stdout 并且换行
        let s1 = format!("{}, world", s); // 格式化文本并返回给 s1
        print!("{}", s1); // stdout 但不换行
        print!("{}\n", "!"); // stdout 并且使用 \n 换行
        // Output:
        // hello, world
        // hello, world!
        //

        eprintln!("Error: we made an error"); // stderr
    }

    #[test]
    fn format_string_padding() {
        // 以下全部输出 "hello x    !"
        println!("hello {:5}!", "x"); // 为"x"后面填充空格，补齐宽度5
        println!("hello {:1$}!", "x", 5); // 使用参数 5 来指定宽度
        println!("hello {:1$}!{}", "x", 5); // 使用参数 5 为参数 x 指定宽度，同时在结尾输出参数 5 => hello x    !5
        println!("hello {1:0$}!", 5, "x"); // 使用 x 作为占位符输出内容，同时使用 5 作为宽度
        println!("hello {:width$}!", "x", width = 5); // 使用有名称的参数作为宽度
    }

    #[test]
    fn format_number_padding() {
        println!("hello {:5}!", 5); // hello     5!（宽度 5）
        println!("hello {:+}!", 5); // hello +5!（输出加号）
        println!("hello {:+}!", -5); // hello -5!（输出负数减号）
        println!("hello {:05}!", 5); // hello 00005!（宽度 5，使用 0 填充）
        println!("hello {:05}!", -5); // hello -0005!（宽度 5，使用 0 填充，最左边填充负数减号占 1 位）
    }

    #[test]
    fn format_align() {
        println!("hello {:<5}!", "x"); // hello x    !（左对齐）
        println!("hello {:>5}!", "x"); // hello     x!（右对齐）
        println!("hello {:^5}!", "x"); // hello   x  !（居中）
        println!("hello {:&<5}!", "x"); // hello x&&&&!（对齐并使用指定字符 & 填充）
    }

    #[test]
    fn format_scale() {
        let num = 27u32;
        println!("{:b}", num); // 二进制
        println!("{:#b}", num); // 带前缀 0b 的二进制
        println!("{:o}", num); // 八进制
        println!("{:#o}", num); // 带前缀 0o 的八进制
        println!("{}", num); // 十进制
        println!("{:x}", num); // 十六进制小写
        println!("{:#x}", num); // 带前缀 0x 的十六进制小写
        println!("{:X}", num); // 十六进制大写
        println!("{:#X}", num); // 带前缀 0x 的十六进制大写

        println!("{:#010b}", num); // 填充 0 的二进制，设宽度为 8 且在前填充 0b（0b 会占用两位所以这里需要设置 10）
    }

    #[test]
    fn format_symbol() {
        println!("hello \"{{world}}\""); // hello "{world}"（`\"` 转义为 `"`，`{{` 转义为 `{`）
    }
}
