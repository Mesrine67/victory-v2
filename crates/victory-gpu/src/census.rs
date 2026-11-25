//! Tracks `XMemVirtualAlloc` console protection flags (specifically
//! write-combining, `protect & 0x400`) and provides `#868` switches:
//! - `VICTORY_SKIP_WC_BACK=1`: skip fetching back GPU shader writes to
//!   write-combining card buffers into title RAM.
//! - `VICTORY_SKIP_WC_COMPARE=1`: also skip hashing/comparing write-combining
//!   card buffers on the GPU (implies `VICTORY_SKIP_WC_BACK=1`).

use std::collections::BTreeMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

struct Reservation {
    len: usize,
    protect: u32,
}

static RESERVATIONS: Mutex<BTreeMap<usize, Reservation>> = Mutex::new(BTreeMap::new());

pub fn title_memory_reserved(at: usize, len: usize, _kind: u32, _flags: u64, protect: u32) {
    if !skip_wc_back() || at == 0 || len == 0 {
        return;
    }
    RESERVATIONS
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .insert(at, Reservation { len, protect });
}

pub(crate) fn skip_wc_back() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| {
        let on = skip_wc_compare()
            || std::env::var_os("VICTORY_SKIP_WC_BACK").is_some_and(|v| v == "1");
        if on {
            tracing::warn!(
                "VICTORY_SKIP_WC_BACK=1: shader writes to write-combining title memory are NOT copied back"
            );
        }
        on
    })
}

pub(crate) fn skip_wc_compare() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| {
        let on = std::env::var_os("VICTORY_SKIP_WC_COMPARE").is_some_and(|v| v == "1");
        if on {
            tracing::warn!(
                "VICTORY_SKIP_WC_COMPARE=1: copies of write-combining title memory are not hashed or compared on the card"
            );
        }
        on
    })
}

pub(crate) fn is_wc(start: u64) -> bool {
    RESERVATIONS
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .range(..=start as usize)
        .next_back()
        .is_some_and(|(base, r)| *base + r.len > start as usize && r.protect & 0x400 != 0)
}

pub(crate) static UNCOMPARED: AtomicU64 = AtomicU64::new(0);
pub(crate) static SKIPPED: [AtomicU64; 2] = [const { AtomicU64::new(0) }; 2];

pub(crate) fn skip_back(start: u64, bytes: u64) -> bool {
    if !skip_wc_back() {
        return false;
    }
    let wc = is_wc(start);
    if wc {
        let n = SKIPPED[0].fetch_add(1, Ordering::Relaxed) + 1;
        let b = SKIPPED[1].fetch_add(bytes, Ordering::Relaxed) + bytes;
        if n.is_power_of_two() {
            tracing::info!(
                "VICTORY_SKIP_WC_BACK: {n} pieces ({} MB) of write-combining memory left on the card so far",
                b >> 20
            );
        }
    }
    wc
}
