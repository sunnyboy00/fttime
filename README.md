# fttime

**fttime**是非凸自研的高速时间戳中间件，高频场景中有大量获取时间戳的需求，fttime可以将单次获取时间戳的延时降低到1ns。其原理是由单独的时间戳写入进程负责将时间戳循环写入指定的共享内存，使用方需要获取当前时间戳时直接从共享内存中获取，对比使用 `std::time::SystemTime`获取时间戳快大约15~20倍（视平台情况），对比使用 `chrono`方法获取时间戳快大约60倍，适用于对于时间戳精度要求不高（测试误差在-900μs ~ +20μs），但是要求快速返回的场景。

CAUTION: this crate use `unchecked_math `unstable feature and `unsafe `code. Only use this crate in rust `nightly` channel.

### 使用

1. 安装writer

   ```bash
   // 安装时间戳写入进程二进制文件
   cd fttime_writer
   cargo install --path ./

   // 启动时间戳写入进程
   nohup fttime_writer > /xxx/log/fttime_writer.log 2>&1 & 

   ```
2. 安装monitor

   ```bash
   // 安装时间戳生成监控二进制文件
   cd fttime_monitor
   cargo install --path ./

   // 启动时间戳监控进程，获取不到时间戳时打印错误日志
   nohup fttime_monitor > /xxx/log/fttime_monitor.log 2>&1 & 

   ```
3. 在项目中引入lib依赖

   ```toml
   [dependencies]
   fttime_reader = {git = "https://github.com/nonconvextech/fttime.git", tag="v0.1.0"}
   ```
4. 读取时间戳

   ```rust
   use fttime_reader::{fast_now_nanos, shm_time_init};

   fn main(){
       // 需要在同一个系统中开启 fttime_writer

       // 使用前需要全局范围内初始化一次
       shm_time_init(); 
       let now_nanos = fast_now_nanos();
       // use now_nanos
       ...
   }
   ```

### 性能对比

_对比标准库和chrono,测试结果为10000次的平均值_

| 平台/类库                                                                                                     | fttime    | std lib    | chrono      |
| ------------------------------------------------------------------------------------------------------------- | --------- | ---------- | ----------- |
| OS：macOS Monterey 12.6<br />cpu：Apple M1 Pro, 3.2 GHz<br />mem：32G<br />Rust：1.67.0-nightly               | 1 ns/iter | 22 ns/iter | 62 ns/iter  |
| OS：Ubuntu Server 22.04 LTS<br />cpu：AMD EPYC 7T83 64-Core, 2.45GHz<br />mem：1.8T<br />Rust：1.67.0-nightly | 2 ns/iter | 35 ns/iter | 131 ns/iter |
