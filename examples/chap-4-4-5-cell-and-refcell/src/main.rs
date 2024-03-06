use std::cell::{Cell, RefCell};

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

// --------------------------------------------------------------------------------

fn is_even(i: i32) -> bool {
    i % 2 == 0
}

fn retain_even(nums: &mut Vec<i32>) {
    let slice: &[Cell<i32>] = Cell::from_mut(&mut nums[..])
        .as_slice_of_cells();

    let mut i = 0;
    for num in slice.iter().filter(|num| is_even(num.get())) {
        slice[i].set(num.get());
        i += 1;
    }

    nums.truncate(i);
}

// --------------------------------------------------------------------------------

fn main() {
    let mq = MessageQueue {
        message_cache: RefCell::new(Vec::new())
    };
    mq.send("hello, world".to_string());

    retain_even(&mut vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}
