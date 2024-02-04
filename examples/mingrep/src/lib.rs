use std::{env, fs};
use std::env::Args;
use std::error::Error;

#[derive(Debug)]
pub struct Param {
    file_path: String,
    keyword: String,
    ignore_case: bool,
}

impl Param {
    pub fn from(mut args: Args) -> Result<Param, &'static str> {
        if args.len() < 3 {
            // 使用 Result 来代替 panic 可以对使用者隐去调试信息
            return Err("require keyword and file path, usage: mingrep <file_path> <keyword>");
        }

        // 跳过可执行文件路径
        args.next();

        let file_path = match args.next() {
            Some(argv) => argv,
            None => return Err("require file path"),
        };

        let keyword = match args.next() {
            Some(argv) => argv,
            None => return Err("require keyword for searching"),
        };

        // 使用环境变量
        // Usage: IGNORE_CASE=1 mingrep <file_path> <keyword>
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Param {
            file_path,
            keyword,
            ignore_case,
        })
    }

    pub fn file_path(&self) -> &str {
        &self.file_path
    }
    pub fn keyword(&self) -> &str {
        &self.keyword
    }
    pub fn ignore_case(&self) -> bool {
        self.ignore_case
    }
}

pub fn run(params: Param) -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string(params.file_path())?;

    let result = if params.ignore_case {
        search_case_insensitive(params.keyword(), &text)
    } else {
        search(params.keyword(), &text)
    };

    for line in result {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(keyword: &str, text: &'a str) -> Vec<&'a str> {
    text
        // 按行迭代
        .lines()
        // 找出包含关键词的行
        .filter(|line| line.contains(keyword))
        // 收集起来
        .collect()
}

pub fn search_case_insensitive<'a>(keyword: &str, text: &'a str) -> Vec<&'a str> {
    let query = keyword.to_lowercase();
    text
        // 按行迭代
        .lines()
        // 忽略大小写的情况下，找出包含关键词的行
        .filter(|line| line.to_lowercase().contains(&query))
        // 收集起来
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let keyword = "duct";
        let text = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(keyword, text));
    }

    #[test]
    fn case_sensitive() {
        let keyword = "duct";
        let text = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";
        assert_eq!(vec!["safe, fast, productive."], search(keyword, text));
    }

    #[test]
    fn case_insensitive() {
        let keyword = "rUsT";
        let text = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(keyword, text));
    }
}
