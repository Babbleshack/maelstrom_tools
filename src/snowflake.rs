use std::time::{Duration, SystemTime, UNIX_EPOCH};

const EPOCH: i64 = 1612460400; // The Unix timestamp corresponding to first second of 2015
const WORKER_ID_BITS: u64 = 5;
const SEQUENCE_BITS: u64 = 12;
const MAX_WORKER_ID: u64 = (1 << WORKER_ID_BITS) - 1;
const MAX_SEQUENCE: u64 = (1 << SEQUENCE_BITS) - 1;

pub struct Snowflake {
    seed: u64,
    sequence: u64,
    last_timestamp: u64,
}

impl Snowflake {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            sequence: 0,
            last_timestamp: 0,
        }
    }
}

impl Iterator for Snowflake {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Could use a logical clock for this instead
        if ts < self.last_timestamp {
            return None;
        }

        if ts == self.last_timestamp {
            self.sequence = (self.sequence + 1) & MAX_SEQUENCE;
            if self.sequence == 0 {
                // wait for 1 second so that we dont have a collision
                std::thread::sleep(Duration::from_millis(1))
            }
        } else {
            self.sequence = 0;
        }

        self.last_timestamp = ts;

        Some(
            (ts << (WORKER_ID_BITS + SEQUENCE_BITS)) | (self.seed << SEQUENCE_BITS) | self.sequence,
        )
    }
}
