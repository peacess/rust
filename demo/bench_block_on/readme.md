# see

[block on](https://github.com/async-rs/async-task/blob/master/examples/block.rs)  
[build_your_own_block_on](https://stevenbai.top/rust/build_your_own_block_on/)
async-task有多个版：

1. https://github.com/async-rs/async-task
2. https://github.com/smol-rs/async-task

# rust async runtime 的 block on bench 测试结果

速度比较（ns / op, 平均一次运行时间）  
compare: /directly code time:   [7.3594 ns 7.3661 ns 7.3741 ns]
compare: /futures_lite  time:   [7.8521 ns 7.8634 ns 7.8760 ns]
compare: /futures_lite  time:   [7.8521 ns 7.8634 ns 7.8760 ns]
compare: /futures       time:   [9.4864 ns 9.5082 ns 9.5285 ns]
compare: /smol          time:   [23.145 ns 23.289 ns 23.414 ns]
compare: /tokio         time:   [42.840 ns 42.870 ns 42.903 ns]
compare: /extreme       time:   [257.14 ns 257.19 ns 257.25 ns]
compare: /async_std     time:   [400.58 ns 400.94 ns 401.37 ns]
其中 “directly code”是手动实现的block on

下面是原始运行结果

```
compare: /directly code time:   [7.3594 ns 7.3661 ns 7.3741 ns]
                        change: [-0.0613% +0.0334% +0.1431%] (p = 0.54 > 0.05)
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe
compare: /directly code old
                        time:   [7.0844 ns 7.1204 ns 7.1606 ns]
                        change: [-4.0969% -3.7097% -3.3420%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 21 outliers among 100 measurements (21.00%)
  14 (14.00%) low severe
  6 (6.00%) low mild
  1 (1.00%) high mild
compare: /futures_lite  time:   [7.8521 ns 7.8634 ns 7.8760 ns]
                        change: [-0.2787% -0.1348% +0.0128%] (p = 0.07 > 0.05)
                        No change in performance detected.
Found 10 outliers among 100 measurements (10.00%)
  7 (7.00%) high mild
  3 (3.00%) high severe
compare: /futures       time:   [9.4864 ns 9.5082 ns 9.5285 ns]
                        change: [+0.0764% +0.6160% +1.1878%] (p = 0.03 < 0.05)
                        Change within noise threshold.
Found 21 outliers among 100 measurements (21.00%)
  3 (3.00%) low severe
  18 (18.00%) low mild
compare: /smol          time:   [23.145 ns 23.289 ns 23.414 ns]
                        change: [-0.1086% +0.2934% +0.7484%] (p = 0.17 > 0.05)
                        No change in performance detected.
Found 21 outliers among 100 measurements (21.00%)
  1 (1.00%) high mild
  20 (20.00%) high severe
compare: /tokio         time:   [42.840 ns 42.870 ns 42.903 ns]
                        change: [+4.1114% +4.2678% +4.3931%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe
compare: /extreme       time:   [257.14 ns 257.19 ns 257.25 ns]
                        change: [+3.3788% +3.4138% +3.4465%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
compare: /async_std     time:   [400.58 ns 400.94 ns 401.37 ns]
                        change: [-11.625% -11.533% -11.441%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  7 (7.00%) high mild
  5 (5.00%) high severe

```