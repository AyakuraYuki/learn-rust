use std::future::Future;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, sync_channel, SyncSender};
use std::task::Context;
use std::time::Duration;

use futures::future::BoxFuture;
use futures::FutureExt;
use futures::task::{ArcWake, waker_ref};

use timer_future::TimerFuture;

/// 执行器，负责从通道接受任务并执行
struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

impl Executor {
    fn run(&self) {
        // 持续接受任务
        while let Ok(task) = self.ready_queue.recv() {
            // 获取一个 Future，若它还没有完成（仍然是 Some，不是 None），则进行一次 poll 并尝试完成它
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                // 基于任务自身创建 LocalWaker
                let waker = waker_ref(&task);
                let ctx = &mut Context::from_waker(&*waker);
                // BoxFuture<'a, T> 是 Pin<alloc::boxed::Box<dyn Future<Output = T> + Send + 'a>> 的类型别名
                // 通过调用 as_mut 方法，可以将上面的类型转换成 Pin<&mut dyn Future + Send + 'static>
                if future.as_mut().poll(ctx).is_pending() {
                    // Pending 中的 Future 继续放回任务中，等待下次 poll
                    *future_slot = Some(future);
                }
            }
        }
    }
}

/// 负责创建新的 Future 然后发送到任务通道
#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    /// spawn 用于生成 Future 并放入任务通道
    fn spawn(&self, future: impl Future<Output=()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            // 把 Spawner 自己的发布者克隆给 Task
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("任务队列已满");
    }
}

/// 一个 Future 任务，它可以调度自己（把自己放到通道里），然后等待执行器 poll
struct Task {
    /// 进行中的 Future，会在未来的某个时间点被完成
    ///
    /// 按理来说`Mutex`在这里是多余的，因为我们只有一个线程来执行任务。但是由于
    /// Rust并不聪明，它无法知道`Future`只会在一个线程内被修改，并不会被跨线程修改。因此
    /// 我们需要使用`Mutex`来满足这个笨笨的编译器对线程安全的执着。
    ///
    /// 如果是生产级的执行器实现，不会使用`Mutex`，因为会带来性能上的开销，取而代之的是使用`UnsafeCell`
    future: Mutex<Option<BoxFuture<'static, ()>>>,

    /// 可以把任务自己丢到任务通道里，等待执行器 poll
    task_sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // 通过发送任务到任务管道来实现 wake，这样 wake 后，任务就可以被执行器 poll 了
        let cloned = arc_self.clone();
        arc_self.task_sender.send(cloned).expect("任务队列已满");
    }
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    // 任务通道允许的最大缓冲数（最大任务队列长度）
    // 演示使用，生产环境中不会这么做
    const MAX_QUEUE_SIZE: usize = 10000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUE_SIZE);
    (Executor { ready_queue }, Spawner { task_sender })
}

fn main() {
    // 创建一组异步任务的执行器和生成器，这两个对象是一组连通的 channel
    let (executor, spawner) = new_executor_and_spawner();
    // 生成一个任务
    spawner.spawn(async {
        println!("howdy!");
        // 创建定时器 Future 并等待它完成
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });
    // drop 掉任务，这样执行器就知道它完成了，并且不会有新的任务（通道的发送端关闭了）
    drop(spawner);
    // 运行执行器直到任务队列清空
    executor.run();
}
