use std::{env, process};

use mingrep::{Param, run};

// run: cargo run -- <file_path> <query_keyword>
fn main() {
    let params = Param::from(env::args()).unwrap_or_else(|err| {
        eprintln!("[Error] {err}");
        process::exit(1);
    });

    println!("[Info] searching for [{}] in file {} ...", params.keyword(), params.file_path());

    if let Err(err) = run(params) {
        eprintln!("[Error] {err}");
        process::exit(1);
    }
}

/*
env::args() - 标准 Unicode 输入

    let args: Vec<String> = env::args().collect();


env::args_os() - 对非 Unicode 字符更兼容的取参数的方法

    let args: Vec<OsString> = env::args_os().collect();


args[0] 永远是程序的可执行文件绝对路径
 */
