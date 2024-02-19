fn main() {
    let mut s = String::from("hello, world");
    takes_ownership(s.clone()); // s will be borrowed if pass s into function without `.clone`
    change(&mut s);
    println!("after change(): {}", s);

    println!();

    let u = "hello, world";
    takes_ownership_str(u);
    println!("u in main: {}", u);

    println!();

    let mut v = "hello";
    println!("v in main (before mut): {}", v);
    takes_ownership_str(v);
    v = "hello, world";
    println!("v in main: {}", v);

    println!();

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
}

fn takes_ownership(some_string: String) {
    println!("takes_ownership: {}, length: {}", some_string, length(&some_string));
}

fn takes_ownership_str(some_string: &str) {
    println!("takes_ownership_str: {}", some_string)
}

fn length(s: &String) -> usize { // s 是对外部 String 的引用，不拥有所有权
    s.len()
} // 离开作用域，因为不拥有所有权，所以不会释放什么资源

fn change(s: &mut String) { // s 是对外部 String 的可变引用，不拥有所有权但可以更改外部 String 的值
    s.push_str(" (changed)")
} // 离开作用域，因为不拥有所有权，所以不会释放什么资源
