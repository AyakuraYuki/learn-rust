fn main() {
    let a = "hello";
    let b = "rust";
    let longest = longest(a, b);
    println!("{:?}", longest);
}

// 这个函数像是声明泛型一样声明了生命周期参数<'a>
// 我们无从得知函数参数的生命周期是否就是<'a>，但这个<'a>不代表一个具体的生命周期
// 这个生命周期表示函数参数 x 和 y 的生命周期至少活得跟<'a>一样久，并且是大于等于<'a>的生命周期
//
// 通过函数签名来指定生命周期参数，我们并没有因此改变传入引用或者返回引用的真实生命周期，
// 仅仅只是告诉编译器，当不满足次约束条件时，就拒绝编译通过。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[cfg(test)]
mod test {
    use crate::longest;

    // 下面这个案例解释了为什么 longest 函数声明的 <'a> 是最小生命周期
    // outside_string 的生命周期比 inside_string 更长，很明显这两个变量拥有不同的生命周期
    // 那么对于 longest 函数来说，<'a> 的生命周期等同于 inside_string 的生命周期
    // 所以返回的 result 的生命周期等同于 inside_string 的
    //
    // 不信？把 println!() 往外放一下试试，会收到下面的编译失败的信息：
    // error[E0597]: `inside_string` does not live long enough
    //   --> examples/lifecycle/src/main.rs:37:55
    //    |
    // 36 |             let inside_string = String::from("inside the inner code block");
    //    |                 ------------- binding `inside_string` declared here
    // 37 |             result = longest(outside_string.as_str(), inside_string.as_str());
    //    |                                                       ^^^^^^^^^^^^^ borrowed value does not live long enough
    // 38 |         }
    //    |         - `inside_string` dropped here while still borrowed
    // 39 |         println!("the longest string: {}", result);
    //    |                                            ------ borrow later used here
    #[test]
    fn lifecycle_demo() {
        let outside_string = String::from("a string outside the sub code block");
        let mut result = "";
        {
            let inside_string = String::from("inside the inner code block");
            result = longest(outside_string.as_str(), inside_string.as_str());
            println!("the longest string: {}", result);
        }
    }
}
