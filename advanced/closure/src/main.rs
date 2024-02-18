use std::thread;
use std::time::Duration;

struct Counter<T, E>
    where T: Fn(E) -> E,
          E: Copy
{
    query: T,
    value: Option<E>,
}

impl<T, E> Counter<T, E>
    where T: Fn(E) -> E,
          E: Copy
{
    fn new(query: T) -> Counter<T, E> {
        Counter {
            query,
            value: None,
        }
    }

    fn value(&mut self, arg: E) -> E {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn workout(intensity: u32, random_number: u32) {
    // 声明闭包
    let action = || {
        println!("muuuuu...");
        thread::sleep(Duration::from_secs(2));
        intensity
    };

    if intensity < 25 {
        println!("来 {} 套俯卧撑！", action());
        println!("不够，再来 {} 套卧推", action());
    } else if random_number % 3 == 0 {
        println!("今天不宜健身");
    } else {
        println!("干拉 {} 套阔肩", action());
    }
}

fn main() {
    let intensity = 10;
    let random_number = 7;
    workout(intensity, random_number);
}

#[cfg(test)]
mod test {
    use crate::Counter;

    #[test]
    fn call_with_different_values() {
        let mut c = Counter::new(|a| a);
        let a = c.value(1);
        let b = c.value(2);
        assert_eq!(b, a);
    }
}
