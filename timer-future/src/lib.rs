use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::Duration;

/// TimerFuture 定时任务
pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

/// SharedState 共享状态
struct SharedState {
    /// 是否完成
    completed: bool,
    /// 睡眠结束后，线程可以用 waker 通知 TimerFuture 唤醒任务
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        // 检查共享状态，来确定定时器是否完成
        if shared_state.completed {
            Poll::Ready(())
        } else {
            // 设置 waker，这样新线程在睡眠结束后可以唤醒当前任务，再次 poll
            //
            // 下面的 clone 每次 poll 都会发生一次，
            // 选择每次都 clone 是因为 TimerFuture 可以在执行器的不同任务间移动，
            // 如果只 clone 一次，那么获得的 waker 可能已经被篡改并指向其他任务，导致执行器运行错误的任务。
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        // 新建线程
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            // 睡眠指定时间以实现定时功能
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            // 通知执行器可以继续 poll 对应的 Future 了
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });

        TimerFuture { shared_state }
    }
}
