use anyhow::{bail, Result};
use ftlog::{debug, error, info};
use ftlog::{LevelFilter::Info, LogBuilder};
use fttime_reader::SHM_KEY;
use fttime_reader::{std_now_millis, V};

pub static mut A: *mut i64 = std::ptr::null_mut();

pub fn share_memery_init() {
    let shmid = shm::shmget!(SHM_KEY, shm::ffi::Ipc::CREAT as i32 | 0o666, 16).unwrap(); //出错则直接panic

    unsafe {
        A = shm::shmat!(shmid, std::ptr::null_mut(), 0).unwrap() as *mut i64;
    }
}

/// 定时从shm读取时间戳并校验
pub fn start_monitor() {
    info!("start monitor service");
    let mut last_ping = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
    share_memery_init();
    let mut last_record = 0;
    loop {
        if let Ok(shm_now) = try_shm_now_nanos() {
            // 获取成功
            if last_record > 0 && last_record == shm_now {
                // 如果两次读出来的时间戳一致,则认为写入有问题
                let alarm_msg = format!("MILLIS_MONITOR_ALARM_{}]获取MILLIS校验失败,last={},now={},连续两次读取的时间相同",
                                         SHM_KEY, last_record, shm_now);
                error!("{}", alarm_msg);
                // 强制刷新为0
                unsafe {
                    let now = 0_i64;
                    let src = &now as *const i64;
                    std::ptr::copy_nonoverlapping(src, A.offset(1), 1);
                    std::ptr::copy_nonoverlapping(src, A, 1);
                }
            } else {
                debug!("millis check passed, shm_ns={}", shm_now);
            }
            last_record = shm_now;
        } else {
            // 获取失败
            let alarm_msg = format!("[MILLIS_MONITOR_ALARM_{}]获取MILLIS失败", SHM_KEY);
            error!("{}", alarm_msg);
        }

        let local_now = std_now_millis();
        if local_now - last_ping > 30 * 1_000 {
            last_ping = local_now;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

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

pub fn init_log() {
    let log_builder = LogBuilder::new().max_log_level(Info);
    let logger = log_builder.build().expect("logger build failed");
    logger.init().expect("set logger failed");
}

fn main() {
    init_log();
    start_monitor();
}
