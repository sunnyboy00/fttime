#![feature(test)]
extern crate test;
use chrono::offset::Local;
use fttime_reader::{fast_now_nanos, shm_time_init, std_now_nanos};
use test::Bencher;

#[bench]
fn bench_fast_nanos(b: &mut Bencher) {
    // 初始化共享内存
    // 需要开启时间戳写入进程
    shm_time_init();
    b.iter(|| fast_now_nanos());
}

#[bench]
fn bench_std_nanos(b: &mut Bencher) {
    b.iter(|| std_now_nanos())
}

#[bench]
fn bench_chrono_nanos(b: &mut Bencher) {
    b.iter(|| chrono_nanos())
}

fn chrono_nanos() -> i64 {
    Local::now().timestamp_nanos()
}
