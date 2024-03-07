use std::slice::from_raw_parts;
use std::str::from_utf8_unchecked;

fn get_memory_location() -> (*const u8, usize) {
    let string = "hello, world";
    let pointer = string.as_ptr();
    let length = string.len();
    (pointer, length)
}

fn get_string_at_location(pointer: *const u8, length: usize) -> &'static str {
    unsafe {
        from_utf8_unchecked(from_raw_parts(pointer, length))
    }
}

fn main() {
    let (pointer, length) = get_memory_location();
    let message = get_string_at_location(pointer, length);
    println!("the {} bytes at {:?} stored [{}]", length, pointer, message);

    // 访问非法地址 -> Process finished with exit code 139 (interrupted by signal 11:SIGSEGV)
    // let message = get_string_at_location(127 as *const u8, 10);
    // println!("invalid access: {}", message);
}

#[cfg(test)]
mod test {
    use std::arch::asm;

    #[test]
    fn raw_pointer() {
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

    #[test]
    fn asm() {
        let x: u64;
        unsafe {
            asm!("nop");
            asm!("mov {}, 5", out(reg) x); // 把 5 赋给 x
        }
        println!("x is {}", x);
    }
}
