use core::{
    sync::atomic::{AtomicU32, Ordering},
    time::Duration,
};

use cortex_m_rt::exception;

/// Returns the time elapsed since boot as a [`Duration`].
///
/// Backed by an [`AtomicU32`] millisecond counter that wraps after ~49.7 days.
/// Callers should use `checked_sub` when computing elapsed time to handle the wrap.
pub fn millis() -> Duration {
    Duration::from_millis(u64::from(MILLIS.load(Ordering::Relaxed)))
}
static MILLIS: AtomicU32 = AtomicU32::new(0);

#[exception]
fn SysTick() {
    MILLIS.fetch_add(1, Ordering::Relaxed);
}
