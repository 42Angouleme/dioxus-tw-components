use std::sync::atomic::AtomicUsize;

const ID_PREFIX: &str = "dx42-";

static UNIQUE_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn use_unique_id() -> String {
    format!(
        "{}{}",
        ID_PREFIX,
        UNIQUE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    )
}
