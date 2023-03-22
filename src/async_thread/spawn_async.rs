/// # 为什么在spawn中不能使用std::sync::Mutex
/// [See tokio::spawn](https://tokio.rs/tokio/tutorial/spawning)

/// 请看下面的例子，可以自行编译看出错提示
// fn sample_error1(){
//     let mutex = std::sync::Mutex::<i32>::new(0);
//     tokio::spawn(async move {
//         let v = mutex.lock().expect("");
//         async{}.await;
//         println!("{}", "");
//     });
// }
/// 编译提示1
/// 9   |     tokio::spawn(async move {
/// |     ^^^^^^^^^^^^ future created by async block is not `Send`
///
/// 127 |         T: Future + Send + 'static,
/// |                     ---- required by this bound in `tokio::spawn`
/// |
/// 编译提示2
/// = help: within `impl Future`, the trait `Send` is not implemented for `std::sync::MutexGuard<'_, i32>`
/// note: future is not `Send` as this value is used across an await
/// 10  |         let v = mutex.lock().expect("");
/// |             - has type `std::sync::MutexGuard<'_, i32>` which is not `Send`
/// 11  |         async{}.await;
/// |         ^^^^^^^^^^^^^ await occurs here, with `v` maybe used later
/// 12  |         println!("{}", "");
/// 13  |     });
/// |     - `v` is later dropped here

/// 编译提示1 说tokio::spawn函数要求参数为“T: Future + Send + 'static”，而提供的参数为“async block is not `Send`”不是Send的，所以就编译就不通过了。
/// 简单说就是函数时，参数类型不对
///
/// 编译提示2 给出 help告诉开发者更多的信息，帮助解决这个问题。
/// 其中“`std::sync::MutexGuard<'_, i32>` which is not `Send`”说这个不是Send，且变量v有可能在“async{}.await”之后被使用。
/// 这个help信息告诉我们两个解决方法
/// 方法1 需要一个可以Send的“MutexGuard”
/// 方法2 不要让变量v可能在await之后被使用。
/// 编译器为我们写代码真操碎了心，“怎么错的，要怎么解决”，就差把代码给写出来了。
/// 同时我们也产生一个疑问，std::sync::Mutex就是多线程开发的，为什么它不能在这里的多线程使用？
/// 分析代码的运行过程，没有安全问题，但为什么编译器不那么认为呢？这里主要在于async{}的生命周比变量v的更长，它是异步的。但是我们还使用了“await”，它并不会异步。
/// 从这个上看是编译器检查的太严格了或者它没有把“await”一起考虑进来，do more do better/再接再厉
/// 下面是按照编译器的建议给出的解决方法

/// //方法1 需要一个可以Send的“MutexGuard”
/// //把std::sync::Mutex换成tokio::Mutex
fn sample_error1_method1() {
    let mutex = tokio::sync::Mutex::<i32>::new(0);
    tokio::spawn(async move {
        let v = mutex.lock().await;
        let _ = v;
        async {}.await;
        println!("{}", "");
    });
}

/// //方法1 不要让变量v可能在await之后被使用
fn sample_error1_method2() {
    let mutex = std::sync::Mutex::<i32>::new(0);
    tokio::spawn(async move {
        {
            let v = mutex.lock().expect("");
            let _ = &v;
        }
        async {}.await;
        println!("{}", "");
    });
}

/// tokio::sync::Mutex的实现为解决std::sync::Mutex的问题做了，更多的工作，所以在使用时，能用std::sync::Mutex解决问题，就不要换了

/// 疑问2 编译器async{}怎么知道是否为Send呢？
/// Send这个trait是自动的，block中的所有内容都可以安全的Send，那么它就会自动实现Send，否则就不实现。

/// 最后一定要看编译器的提示，这是最高效的解决方法

fn sample() {}

