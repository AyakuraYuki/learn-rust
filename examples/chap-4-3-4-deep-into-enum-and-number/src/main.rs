#[repr(i32)]
#[derive(Debug)]
enum MyEnum {
    A = 1,
    B,
    C,
}

// Rust 1.34 后可以用 TryFrom 特征来做枚举到数值的转换
// 下面实现了从 i32 转换到 MyEnum，接下来就可以使用 TryInto 转换
impl TryFrom<i32> for MyEnum {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            x if x == MyEnum::A as i32 => Ok(MyEnum::A),
            x if x == MyEnum::B as i32 => Ok(MyEnum::B),
            x if x == MyEnum::C as i32 => Ok(MyEnum::C),
            _ => Err(()),
        }
    }
}

// 鉴于需要手动对每个枚举成员都实现一个转换分支会非常麻烦和易错，可以使用宏来简化
// 简单说明，宏定义由 `(模板代码) => {转换实现}` 的格式来定义
// 目前出现的关键词有：
// * meta  注解
// * vis   可见性
// * ident 类型名称
// * expr  表达式
// 为模板代码定义变量的格式是 `$val_name:keyword`，就比如 `$name:ident` 把模板代码的类名定义成模板变量 `$name`
#[macro_export]
macro_rules! back_to_enum {
    ($(#[$meta:meta])* $visit:vis enum $name:ident {
        $($(#[$vm:meta])* $vn:ident $(= $v:expr)?,)*
    }) => {
        $(#[$meta])*
        $visit enum $name {
            $($(#[$vm])* $vn $(= $v)?,)*
        }

        impl std::convert::TryFrom<i32> for $name {
            type Error = ();

            fn try_from(value: i32) -> Result<Self, Self::Error> {
                match value {
                    $(x if x == $name::$vn as i32 => Ok($name::$vn),)*
                    _ => Err(()),
                }
            }
        }
    };
}

back_to_enum! {
    #[repr(i32)]
    #[derive(Debug)]
    enum ABCEnum {
        A = 1,
        B,
        C,
    }
}

fn main() {
    // 这里把枚举转换成数值
    let x = MyEnum::C as i32;
    // try_into() 会使用对象相应的 try_from() 来转换成枚举
    match x.try_into() {
        Ok(MyEnum::A) => println!("A"),
        Ok(MyEnum::B) => println!("B"),
        Ok(MyEnum::C) => println!("C"),
        Err(_) => eprintln!("unknown")
    }

    let x = ABCEnum::B as i32;
    match x.try_into() {
        Ok(ABCEnum::A) => println!("A"),
        Ok(ABCEnum::B) => println!("B"),
        Ok(ABCEnum::C) => println!("C"),
        Err(_) => eprintln!("unknown")
    }
}
