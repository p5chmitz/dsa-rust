#![allow(unused)]
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::_rdtsc;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CycleCount(u64);

impl CycleCount {
    #[inline(always)]
    pub fn now() -> Self {
        // 1. Miri Check: Must come first to intercept before hardware execution
        #[cfg(miri)]
        {
            // Miri does not have real CPU cycles. We can simulate a monotonic
            // counter using the standard library's Instant or an atomic counter.
            static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
            CycleCount(COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
        }

        // 2. x86_64 Hardware (Skip if under Miri)
        #[cfg(all(target_arch = "x86_64", not(miri)))]
        unsafe {
            CycleCount(std::arch::x86_64::_rdtsc())
        }

        // 3. AArch64 Hardware (Skip if under Miri)
        #[cfg(all(target_arch = "aarch64", not(miri)))]
        {
            let ticks: u64;
            unsafe {
                std::arch::asm!("mrs {}, cntvct_el0", out(reg) ticks);
            }
            CycleCount(ticks)
        }

        // 4. Catch-all fallback for unsupported architectures when not covered above
        #[cfg(not(any(miri, target_arch = "x86_64", target_arch = "aarch64")))]
        {
            // Fall back to system time context if no native assembly is written
            static START: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();
            let start = START.get_or_init(std::time::Instant::now);
            CycleCount(start.elapsed().as_nanos() as u64)
        }
    }

    #[inline(always)]
    pub fn cycles_since(&self, earlier: CycleCount) -> u64 {
        self.0.saturating_sub(earlier.0)
    }
}
