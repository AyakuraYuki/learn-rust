use std::sync::atomic::{AtomicUsize, Ordering};

static GLOBAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
const MAX_ID: usize = usize::MAX / 2;

struct IDFactory {
    factory_id: usize,
}

impl IDFactory {
    fn new() -> Self {
        Self { factory_id: generate_id() }
    }
}

fn generate_id() -> usize {
    let current_value = GLOBAL_ID_COUNTER.load(Ordering::Relaxed);
    if current_value > MAX_ID {
        panic!("factory id overflow");
    }
    GLOBAL_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    let next_id = GLOBAL_ID_COUNTER.load(Ordering::Relaxed);
    if next_id > MAX_ID {
        panic!("next id overflow");
    }
    next_id
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::IDFactory;

    #[test]
    fn test_global_id_factory() {
        println!("{}", IDFactory::new().factory_id);
        println!("{}", IDFactory::new().factory_id);
        println!("{}", IDFactory::new().factory_id);
        println!("{}", IDFactory::new().factory_id);
        println!("{}", IDFactory::new().factory_id);
    }
}
