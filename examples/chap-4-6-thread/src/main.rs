use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::thread::JoinHandle;

const N_TIMES: u64 = 10_000_000;
const N_THREADS: usize = 10;

static R: AtomicU64 = AtomicU64::new(0);

fn add_n_times(n: u64) -> JoinHandle<()> {
    thread::spawn(move || {
        for _ in 0..n {
            R.fetch_add(1, Ordering::Relaxed);
        }
    })
}

#[derive(Debug)]
struct MyBoxU8(*mut u8);

// 实现 Send 的类型可以在线程间安全的传递其所有权
unsafe impl Send for MyBoxU8 {}

#[derive(Debug)]
struct MyBoxConstU8(*const u8);

// 实现 Sync 的类型可以在线程间安全的共享（通过引用）
unsafe impl Sync for MyBoxConstU8 {}

fn main() {}

#[cfg(test)]
mod test {
    use std::ops::Sub;
    use std::sync::{Arc, Barrier, mpsc, Mutex};
    use std::sync::atomic::Ordering;
    use std::thread;
    use std::time::{Duration, Instant};

    use crate::{add_n_times, MyBoxConstU8, MyBoxU8, N_THREADS, N_TIMES, R};

    #[test]
    fn join() {
        let handle = thread::spawn(|| {
            for i in 0..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 0..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

        // 等待子线程结束
        handle.join().unwrap();
    }

    #[test]
    fn move_into_thread() {
        // move 把 v 转移到子线程里
        let v = vec![1, 2, 3];
        let handle = thread::spawn(move || {
            println!("here's a vec: {:?}", v);
        });
        handle.join().unwrap();
    }

    // Barrier（线程屏障）可以让多个线程执行到某个点后，再继续一起往后执行。
    // barrier.wait() 就是一个屏障，目的是当所有线程打印完 `before wait at {i}` 再继续执行。
    #[test]
    fn barrier() {
        let cpu_core = num_cpus::get();

        let mut handles = Vec::with_capacity(cpu_core);
        let barrier = Arc::new(Barrier::new(cpu_core));

        for i in 0..cpu_core {
            let b = barrier.clone();
            handles.push(thread::spawn(move || {
                println!("before wait at {}", i);
                b.wait();
                println!("after wait at {}", i);
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn multiple_producer_single_receiver() {
        // 创建一个【多播单收】的消息通道，返回一组发送者和接受者
        let (sender, receiver) = mpsc::channel();
        // 新建一个线程去发送消息
        thread::spawn(move || {
            // 发送数字 1
            sender.send(1).unwrap();
        });
        // 在主线程接收并打印消息
        println!("received: {}", receiver.recv().unwrap());
    }

    #[test]
    fn loop_send_loop_receive() {
        let (sender, receiver) = mpsc::channel();

        thread::spawn(move || {
            let values = vec![
                "hi".to_string(),
                "from".to_string(),
                "the".to_string(),
                "thread".to_string(),
            ];
            for v in values {
                sender.send(v).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for receive in receiver {
            println!("got: {}", receive);
        }
    }

    #[test]
    fn mutex() {
        // 创建了一个互斥锁，它持有的值是 5
        let m = Mutex::new(5);
        {
            // 锁住并更改值到 6
            let mut num = m.lock().unwrap();
            *num = 6;
            // 锁自动解除
        }
        println!("m = {:?}", m);
    }

    #[test]
    fn mutex_in_thread() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("result: {}", *counter.lock().unwrap());
    }

    #[test]
    fn atomic_add() {
        let s = Instant::now();
        let mut threads = Vec::with_capacity(N_THREADS);
        for _ in 0..N_THREADS {
            threads.push(add_n_times(N_TIMES));
        }
        for thread in threads {
            thread.join().unwrap();
        }
        assert_eq!(N_TIMES * N_THREADS as u64, R.load(Ordering::Relaxed));
        println!("{:?}", Instant::now().sub(s));
    }

    #[test]
    fn send_ptr() {
        // MyBoxU8 为裸指针实现 Send
        let p = MyBoxU8(5 as *mut u8);
        let t = thread::spawn(move || {
            println!("{:?}", p);
        });
        t.join().unwrap();
    }

    #[test]
    fn sync_ptr() {
        // MyBoxConstU8 为裸指针实现了 Sync
        let b = &MyBoxConstU8(5 as *const u8);
        let v = Arc::new(Mutex::new(b));
        let t = thread::spawn(move || {
            let _v = v.lock().unwrap();
        });
        t.join().unwrap();
    }
}
