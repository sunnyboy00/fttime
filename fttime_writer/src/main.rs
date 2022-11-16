use anyhow::{bail, Result};
use ftlog::{error, info, LevelFilter::Info, LogBuilder};
use fttime_reader::SHM_KEY;
use fttime_reader::V;

/// 共享内存地址
pub static mut A: *mut i64 = std::ptr::null_mut();

/// 更快的系统时间获取，时间消耗约 1ns，平均误差约 ±0.08ms
///
/// `std::time::SystemTime`获取时间戳耗时约 45ns
pub fn try_shm_now_nanos() -> Result<i64> {
    let v = V::new();
    unsafe {
        std::ptr::copy_nonoverlapping(A, v.x, 2);
        if v.v.get_unchecked(0) == v.v.get_unchecked(1) {
            let tm = *v.v.get_unchecked(0);
            if tm > 0 {
                return Ok(tm);
            }
        }
        std::ptr::copy_nonoverlapping(A, v.x, 2);
        if v.v.get_unchecked(0) == v.v.get_unchecked(1) {
            let tm = *v.v.get_unchecked(0);
            if tm > 0 {
                return Ok(tm);
            }
        }
        std::ptr::copy_nonoverlapping(A, v.x, 2);
        if v.v.get_unchecked(0) == v.v.get_unchecked(1) {
            let tm = *v.v.get_unchecked(0);
            if tm > 0 {
                return Ok(tm);
            }
        }
        std::ptr::copy_nonoverlapping(A, v.x, 2);
        if v.v.get_unchecked(0) == v.v.get_unchecked(1) {
            let tm = *v.v.get_unchecked(0);
            if tm > 0 {
                return Ok(tm);
            }
        }
        std::ptr::copy_nonoverlapping(A, v.x, 2);
        if v.v.get_unchecked(0) == v.v.get_unchecked(1) {
            let tm = *v.v.get_unchecked(0);
            if tm > 0 {
                return Ok(tm);
            }
        }
    }
    bail!("get nans from shm error")
}

/// 初始化，定位fttime时间戳共享内存位置。
///
/// 请在主程序入口写该函数。
pub fn now_nanos_init() {
    let shmid = shm::shmget!(SHM_KEY, shm::ffi::Ipc::CREAT as i32 | 0o666, 16).unwrap(); //出错则直接panic

    unsafe {
        A = shm::shmat!(shmid, std::ptr::null_mut(), 0).unwrap() as *mut i64;
    }
}

/// 同一服务器需保证只有一个进程在调用该 loop
pub fn start_sharing_system_now_nanos() {
    info!("start sharing system time service");
    now_nanos_init();
    let local_now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as i64;
    // 启动前先读一下,如果有最新的数据就代表已经有进程在写了
    if let Ok(shm_now) = try_shm_now_nanos() {
        if (shm_now - local_now).abs() < 1_000_000 {
            let alarm_msg = format!(
                "[SYSTEM_MILLIS_SHARING_COR][SHM:0x{:08X}]启动失败,当前检测到已有程序在写入",
                SHM_KEY
            );
            error!("{}", alarm_msg);
            return;
        }
    }

    loop {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as i64;
        unsafe {
            let src = &now as *const i64;
            // 写的时候逆序写入
            std::ptr::copy_nonoverlapping(src, A.offset(1), 1);
            std::ptr::copy_nonoverlapping(src, A, 1);
        }
        std::thread::sleep(std::time::Duration::from_micros(1));
    }
}

pub fn init_log() {
    let log_builder = LogBuilder::new().max_log_level(Info);
    let logger = log_builder.build().expect("logger build failed");
    logger.init().expect("set logger failed");
}

fn main() {
    init_log();

    start_sharing_system_now_nanos();
}
