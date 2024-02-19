fn main() {
    // let _reference_to_nothing = dangle(); // 在这里接住了虚悬引用

    let _object = no_dangle(); // 在这里接住了真实对象
}

// fn dangle() -> &String { // dangle 返回了一个对 String 的引用
//     let s = String::from("chap-2-3-2-dangling string"); // 在这里创建了新的 String
//     &s // 返回 s 的引用
// } // 在这里，s 离开了作用域，所以内存被释放了，则返回一个对 s 的引用会变成一个危险操作

fn no_dangle() -> String { // no_dangle 则是一个正确的实现，它把新的 String 本身返回了出去，而不是一个引用
    let s = String::from("dangling string"); // 在这里创建了新的 String
    s
} // 在这里离开作用域，因为 s 的所有权被转移到外部的调用（move），所以这里什么都没有被销毁
