use std::fmt::{Display, Formatter};

// User 定义为一个公开可用的结构体
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "User(username: {}, email: {}, active: {}, sign_in_count: {})", self.username, self.email, self.active, self.sign_in_count)
    }
}

pub fn build_user(username: String, email: String) -> User {
    User {
        // 赋值时允许使用类似 javascript/typescript 的同名字段缩略写法
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    // 创建一个 User 结构体的实例
    // 不需要遵守定义字段的顺序去赋值，但一定要给所有字段初始化
    let user = User {
        active: true,
        username: String::from("user001"),
        email: String::from("user001@example.com"),
        sign_in_count: 1,
    };
    let user = dbg!(user); // 输出到 stderr
    println!("{}", user); // 输出到 stdout

    // 如果想要修改某个字段，必须将整个实例声明为可修改的
    let mut user = build_user("user001".to_string(), "user001@example.com".to_string());
    user.email = String::from("user-001@example.com");
    let user = dbg!(user);

    // 结构体更新语法
    // .. 表明但凡没有显式初始化的字段，全部取自目标实例获取
    // 这里的 ..user 必须在初始化代码块的尾部使用
    // 另外，这样的更新方式会导致 user 实例中原本会发生所有权转移的字段无法使用，在这个用例中，user.username 转移给了 user2.username，不再可以使用
    let user2 = User {
        email: String::from("user002@example.com"),
        ..user
    };
    let _user2 = dbg!(user2);
}
