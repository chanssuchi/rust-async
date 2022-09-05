# Rust异步编程
## Async
async编程是一种并发编程模型。
* 允许你在少数系统线程上运行大量的并发任务。
* 通过async/await语法，看起来和同步编程差不多。
## 其他并发模型
### OS线程
* 无需改变编程模型，线程间同步困难，性能开销大。
* 线程池可以降低一些成本，但难以支撑大量IO绑定的工作。
### Event-driven编程
* 与回调函数一起使用，可能高效。
* 非线程控制流，数据流和错误传播难以追踪。
### Coroutines
* 类似线程，无需改变编程模型。
* 类型async，支持大量任务。
* 抽象掉了底层细节（这对系统编程、自定义运行时的实现很重要）。
### Actor模型
* 将所有并发计算划分为actor，消息通信易出错。
* 可以有效的实现actor模型，但许多实际问题没解决（例如控制流、重试逻辑）。
## Rust Async
* Future是惰性的
  * 只有poll时才能取得进展，被丢弃的future就无法取得进展。
* Async是零成本的
  * 使用async可以无需堆内存分配和动态调度，对性能大好且允许在受限环境使用async。
* 不提供内置运行时
  * 运行时由社区提供
* 单线程、多线程均支持
  * 但优缺点不同
## Rust Async和线程
* OS线程
  * 适用于少量任务，有内存和CPU开销，且线程生成和线程间切换非常昂贵。
  * 线程池可以降低一些成本。
  * 允许重用同步代码，代码无需大改，无需特定编程模型。
  * 有些系统支持修改线程优先级。
* Async
  * 显著降低内存和CPU开销。
  * 同等条件下，支持比线程多几个数量级的任务（少数线程支持大量任务）。
  * 可执行文件大（需要生成状态机，每个可执行文件捆绑一个异步运行时）。
### Example
```rust
fn get_two_sites() {
    // spawn two threads to do work.
    let t1 = thread::spawn(|| download("https://www.foo.com"));
    let t2 = thread::spawn(|| download("https://www.bar.com"));
    
    // wait for both threads to complete.
    t1.join().expect("t1 panicked");
    t2.join().expect("t2 panicked");
}

async fn get_two_sites_async() {
    // create two different "futures" which, when run to completion, will asynchronously download the webpages.
    let f1 = download_async("https://www.foo.com");
    let f2 = download_async("https://www.bar.com");
    
    // run both futures to completion at the same time.
    join!(f1, f2);
}
```
## 自定义并发模型
除了线程和async，还可以用其他的并发模型（如：event-driven）。
## 注意
Rust不允许在trait里面声明async函数。
## Async
* async会一段代码转化为一个实现了Future trait的状态机。
* 虽然在同步方法中调用阻塞函数会阻塞整个线程，但阻塞的Future将放弃对线程的控制，从而允许其他Future来运行。
### 异步函数语法
  * `async fn do_something() { /*...*/ }`
  * 返回的是`Future`，需要由一个执行者来运行。 
### futures::executor::block_on
  * `block_on`阻塞当前线程，直到提供的Future运行完成。
  * 其他执行者提供更复杂的行为，例如将多个Future安排到一个线程上。
## Await
* 在async fn中，可以使用`.await`来等待另一个实现Future trait的完成。
* 与`block_on`不同，`.await`不会阻塞当前线程，而是异步的等待Future完成（如果该Future目前无法取得进展，就允许其他任务运行）。
## Future trait
* Future trait是Rust Async编程的核心。
* Future是一种异步计算，它可以产生一个值。
* 实现了Future的类型表示目前可能还不用的值。
```rust
trait SimpleFuture {
  type Output;
  fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
  Ready(T),
  Pending,
}
```
* Future代表一种可以检验其是否完成的操作。
* Future可以通过调用poll函数来取得进展。
  * poll函数会驱动Future尽可能接近完成。
  * 如果Future完成了：就返回poll::Ready(result)，其中result就是结果。
  * 如果Future还无法完成：就返回poll::Pending，并当Future准备好取得更多进展时，调用一个waker的wake函数。
* 针对Future，唯一能做的就是使用poll来敲它，直到一个值掉出来。
## Wake()
* 当wake()函数被调用时：
  * 执行器将驱动Future再次调用poll函数，以便Future能取得更多的进展。
* 没有wake()函数，执行器就不知道特定的Future何时能取得进展（就不断地poll）。
* 通过wake()函数，执行器就确切的直到哪些Future已经准备好进行poll的调用。


