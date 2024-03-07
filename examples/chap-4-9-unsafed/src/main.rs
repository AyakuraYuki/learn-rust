fn main() {
    let mut num = 5;
    let ptr = &num as *const i32;
    // Dereferencing a raw pointer is only allowed inside an `unsafe` block or function
    // -> https://doc.rust-lang.org/error_codes/E0133.html
    unsafe {
        println!("ptr is {}", *ptr);
    }
    println!("num is {}", num);

    num = 6;
    unsafe {
        println!("ptr is {}", *ptr);
    }
    println!("num is {}", num);
}
