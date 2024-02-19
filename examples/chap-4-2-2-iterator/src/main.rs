use std::collections::HashMap;

// 模拟实现一个 for 循环
fn manual_loop() {
    let values = vec![1, 2, 3];

    let result = match IntoIterator::into_iter(values) {
        mut iter => loop {
            match iter.next() {
                Some(x) => { println!("{}", x); }
                None => break
            }
        },
    };

    println!();
    result
}

fn a_strange_iter() {
    let numbers = vec![1, 2, 3];
    // 迭代器自身实现了 IntoIterator 特征，所以写多少个 .into_iter() 都 OK
    for number in numbers.into_iter().into_iter().into_iter().into_iter().into_iter() {
        println!("{}", number);
    }

    println!();
}

fn mut_or_into_or_borrow_in_iter() {
    let values = vec![1, 2, 3];

    for v in values.into_iter() {
        println!("{}", v);
    }

    // 下面这行代码执行了会报错，因为 values 的所有权已经由 into_iter() 转移走了
    // println!("{:?}", values);

    let values = vec![1, 2, 3];
    let _borrow_from_value = values.iter();
    // 这里不会报错，是因为 _borrow_from_value 只是借用了 values 中的元素
    println!("{:?}", values);

    let mut values = vec![1, 2, 3];
    // 使用 iter_mut() 对 values 进行可变借用
    let mut borrow_and_mut_from_values = values.iter_mut();
    // 取出第一个元素，修改为 0
    if let Some(v) = borrow_and_mut_from_values.next() {
        // 在这个例子里，因为来自可变借用的迭代器的 next() 返回了 &mut i32，所以要用 *v 来修改值
        *v = 0;
    }
    // 这里将会输出 [0,2,3]
    println!("{:?}", values);

    println!();
}

fn stream_api_in_rust() {
    let v1 = vec![1, 2, 3];
    // Vec<_> 告诉编译器，我们声明了 v2 的类型，编译器自己推导最终类型
    let v2: Vec<_> = v1.iter()
        .map(|x| x % 2 == 0)
        .collect();
    println!("{:?}", v2);
    println!("{:?}", v1);

    let names = ["kokomi", "tadokoro"];
    let ages = [18, 24];
    let folks: HashMap<_, _> = names.into_iter()
        // 这里用到了 zip，这是一个可以将两个迭代器压缩到一起的操作
        .zip(ages.into_iter())
        .collect();
    println!("{:?}", folks);

    println!();
}

// --------------------------------------------------------------------------------

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// 这里我们给 Counter 实现了 Iterator 特征
impl Iterator for Counter {
    // 定义特征关联类型 u32
    type Item = u32;

    // 每一次迭代都会让计数器自增，并返回最新的计数。一旦计数大于 5，不再继续自增而是返回 None。
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// --------------------------------------------------------------------------------

fn main() {
    manual_loop();
    a_strange_iter();
    mut_or_into_or_borrow_in_iter();
    stream_api_in_rust();
}

#[cfg(test)]
mod test {
    use crate::Counter;

    #[test]
    fn test_counter() {
        let mut counter = Counter::new();
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn zip_two_array_with_different_length() {
        let v1 = [1, 2, 3, 4, 5];
        let v2 = [2, 3, 4, 5];
        // 不相等长度的两个迭代器，压缩后会把多余的元素剪掉
        let zipped: Vec<_> = v1.into_iter().zip(v2).collect();
        println!("{:?}", zipped);
    }

    #[test]
    fn zip_map_filter_and_sum() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        // [(1, 2), (2, 3), (3, 4), (4, 5)] (zip)
        //   -> [2, 6, 12, 20]              (map)
        //   -> [6, 12]                     (filter)
        //   -> 18                          (sum)
        assert_eq!(18, sum);
    }
}
