fn main() {
    pointer_to_function_ptr();
}

// --------------------------------------------------------------------------------

#[allow(dead_code)]
struct Foo {
    x: u32,
    y: u16,
}

#[allow(dead_code)]
struct Bar {
    a: u32,
    b: u16,
}

#[allow(unused)]
// 这个方法演示了一种比较笨的办法去做结构体转换
// 本质上还是做了一次解构和赋值的操作，不够优雅
fn reinterpret(foo: Foo) -> Bar {
    let Foo { x, y } = foo;
    Bar { a: x, b: y }
}

// --------------------------------------------------------------------------------

// !!! unsafe !!!

// case 1: 将裸指针转换为函数指针

fn foo() -> i32 { 0 }

fn pointer_to_function_ptr() {
    let pointer = foo as *const ();
    let function = unsafe {
        std::mem::transmute::<*const (), fn() -> i32>(pointer)
    };
    assert_eq!(function(), 0);
}

// case 2: 延长生命周期

struct R<'a>(&'a i32);

unsafe fn extend_lifetime<'b>(r: R<'b>) -> R<'static> {
    // 把 'b 生命周期的 r 变成 'static
    std::mem::transmute::<R<'b>, R<'static>>(r)
}

unsafe fn shorten_invariant_lifetime<'b, 'c>(r: &'b mut R<'static>) -> &'b mut R<'c> {
    // 把 'static 生命周期的 r 变成 'c
    std::mem::transmute::<&'b mut R<'static>, &'b mut R<'c>>(r)
}

// !!! unsafe !!!

// --------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use std::num::TryFromIntError;

    #[test]
    fn error_case_comparing_with_different_types() {
        // 下面这段代码，条件判断会因为类型不一样而报错
        // let a: i32 = 10;
        // let b: u16 = 100;
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

    #[test]
    fn mem_addr_convert_to_ptr() {
        let mut values: [i32; 2] = [1, 2];
        let p1: *mut i32 = values.as_mut_ptr();
        // 将 p1 内存地址转换成整数
        let first_address = p1 as usize;
        // 第二个地址由第一个地址增加 4 个地址位获得，因为 std::mem::size_of::<i32>() == 4
        let second_address = first_address + std::mem::size_of::<i32>();
        // 访问第二个地址指向的整数
        let p2 = second_address as *mut i32;
        unsafe {
            // 2 + 1 = 3，values[1] 的结果会更新成 3
            *p2 += 1;
        }
        assert_eq!(values[1], 3);
    }

    #[test]
    fn test_try_into() {
        // error_case_comparing_with_different_types() 测试用例可以用下面的方法去解决
        let a = 10u8;
        let b = 100u16;
        // TryInto 特征声明了 try_into() 返回一个 Result，可以在类型转换上拥有比 as 更多的控制
        // 在 2021 以前的版本，需要我们手动引入 std::convert::TryInto，而 2021 版本在 std::prelude 自动引入了 TryInto 特征
        let b_u8 = b.try_into().unwrap();
        if a < b_u8 {
            println!("Ten is less than a hundred.")
        }

        let c = 1500i16;
        let c_u8: u8 = c.try_into()
            .unwrap_or_else(|e: TryFromIntError| {
                eprintln!("{:?}", e.to_string());
                0
            });
        println!("{}", c_u8);
    }
}
