use std::sync::atomic::AtomicU64;

/// Clock implementations
///

/// Lamport Clock a lamport clock implementation
pub struct LamportClock {
    counter: AtomicU64,
}

impl LamportClock {
    pub fn new() -> Self {
        0.into()
    }

    pub fn time(&self) -> u64 {
        self.counter.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub fn increment(&self) -> u64 {
        self.counter
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }

    // Fetch counter and set to `other`
    pub fn fetch_set(&self, other: u64) {
        loop {
            let cur = self.counter.load(std::sync::atomic::Ordering::SeqCst);

            if other < cur {
                return;
            }

            match self.counter.compare_exchange(
                cur,
                other + 1,
                std::sync::atomic::Ordering::SeqCst,
                std::sync::atomic::Ordering::SeqCst,
            ) {
                Ok(n) => {
                    if cur == n {
                        return;
                    }
                }
                Err(_) => continue,
            }
        }
    }
}

impl From<u64> for LamportClock {
    fn from(value: u64) -> LamportClock {
        LamportClock {
            counter: AtomicU64::new(value),
        }
    }
}

impl Default for LamportClock {
    fn default() -> Self {
        Self::new()
    }
}
