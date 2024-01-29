#[allow(dead_code)]
#[derive(Debug)]
pub enum PokerSuit {
    // 梅花
    Clubs,
    // 黑桃
    Spades,
    // 方块
    Diamonds,
    // 红心
    Hearts,
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

#[allow(dead_code)]
#[derive(Debug)]
enum Intend {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let heart = PokerSuit::Hearts;
    let diamond = PokerSuit::Diamonds;
    print_suit(heart);
    print_suit(diamond);

    let heart_a = PokerCard::Hearts('A');
    let diamond_5 = PokerCard::Diamonds('5');
    print_card(heart_a);
    print_card(diamond_5);

    let step = Intend::Move { x: 10, y: 280 };
    match step {
        Intend::Quit => { println!("intend to quit"); }
        Intend::Move { x, y } => { println!("move to point ({}, {})", x, y); }
        Intend::Write(msg) => { println!("writing message: {}", msg); }
        Intend::ChangeColor(r, g, b) => { println!("change color to rgb({}, {}, {})", r, g, b); }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    dbg!(six);
    dbg!(none);
}

fn print_suit(suit: PokerSuit) {
    println!("{:?}", suit);
}

fn print_card(card: PokerCard) {
    println!("{:?}", card);
}

fn plus_one(v: Option<i32>) -> Option<i32> {
    match v {
        None => None,
        Some(num) => Some(num + 1),
    }
}
