# see

[block on](https://github.com/async-rs/async-task/blob/master/examples/block.rs)  
[build_your_own_block_on](https://stevenbai.top/rust/build_your_own_block_on/)
async-task有多个版：

1. https://github.com/async-rs/async-task
2. https://github.com/smol-rs/async-task

# rust async runtime 的 block on bench 测试结果

速度比较（ns / op, 平均一次运行时间）  
tokio time:   [118.44 ns 118.45 ns 118.46 ns]  
smol time:   [117.41 ns 117.72 ns 118.06 ns]  
futures time:   [29.487 ns 29.488 ns 29.488 ns]  
async_std time:   [825.67 ns 826.50 ns 827.29 ns]  
futures_lite time:   [22.708 ns 22.712 ns 22.717 ns]  
directly code time:   [22.012 ns 22.019 ns 22.027 ns]  
extreme time:   [229.79 ns 230.06 ns 230.34 ns]

其中 “directly code”是手动实现的block on

下面是原始运行结果

```
Benchmarking tokio
Benchmarking tokio: Warming up for 3.0000 s
Benchmarking tokio: Collecting 100 samples in estimated 5.0004 s (42M iterations)
Benchmarking tokio: Analyzing
tokio                   time:   [118.44 ns 118.45 ns 118.46 ns]
                        change: [-0.0092% +0.0118% +0.0275%] (p = 0.23 > 0.05)
                        No change in performance detected.
Found 14 outliers among 100 measurements (14.00%)
  1 (1.00%) low mild
  10 (10.00%) high mild
  3 (3.00%) high severe

Benchmarking smol
Benchmarking smol: Warming up for 3.0000 s
Benchmarking smol: Collecting 100 samples in estimated 5.0003 s (43M iterations)
Benchmarking smol: Analyzing
smol                    time:   [117.41 ns 117.72 ns 118.06 ns]
                        change: [+2.4784% +2.7855% +3.1111%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

Benchmarking futures
Benchmarking futures: Warming up for 3.0000 s
Benchmarking futures: Collecting 100 samples in estimated 5.0001 s (170M iterations)
Benchmarking futures: Analyzing
futures                 time:   [29.487 ns 29.488 ns 29.488 ns]
                        change: [-96.857% -96.856% -96.854%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) low severe
  4 (4.00%) high mild
  2 (2.00%) high severe

Benchmarking async_std
Benchmarking async_std: Warming up for 3.0000 s
Benchmarking async_std: Collecting 100 samples in estimated 5.0010 s (6.0M iterations)
Benchmarking async_std: Analyzing
async_std               time:   [825.67 ns 826.50 ns 827.29 ns]
                        change: [-80.519% -80.494% -80.471%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) low mild

Benchmarking futures_lite
Benchmarking futures_lite: Warming up for 3.0000 s
Benchmarking futures_lite: Collecting 100 samples in estimated 5.0000 s (220M iterations)
Benchmarking futures_lite: Analyzing
futures_lite            time:   [22.708 ns 22.712 ns 22.717 ns]
                        change: [-96.753% -96.751% -96.750%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) high mild
  6 (6.00%) high severe

Benchmarking directly code
Benchmarking directly code: Warming up for 3.0000 s
Benchmarking directly code: Collecting 100 samples in estimated 5.0001 s (227M iterations)
Benchmarking directly code: Analyzing
directly code           time:   [22.012 ns 22.019 ns 22.027 ns]
                        change: [-96.745% -96.744% -96.743%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

Benchmarking extreme
Benchmarking extreme: Warming up for 3.0000 s
Benchmarking extreme: Collecting 100 samples in estimated 5.0001 s (22M iterations)
Benchmarking extreme: Analyzing
extreme                 time:   [229.79 ns 230.06 ns 230.34 ns]
                        change: [-96.678% -96.618% -96.568%] (p = 0.00 < 0.05)
```