use std::fmt::{Debug, Display};

use crate::traits::{Post, Summary, Weibo};

// 特性的范例代码
mod traits;
// 特性对象的范例代码
mod objects;

// 一种【孤儿规则】，可以为任何结构实现自定义的特性，同时不会破坏结构体本身的封装
impl Summary for String {
    fn summaries(&self) -> String {
        format!("char count: {}, length: {}, size_of(): {}", self.chars().count(), self.len(), std::mem::size_of_val(self))
    }
}

fn main() {
    let post = Post::new("Learn Rust".to_string(), "AY".to_string(), "AY is learning rust.".to_string());
    let weibo = Weibo::new("AY".to_string(), "learning rust...".to_string());

    // 上面两个结构体都实现了 Summary 特性
    println!("{}", post.summaries());
    println!("{}", weibo.summaries());

    println!("{}", "你好，世界".to_string().summaries());
}
