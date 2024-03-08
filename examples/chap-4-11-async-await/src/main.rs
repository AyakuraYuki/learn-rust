use std::future::Future;

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

fn main() {
    let f = do_something();
    println!("main is busy");
    block_on(f);

    block_on(async_main());
}
