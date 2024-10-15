
极至的单机性能总结：  
extreme single pc performance:  
1, 把线程绑定的cpu物理核心（6核12线程，使用超线程的，一定要关闭。超线程让一个物理核心变为2个逻辑核心或2线程，性能肯定降低了），以减少操作系统调度cpu核心的影响  
1, bind the cpu physical core to the threads(don't use the hyper threading, it must be turned off.)  
2,  清楚异步调用本身所使用的时间。如在rust中tokio::swpan本身的运行时间，在我的机器上  
（后面的时间都是我本机的测试时间，不同机器使用时间是不一样的）是1000ns(1000纳秒=1微秒)，  
而channel send一次100ns，而改用Mutex自己实现channel大概是20ns（注：20ns是在没有冲突时的时间，有冲突时间不确定），  
而改用free lock方式实现时间是5ns(注：如果有冲突时间并不确定)  
所以线程之间交互使用free lock可以达到极至的性能，一般的异步或并发库正常情况下是无法做到  
2, be sure about the time. for example(all test in my pc), tokio::swpan takes 1000ns, Mutex takes 20ns, free lock takes 5ns.  
3, 小心使用Map，在数据集的数量不超过1万时，如果需要极至的性能，就不要使用map。这时sorted vector的性能是远大于map(测试代码使用的是二分查找插入)  
3, when the count is less than 10000, use the sorted vector to instead of map  
4, log 代码中的日志也会使用不少时间，尽量使用条件编译，让不输出的日志level不编译到执行程序中  
4, log will take a lot of time. use conditional compilation to remove it from the execution program.  
5, 提炼并简化业务  
6, simple business  
