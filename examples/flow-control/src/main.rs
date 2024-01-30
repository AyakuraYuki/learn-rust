fn main() {
    example_basic_for_iter();
    println!();
    example_loop_structs();
    println!();
    example_loop_with_index();
    println!();
    example_match();
    println!();
    example_if_let_while_let();
    println!();
}

fn example_basic_for_iter() {
    // range 表达式在这两个 for loop 的区别是，0..5 不会走到第六个元素【5】，但 0..=5 会，因为后者指明了需要走到【5】
    for i in 0..5 {
        print!("{} ", i); // Output: "0 1 2 3 4 "
    }
    println!();
    for i in 0..=5 {
        print!("{} ", i); // Output: "0 1 2 3 4 5 "
    }
    println!();
}

fn example_loop_structs() {
    let array: [Foo; 5] = std::array::from_fn(|i| Foo { name: format!("foo_{}", i) });
    // 对结构体数组的遍历，需要使用引用，否则每个元素的所有权会被转移到 for 循环里，导致后面的代码不能继续使用 array
    for foo in &array {
        print!("{} ", foo.name); // Output: "foo_0 foo_1 foo_2 foo_3 foo_4 "
    }
    println!();
    println!("{:?}", array);
    // 对于可变借用，需要对 &mut array 迭代，并且 array 本身需要用 mut 修饰
    let mut array: [Foo; 5] = std::array::from_fn(|i| Foo { name: format!("foo_{}", i) });
    for foo in &mut array {
        foo.name = format!("new_{}", foo.name);
    }
    println!("{:?}", array);
}

fn example_loop_with_index() {
    let numbers: [i32; 10] = std::array::from_fn(|i| (i + 1) as i32);
    for (index, value) in numbers.iter().enumerate() {
        println!("第 {} 个数字是 {}", index, value);
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum Direction {
    East,
    West,
    North,
    South,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum PokerCard {
    // 梅花
    Clubs(char),
    // 黑桃
    Spades(char),
    // 方块
    Diamonds(char),
    // 红心
    Hearts(char),
}

#[derive(Debug)]
enum Message {
    Greeting { id: i32 }
}

fn example_match() {
    let direction = Direction::North;
    let step = match direction {
        // 用类似逻辑运算符【或】的写法来匹配多个模式
        Direction::East | Direction::West => { "East or West" }
        Direction::North => { "North" }
        // 下划线 `_` 形似其他语言的 `default` 关键字，表示所有模式匹配中没有匹配的部分
        _ => { "South" }
    };
    dbg!(step);

    let coin = Coin::Penny;
    let money = match coin {
        Coin::Penny => {
            // 一个模式可以做很多事，如果是需要返回结果给一个变量，最后需要用表达式结尾
            println!("lucky penny");
            1
        }
        // 一些简单的模式，可以直接返回表达式
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    };
    dbg!(money);

    let card = PokerCard::Diamonds('K');
    // 模式绑定取出绑定的值
    match card {
        PokerCard::Clubs(v) => { println!("Clubs_{}", v); }
        PokerCard::Spades(v) => { println!("Spades_{}", v); }
        PokerCard::Diamonds(v) => { println!("Diamonds_{}", v); }
        PokerCard::Hearts(v) => { println!("Hearts_{}", v); }
    }
    // 匹配没有匹配到的部分，也可以用一个变量承接其他情况
    match card {
        PokerCard::Hearts(v) => { println!("Hearts_{}", v); }
        other => { println!("other cards: {:?}", other); }
    }

    // 模式匹配可以使用 @ 绑定到一个新的变量，允许在这个分支里使用绑定的变量
    let msg = Message::Greeting { id: 5 };
    match msg {
        Message::Greeting { id: id_message @ 3..=7 } => {
            // 这里解构了 msg，并且把 id 绑定给 id_message 以供分支内使用
            println!("发现一个落在 [3, 7] 内的数值: {}", id_message);
        }
        Message::Greeting { id: 10..=12 } => {
            // 可以看到这里不能使用 id
            println!("发现一个落在 [10, 12] 内的数值");
        }
        Message::Greeting { id } => {
            // 没有条件的解构，则可以在分支里使用 id（又或者其他的名称）
            println!("发现一个落在其他地方的数值: {}", id);
        }
    }
}

fn example_if_let_while_let() {
    let v = Some(3u8);
    // 想要忽略其他的可能，只处理【3】，可以用 if let 的语法实现
    // 类似的，对于那些只关心 Some 而不处理 None 的情况，这个语法很有用
    // 下面这段代码等效于：
    // match v {
    //     Some(3) => println!("three");
    //     _ => (),
    // }
    if let Some(3) = v {
        println!("three");
    }

    // 变量遮蔽：发生在 match 或者 if let 的绑定，相当于产生了一个新的变量，这些新的变量往往不那么容易看出，所以建议使用不同的名称
    // 为了演示，这里使用了同样的变量名来解释变量遮蔽
    let age = Some(30);
    println!("匹配前的 age 是 {:?}", age);
    if let Some(age) = age {
        println!("模式绑定的 age 是 {:?}", age);
    }
    println!("匹配后的 age 是 {:?}", age);

    let version = Some("1.0.0");
    println!("匹配前的 version 是 {:?}", version);
    match version {
        Some(version) => println!("模式绑定的 version 是 {:?}", version),
        _ => ()
    }
    println!("匹配后的 version 是 {:?}", version);

    // 对于需要在 Vec 做过滤的操作，有时候不能用对象和枚举直接做【==】对比，可以用 matches! 宏来比较
    let coins = vec![Coin::Penny, Coin::Dime, Coin::Quarter];
    let mut iter = coins.iter().filter(|c| matches!(c, Coin::Dime));
    // while let 也是模式匹配的一种，是条件循环的一种实现，一旦匹配到 None 则停止循环
    while let Some(coin) = iter.next() {
        println!("filtered coin: {:?}", coin);
    }
}

#[derive(Debug)]
struct Foo {
    name: String,
}
