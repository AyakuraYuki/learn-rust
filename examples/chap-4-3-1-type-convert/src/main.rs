fn main() {}

#[cfg(test)]
mod test {
    #[test]
    fn error_case_comparing_with_different_types() {
        let a: i32 = 10;
        let b: u16 = 100;
        // 下面这个条件判断会因为类型不一样而报错
        // if a < b {
        //     println!("Ten is less than a hundred.");
        // }
    }

    #[test]
    fn a_common_type_convert() {
        let a = 3.1 as i8;
        let b = 100_i8 as i32;
        // 这里做了一个 ascii 字符转换成数值的操作，a 字符会转换成 97
        let c = 'a' as u8;
        println!("a: {}, b: {}, c: {}", a, b, c);
    }
}
