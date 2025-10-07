use std::time::Duration;
/// tokio::select的执行过程
/// 1. 执行二分支，调用函数conditional_sleeper，返回None
/// 2. 不满足条件，所以不执行二分支的代码块
/// 3. 接着等待一分支的超时完成
/// 4. 一分支的超时完成，执行一分支的代码块
/// 5. 退出select
/// 注：select退出的条件是任何一个分支的条件满足或所有分支的future都运行完成
#[tokio::test]
async fn test_tokio_select() {
    env_logger::builder().is_test(false).filter_level(log::LevelFilter::Debug).init();
    let mut timer2: Option<tokio::time::Sleep> = None;
    let timer1 = tokio::time::sleep(Duration::from_millis(1000));
    tokio::select! {
        _ = timer1 => {//一分支
            log::debug!("hello world");
            timer2 = Some(tokio::time::sleep(Duration::from_millis(10)));
        },
        Some(_) = conditional_sleeper(timer2) => {//二分支
            log::debug!("goodbye cruel world");
            // break;
        }
    }
    log::debug!("after tokio::select");
}

async fn conditional_sleeper(t: Option<tokio::time::Sleep>) -> Option<()> {
    log::debug!("called conditional_sleeper");
    match t {
        Some(timer) => {
            timer.await;
            Some(())
        }
        None => None,
    }
}
