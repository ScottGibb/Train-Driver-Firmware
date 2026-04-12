use core::{
    sync::atomic::{AtomicU32, Ordering},
    time::Duration,
};

use cortex_m_rt::exception;

pub fn millis() -> Duration {
    Duration::from_millis(u64::from(MILLIS.load(Ordering::Relaxed)))
}
static MILLIS: AtomicU32 = AtomicU32::new(0);

#[exception]
fn SysTick() {
    MILLIS.fetch_add(1, Ordering::Relaxed);
}
