use std::{fs, io};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

// 对 main 函数动手，rust 还支持另一种 main 函数的声明
// 这种声明比较复杂，但可以支持使用 `?` 提前返回
fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}

// 这个方法演示了一种传播错误的案例
fn file_readline<P: AsRef<Path>>(path: P) -> Result<String, io::Error> {
    let f = File::open(path);
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut string = String::new();
    match f.read_to_string(&mut string) {
        Ok(_) => Ok(string),
        Err(e) => Err(e),
    }
}

// 在这个实现，文件操作的结尾都有一个问号`?`，问号在这里表示一个宏，它代表了下面的语法：
// let mut f = match f {
//     Ok(file) => file,
//     Err(e) => return Err(e),
// }
// 但是，`?` 具备类型提升的特点。错误之间存在上下级关系，std::error::Error 是标准错误特征，std::io::Error 是一个与 IO 相关的具体的错误实现
// 使用 `?` 可以把 std::io::Error 提升到 std::error::Error，使得最后的方法签名可以从
// -> Result<String, io::Error>
// 变成
// -> Result<String, Box<dyn std::error::Error>>
fn file_readline_another_implement<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn std::error::Error>> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// 掌握了上面说的特征，可以用链式调用来完成读取文件的行为
fn file_readline_inline<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn std::error::Error>> {
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}

// rust 标准库封装了一个函数，不过这个实现跟现在学的内容没太大关联，并且可以看到 -> Result<String, io::Error> 后部的 Error 声明又回到了 io::Error
pub fn file_readline_std_fs<P: AsRef<Path>>(path: P) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::{ErrorKind, Read};

    use homedir::get_my_home;

    // 演示了一个简单的严重错误案例
    // v 是一个拥有 3 个数字的数组，尝试访问数组界限以外的索引，爆出 index out of bounds 错误
    // 这是一种被动触发的 panic，虽然代码里为了示范主动填写了界限外的索引，但对于 release 程序，索引可能是任意用户输入，这是一种被动的场景
    //
    // 这个案例是一个典型的【缓冲区溢出】案例，在 C 语言如果尝试读取界限外的索引，能够拿到数组外的，可能是其他变量或者其他程序的内存数据
    #[test]
    fn simple_index_out_of_bounds_panic() {
        let v = vec![1, 2, 3];
        v[999];

        // 要想规避 index out of bounds，应该考虑对比索引后再访问，或者使用 v.get(index) -> Option<?> 来判断 Some(x) 或者 None
    }

    // 演示了一个主动发出 panic 的案例
    // 主动调用 panic!() 宏，可以自主抛出 panic 错误
    #[test]
    fn make_a_panic() {
        panic!("crash and burn")
    }

    #[test]
    fn recoverable_error__read_file() {
        let home_dir = get_my_home().unwrap().unwrap();
        let f = File::open(home_dir.as_path().join("Desktop").join("tasks.json"));
        let mut f = match f {
            Ok(v) => v,
            Err(e) => panic!("error opening file: {:?}", e)
        };
        let mut buf = String::from("");
        match f.read_to_string(&mut buf) {
            Ok(_) => println!("{}", buf),
            Err(_) => ()
        }
    }

    #[test]
    fn better_read_file_or_create() {
        let home_dir = get_my_home().unwrap().unwrap();
        let file_path = home_dir.as_path().join("Desktop").join("hello.txt");
        let f = File::open(&file_path);
        let _f = match f {
            Ok(v) => v,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => match File::create(&file_path) {
                    Ok(vv) => vv,
                    Err(ee) => panic!("{:?}", ee)
                },
                other => panic!("{:?}", other)
            }
        };
    }
}
