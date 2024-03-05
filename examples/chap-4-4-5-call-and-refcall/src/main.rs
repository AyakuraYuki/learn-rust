use std::cell::RefCell;

pub trait Messenger {
    fn send(&self, msg: String);
}

pub struct MessageQueue {
    message_cache: RefCell<Vec<String>>,
}

impl Messenger for MessageQueue {
    fn send(&self, msg: String) {
        // borrow_mut() 让 &self 中的 message_cache 变成了一个可变值
        self.message_cache.borrow_mut().push(msg)
    }
}

fn main() {
    let mq = MessageQueue {
        message_cache: RefCell::new(Vec::new())
    };
    mq.send("hello, world".to_string());
}
