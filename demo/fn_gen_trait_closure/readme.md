# install in linux
sudo apt install gnuplot

# result
compare: /closure no parameter time:   [1.1702 ns 1.1841 ns 1.2150 ns]  
compare: /closure              time:   [1.4045 ns 1.4059 ns 1.4088 ns]  
compare: /fn                   time:   [1.4062 ns 1.4124 ns 1.4219 ns]  
compare: /fn pointer           time:   [1.8722 ns 1.8723 ns 1.8723 ns]  
compare: /generics             time:   [1.8723 ns 1.8724 ns 1.8725 ns]  
compare: /trait object         time:   [2.5744 ns 2.5745 ns 2.5746 ns]  

闭包捕获参数方式性能最好， 闭包与普通函数性能差不多，函数指针与泛型性能差不多，最差的是trait object(大概慢1.1 ns)  

## detail 
compare: /fn            
time:   [1.4062 ns 1.4124 ns 1.4219 ns]  
change: [+2.1311% +4.7990% +7.3816%] (p = 0.00 < 0.05)  
Performance has regressed.  
Found 18 outliers among 100 measurements (18.00%)  
18 (18.00%) high severe  
compare: /trait object    
time:   [2.5744 ns 2.5745 ns 2.5746 ns]  
change: [+0.0022% +0.0066% +0.0109%] (p = 0.00 < 0.05)  
Change within noise threshold.  
Found 9 outliers among 100 measurements (9.00%)  
1 (1.00%) low mild  
7 (7.00%) high mild  
1 (1.00%) high severe  
compare: /closure         
time:   [1.4045 ns 1.4059 ns 1.4088 ns]  
change: [-2.4447% -0.9237% +0.1647%] (p = 0.21 > 0.05)  
No change in performance detected.  
Found 9 outliers among 100 measurements (9.00%)  
4 (4.00%) high mild  
5 (5.00%) high severe  
compare: /closure no parameter  
time:   [1.1702 ns 1.1841 ns 1.2150 ns]  
change: [-2.8932% -0.8251% +1.1436%] (p = 0.49 > 0.05)  
No change in performance detected.  
Found 14 outliers among 100 measurements (14.00%)  
2 (2.00%) low mild  
5 (5.00%) high mild  
7 (7.00%) high severe  
compare: /fn pointer     
time:   [1.8722 ns 1.8723 ns 1.8723 ns]  
change: [-1.6413% -0.9864% -0.6243%] (p = 0.00 < 0.05)  
Change within noise threshold.  
Found 14 outliers among 100 measurements (14.00%)  
1 (1.00%) low mild  
6 (6.00%) high mild  
7 (7.00%) high severe  
compare: /generics        
time:   [1.8723 ns 1.8724 ns 1.8725 ns]  
change: [-0.6252% -0.5165% -0.4272%] (p = 0.00 < 0.05)  
Change within noise threshold.  
Found 11 outliers among 100 measurements (11.00%)  
2 (2.00%) low mild  
8 (8.00%) high mild  
1 (1.00%) high severe  

