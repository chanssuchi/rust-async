use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!")
}

fn main() {
    let future = hello_world(); // 不会打印
    println!("after future");
    block_on(future);

    let song = block_on(learn_song());
    block_on(sing_song(song));
    block_on(dance());

    block_on(async_main());
}

struct Song {}

async fn learn_song() -> Song {
    Song {}
}

async fn sing_song(s: Song) {
    // ...
}

async fn dance() {
    // ...
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();
    futures::join!(f1, f2);
}
