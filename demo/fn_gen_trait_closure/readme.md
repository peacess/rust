# install in linux

sudo apt install gnuplot

# result(每次支行的结果都不一样，多运行几次)

fn time:          [1.1702 ns 1.1771 ns 1.1921 ns]   
no fn time:          [1.2872 ns 1.2872 ns 1.2873 ns]   
closure time:          [1.1702 ns 1.1703 ns 1.1704 ns]   
closure no parameter time:   [1.4043 ns 1.4158 ns 1.4355 ns]   
fn pointer time:          [2.1065 ns 2.1121 ns 2.1240 ns]   
generics time:          [2.1063 ns 2.1064 ns 2.1064 ns]   
trait object time:          [2.5765 ns 2.5782 ns 2.5801 ns]

闭包普通函数或不使用函数性能差别不大（每次运行的结果会有不一样），函数指针与泛型性能差不多，最差的是trait object(大概慢1.1 ns)

## detail

Benchmarking compare: /fn: Collecting 100 samples in estimated 5.0000 s (4.3B itcompare: /fn
time:   [1.1702 ns 1.1771 ns 1.1921 ns]   
change: [-2.0516% -0.5385% +0.5533%] (p = 0.55 > 0.05)   
No change in performance detected.   
Found 11 outliers among 100 measurements (11.00%)   
2 (2.00%) low mild   
4 (4.00%) high mild   
5 (5.00%) high severe   
Benchmarking compare: /trait object: Collecting 100 samples in estimated 5.0000 compare: /trait object
time:   [2.5765 ns 2.5782 ns 2.5801 ns]   
change: [+0.0400% +0.0724% +0.1153%] (p = 0.00 < 0.05)   
Change within noise threshold.   
Found 16 outliers among 100 measurements (16.00%)   
1 (1.00%) high mild   
15 (15.00%) high severe   
Benchmarking compare: /closure: Collecting 100 samples in estimated 5.0000 s (4.compare: /closure
time:   [1.1702 ns 1.1703 ns 1.1704 ns]   
change: [-0.0003% +0.0097% +0.0207%] (p = 0.08 > 0.05)   
No change in performance detected.   
Found 13 outliers among 100 measurements (13.00%)   
2 (2.00%) low mild   
5 (5.00%) high mild   
6 (6.00%) high severe   
Benchmarking compare: /closure no parameter: Collecting 100 samples in estimatedcompare: /closure no parameter   
time:   [1.4043 ns 1.4158 ns 1.4355 ns]   
change: [-0.0365% +0.5316% +1.4854%] (p = 0.24 > 0.05)   
No change in performance detected.   
Found 10 outliers among 100 measurements (10.00%)   
1 (1.00%) low severe   
1 (1.00%) high mild   
8 (8.00%) high severe   
Benchmarking compare: /fn pointer: Collecting 100 samples in estimated 5.0000 s compare: /fn pointer
time:   [2.1065 ns 2.1121 ns 2.1240 ns]   
change: [-0.7325% -0.1506% +0.2746%] (p = 0.75 > 0.05)   
No change in performance detected.   
Found 9 outliers among 100 measurements (9.00%)   
1 (1.00%) low severe   
1 (1.00%) high mild   
7 (7.00%) high severe   
Benchmarking compare: /generics: Collecting 100 samples in estimated 5.0000 s (2compare: /generics
time:   [2.1063 ns 2.1064 ns 2.1064 ns]   
change: [-0.0571% -0.0269% -0.0054%] (p = 0.02 < 0.05)   
Change within noise threshold.   
Found 8 outliers among 100 measurements (8.00%)   
2 (2.00%) low severe   
4 (4.00%) high mild   
2 (2.00%) high severe   
Benchmarking compare: /no fn: Collecting 100 samples in estimated 5.0000 s (3.9Bcompare: /no fn
time:   [1.2872 ns 1.2872 ns 1.2873 ns]   
change: [-0.0342% -0.0179% -0.0054%] (p = 0.01 < 0.05)   
Change within noise threshold.   
Found 12 outliers among 100 measurements (12.00%)   
2 (2.00%) low severe   
3 (3.00%) high mild   
7 (7.00%) high severe   



