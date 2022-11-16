use fttime_reader::{fast_now_micros, fast_now_millis, fast_now_nanos, shm_time_init};

fn main() {
    // 需要在同一个系统中开启 fttime_writer

    // 使用前需要全局范围内初始化一次
    shm_time_init();
    // 获取纳秒时间戳
    let nanos = fast_now_nanos();
    // 获取微秒时间戳
    let micros = fast_now_micros();
    // 获取毫秒时间戳
    let millis = fast_now_millis();
    println!("{nanos}-{micros}-{millis}")
}
