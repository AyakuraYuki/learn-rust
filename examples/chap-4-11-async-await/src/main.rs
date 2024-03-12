use std::future::Future;
use std::io;
use std::pin::Pin;

use futures::{join, Stream, StreamExt, TryStreamExt};
use futures::executor::block_on;

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
    use futures::executor::block_on;

    use crate::foo;

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
}
