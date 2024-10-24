use std::future::Future;
use std::io;
use std::pin::Pin;
use std::rc::Rc;

use futures::{FutureExt, join, pin_mut, select, Stream, StreamExt, try_join, TryFutureExt, TryStreamExt};
use futures::executor::block_on;
use futures::stream::FusedStream;

async fn do_something() {
    // .await 等待另一个 async 方法执行完成
    hello_cat().await;
    println!("run in async, go go go");
}

async fn hello_cat() {
    println!("meow");
}

// --------------------------------------------------------------------------------

struct Song {
    author: String,
    name: String,
}

async fn learn_song() -> Song {
    Song {
        author: "蔡琴".to_string(),
        name: "《渡口》".to_string(),
    }
}

async fn play_song(song: Song) {
    println!("now playing: {} - {}", song.name, song.author);
}

async fn band() {
    println!("嗵，咚咚咚");
}

async fn learn_and_play() {
    // 方法内使用 await 保证顺序性，并且这里的 await 不会影响外部
    let song = learn_song().await;
    play_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_play();
    let f2 = band();
    // `join!()` 可以并发处理和等待多个 Future，在这里，f1 和 f2 任何一个被阻塞，另一个 Future 就会拿走线程所有权继续执行
    // 如果两者都被阻塞，则 async_main() 会变成阻塞，让出线程所有权
    futures::join!(f1, f2);
}

// --------------------------------------------------------------------------------

// lifecycle of async

async fn foo() -> u8 { 5 }

async fn foo_expanded_async(x: &u8) -> u8 { *x }

// foo_expanded 的声明等价于 foo_expanded_async
// 意味着 async fn 函数返回的 Future 必须满足以下条件:
//     当 x 依然有效时， 该 Future 就必须继续等待（.await）, 也就是说 x 必须比 Future 活得更久。
fn foo_expanded<'a>(x: &'a u8) -> impl Future<Output=u8> + 'a {
    async move { *x }
}

async fn borrow_x(x: &u8) -> u8 { *x }

// fn bad() -> impl Future<Output=u8> {
//     let x = 5;
//     borrow_x(&x) // ERROR: `x` does not live long enough
// }

fn good() -> impl Future<Output=u8> {
    async {
        let x = 5;
        borrow_x(&x).await
    }
}

// --------------------------------------------------------------------------------

// async move

// 多个不同的 `async` 代码块可以访问同一个本地变量，只要在被访问变量的作用域内执行即可
async fn blocks() {
    let text = "foo".to_string();

    let f1 = async {
        println!("{text}");
    };
    let f2 = async {
        println!("{text}");
    };

    futures::join!(f1, f2);
}

// 由于 `async move` 会捕获环境中的变量，因此只有一个 `async move` 语句块可以访问该变量，
// 但是它也有非常明显的好处：变量可以转移到返回的 Future 中，不再受借用生命周期的限制
fn move_block() -> impl Future<Output=()> {
    let text = "foo".to_string();
    async move {
        println!("{text}");
    }
}

// --------------------------------------------------------------------------------

// Stream 的迭代和并发

async fn sum_with_next(mut stream: Pin<&mut dyn Stream<Item=i32>>) -> i32 {
    use futures::stream::StreamExt; // 引入 next

    let mut sum = 0;
    while let Some(v) = stream.next().await {
        sum += v;
    }
    sum
}

async fn sum_with_try_next(mut stream: Pin<&mut dyn Stream<Item=Result<i32, io::Error>>>) -> Result<i32, io::Error> {
    use futures::stream::TryStreamExt; // 引入 try_next

    let mut sum = 0;
    // try_next() 是 next() 的一个可以遇到错误提前返回的版本
    while let Some(v) = stream.try_next().await? {
        sum += v;
    }
    Ok(sum)
}

// 上面的 sum_with_next 和 sum_with_try_next 并不是真正的并发，只是一次处理一个值的模式。下面展示了更【并发】的处理方式。

async fn jump_around(mut stream: Pin<&mut dyn Stream<Item=Result<u8, io::Error>>>) -> Result<(), io::Error> {
    use futures::stream::TryStreamExt; // 引入 try_for_each_concurrent
    const MAX_CONCURRENT_JUMPERS: usize = 100;

    stream.try_for_each_concurrent(MAX_CONCURRENT_JUMPERS, |num| async move {
        println!("{num}");
        // do somethings...
        Ok(())
    }).await?;

    Ok(())
}

// --------------------------------------------------------------------------------

// 【实战向】同时运行多个 Future

struct Book;

struct Music;

async fn enjoy_book() -> Book {
    Book {}
}

async fn enjoy_music() -> Music {
    Music {}
}

// 一个正确的并发
async fn enjoy_book_and_music() -> (Book, Music) {
    let book_fut = enjoy_book();
    let music_fut = enjoy_music();

    // 下面这行是错误的范例
    // (book_fut.await, music_fut.await)

    // 而 join!() 才是正确的范例
    join!(book_fut, music_fut)

    // 如果有一个数组，数组里全是异步任务，可以用 futures::future::join_all 方法同时运行
}

async fn get_book() -> Result<Book, String> { Ok(Book) }

async fn another_get_book() -> Result<Book, ()> { Ok(Book) }

async fn get_music() -> Result<Music, String> { Ok(Music) }

async fn try_get_book_and_music() -> Result<(Book, Music), String> {
    let book_fut = get_book();
    let music_fut = get_music();
    // try_join!() 会在任意 future 失败后停止所有 future 的执行
    try_join!(book_fut, music_fut)
}

async fn try_another_get_book_and_music() -> Result<(Book, Music), String> {
    // try_join!() 接受的 future 必须都拥有相同的错误类型，如果存在不同的错误类型，
    // 可以考虑使用 futures::future::TryFutureExt 提供的 map_err 和 err_info 转换
    let book_fut = another_get_book().map_err(|()| "unable to get book".to_string());
    let music_fut = get_music();
    try_join!(book_fut, music_fut)
}

// --------------------------------------------------------------------------------

// select! {}

async fn task_one() {}

async fn task_two() {}

async fn race_tasks() {
    // .fuse() 让 future 实现了 FusedFuture 特征
    let t1 = task_one().fuse();
    let t2 = task_two().fuse();
    // pin_mut!() 会为 future 实现 Unpin 特征
    pin_mut!(t1, t2);
    // select! 需要 future 实现 FusedFuture 和 Unpin
    // Unpin 是因为 select 不会通过拿走所有权的方式去使用 future，保证 future 的所有权还可以被其他代码使用，
    // 有点类似于观察者的思路；
    // FusedFuture 意味着熔断，一旦 future 完成后，select 就不能对其再次轮询使用，相当于一旦 future 完成，
    // 调用 poll 会返回 Poll::Pending。
    //
    // 下面展示了一次性 select 的写法，不论 t1 和 t2 谁先完成，都会输出 t1 分支的结果，并且结束 select。
    select! {
        () = t1 => println!("task 1 complete"),
        () = t2 => println!("task 2 complete"),
    }

    // 在测试用例 select_default_complete 展示了搭配 loop 的写法，这种写法可以监控到所有的 future 并且作出
    // 相应地分支处理。
}

async fn add_two_streams(
    mut s1: impl Stream<Item=u8> + FusedStream + Unpin,
    mut s2: impl Stream<Item=u8> + FusedStream + Unpin,
) -> u8 {
    let mut total = 0;
    loop {
        let item = select! {
            x = s1.next() => x,
            x = s2.next() => x,
            complete => break,
        };
        if let Some(next_num) = item {
            total += next_num
        }
    }
    total
}

// --------------------------------------------------------------------------------

// 一种在 async 块中使用未实现 Send 的方法

#[derive(Default)]
struct NoSend(Rc<()>);

async fn foo_with_no_send() {
    {
        // 如果把语句块去掉，就不会为 _x 触发 Drop，那么这个方法会因为 _x 影响到 .await 使得编译错误
        let _x = NoSend::default();
    }
    foo().await;
}

// --------------------------------------------------------------------------------

fn main() {
    let f = do_something();
    println!("main is busy");
    block_on(f);

    block_on(async_main());

    block_on(blocks());
    block_on(move_block());
}

#[cfg(test)]
mod test {
    use futures::{future, select};
    use futures::executor::block_on;

    use crate::{foo, race_tasks};

    #[test]
    fn async_block() {
        let f1 = async {
            let x: u8 = foo().await;
            x + 5
        };
        let f2 = async {
            println!("233333");
        };
        block_on(f1);
        block_on(f2);
    }

    #[test]
    fn test_race_tasks() {
        block_on(race_tasks());
    }

    #[test]
    fn select_default_complete() {
        let mut a_fut = future::ready(4);
        let mut b_fut = future::ready(6);
        let mut total = 0;
        loop {
            select! {
                a = a_fut => total += a,
                b = b_fut => total += b,
                complete => break, // 所有分支都完成后会执行，往往配合 loop 去循环完成所有 future 并跳出
                default => panic!(), // 没有任何 future 或者 stream 处于 ready 状态的，会执行这个分支
            }
        }
        assert_eq!(total, 10);
    }

    #[test]
    fn tell_async_ret_type() {
        async fn foo() -> Result<u8, String> {
            Ok(1)
        }

        async fn bar() -> Result<u8, String> {
            Ok(1)
        }

        let _fut = async {
            foo().await?;
            bar().await?;
            // 在 async 语句块不能显式声明返回类型，但是可以使用 ::<(), String> 来增加类型注释告诉编译器返回的类型
            Ok::<(), String>(())
        };
    }
}
