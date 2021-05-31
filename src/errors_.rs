// Result Option panic match

// 错误处理的过去与未来
// C语言的处理很简单，直接返回值，不同的数字表示不同的错误，操作系统也是这么做（setjmp与longjmp算是一种野路子，同理goto也是）；
// 到了C++，有了专门的异常处理，终于有一种丰常优美的错误处理机制
// 后来的Java，C# 异常处理是语言必备
// 最近的Go把异常处理去掉了，提供与C相似的处理，通过返回error处理，再加上“defer,panic,recover”
// 最近的Rust也把异常处理去掉了，也提供与C相似的处理，通过“Result,Option,match”处理，再加上“panic，catch_unwind，resume_unwind”等
// 如果要再造一门语言，也让我来选择的话，我会使用Rust的方式，原因四
// 第一，必须要处理错误分支，不然编译不过（C，Go很容易忽略）
// 第二，直接处理（没有必须增加复杂性，如异常处理机制）
// 第三，性能高（这是很多底层开发不使用C++的原因之一）
// 第四，再加一个粗暴处理（panic）
//
fn two_times_impl() -> impl Fn(i32) -> i32 {
    let i = 2;
    move |j| j * i
}

fn OptionClone() {
    let a = Some(1);
    let b = a.clone();
    let refa = Some(&1);
    let refb = refa.copied();
}

#[test]
fn test_map_and_then() {
    let mut o: Option<i64> = Some(1);
    o.and_then(|t| {
        print!("{}", t);
        let tt: Option<i32> = Some(1);
        tt
    }).and_then(|t2| {
        print!("{}", t2);
        let tt: Option<i32> = None;
        tt
    }).or_else(|| {
        let tt: Option<i32> = None;
        tt
    });
    return;
}

#[test]
fn test_match() {
    let s: String = "".to_owned();

    let i = 2;
    match i {
        _ if i < 2 => println!("i < 2"),
        // 0 .. 2 => println!("0..<2"),
        2..=3 => println!("2..=3"),
        0 | 1 => println!("0 | 1"),
        // _ => println!("default")
        _ => {}
    }

    let arr = [2, 2, 3];
    match arr {
        [1, _, _] => println!("starts with one"),
        [a, b, c] => println!("{} {} {}", a, b, c)
    };
}