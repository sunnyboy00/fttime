/// 共享内存地址
pub static SHM_KEY: i32 = 0x00_CE_00_01;
pub static mut A: *mut i64 = std::ptr::null_mut();

pub struct V {
    pub v: [i64; 2],
    pub x: *mut i64,
}

impl V {
    pub fn new() -> Self {
        let mut res = Self {
            v: [0, 1],
            x: std::ptr::null_mut(),
        };
        res.x = res.v.as_mut_ptr();
        res
    }
}

/// 更快的系统时间获取，时间消耗约 1~2ns，误差在 -900μs ~ +20μs。
///
/// `std::time::SystemTime`获取时间戳耗时约 45ns。
///
/// 通过共享内存读取时间戳，该时间戳每100μs写一次，主要误差来源于此。
/// * 若尚未初始化，则直接返回系统时间，适用于单元测试等场景；
pub fn fast_now_nanos() -> i64 {
    unsafe {
        // 未初始化时A，则直接返回系统时间。
        if A.is_null() {
            return std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as i64;
        }
    }
    let v = V::new();
    unsafe {
        std::ptr::copy_nonoverlapping(A, v.x, 2);
        if v.v.get_unchecked(0) == v.v.get_unchecked(1) {
            let tm = *v.v.get_unchecked(0);
            if tm > 0 {
                return tm;
            }
        }
        std::ptr::copy_nonoverlapping(A, v.x, 2);
        if v.v.get_unchecked(0) == v.v.get_unchecked(1) {
            let tm = *v.v.get_unchecked(0);
            if tm > 0 {
                return tm;
            }
        }
        std::ptr::copy_nonoverlapping(A, v.x, 2);
        if v.v.get_unchecked(0) == v.v.get_unchecked(1) {
            let tm = *v.v.get_unchecked(0);
            if tm > 0 {
                return tm;
            }
        }
        std::ptr::copy_nonoverlapping(A, v.x, 2);
        if v.v.get_unchecked(0) == v.v.get_unchecked(1) {
            let tm = *v.v.get_unchecked(0);
            if tm > 0 {
                return tm;
            }
        }
        std::ptr::copy_nonoverlapping(A, v.x, 2);
        if v.v.get_unchecked(0) == v.v.get_unchecked(1) {
            let tm = *v.v.get_unchecked(0);
            if tm > 0 {
                return tm;
            }
        }
    }
    return std_now_nanos();
}

/// 当前毫秒,等于fast_now_nanos() / 1_000_000
pub fn fast_now_millis() -> i64 {
    fast_now_nanos() / 1_000_000
}

/// 当前微秒,等于fast_now_nanos() / 1_000
pub fn fast_now_micros() -> i64 {
    fast_now_nanos() / 1_000
}

pub fn std_now_nanos() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as i64
}

pub fn std_now_millis() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

/// 初始化，定位fttime时间戳共享内存位置。
/// 请在主程序入口写该函数。
pub fn shm_time_init() {
    let shmid = shm::shmget!(SHM_KEY, shm::ffi::Ipc::CREAT as i32 | 0o666, 16).unwrap(); // 出错则直接panic
    unsafe {
        A = shm::shmat!(shmid, std::ptr::null_mut(), 0).unwrap() as *mut i64;
    }
    let shm_now_nanos = fast_now_nanos(); // 先读一次当前shm的时间戳
    let local_now_nanos = std_now_nanos(); // 本地获取一次
                                           // 如果差别超过1ms,则认为shm中的时间戳有问题,把shm中的数据覆盖为0
    if shm_now_nanos > 0 && local_now_nanos - shm_now_nanos > 1_000_000 {
        unsafe {
            let now = 0_i64;
            let src = &now as *const i64;
            // 写的时候逆序写入
            std::ptr::copy_nonoverlapping(src, A.offset(1), 1);
            std::ptr::copy_nonoverlapping(src, A, 1);
        }
    }
}
