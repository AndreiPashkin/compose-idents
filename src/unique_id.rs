use std::sync::atomic::AtomicU64;

static mut SEED_SEQ: AtomicU64 = AtomicU64::new(0);

/// Generates a unique ID for each invocation.
pub fn next_unique_id() -> u64 {
    #[allow(static_mut_refs)]
    unsafe {
        SEED_SEQ.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }
}
