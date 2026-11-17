//! Bootstrap seed ring: rotates through the configured seed nodes, marking a seed
//! failed on a refused probe and recovering it on a successful one.
//!
//! There is a known latch bug in [`SeedRing::mark_ok`]; the interleaving that
//! triggers it is recorded in `tests/fixtures/peer_trace.log:1` and exercised by
//! the crate's tests. The defect is at `src/lib.rs:34`.

use std::collections::HashSet;

/// A ring of bootstrap seeds with a failure latch.
pub struct SeedRing {
    seeds: Vec<String>,
    failed: HashSet<String>,
    exhausted: bool,
}

impl SeedRing {
    pub fn new(seeds: Vec<String>) -> Self {
        Self {
            seeds,
            failed: HashSet::new(),
            exhausted: false,
        }
    }

    pub fn mark_fail(&mut self, seed: &str) {
        self.failed.insert(seed.to_string());
        if self.failed.len() >= self.seeds.len() {
            self.exhausted = true;
        }
    }

    /// Recover a seed after a successful probe.
    pub fn mark_ok(&mut self, seed: &str) {
        // Latch bug: clears the per-seed failure but not the `exhausted` latch, so
        // once every seed has failed once, a later recovery never re-opens the ring
        // and `next` keeps returning None.
        // Recorded interleaving: tests/fixtures/peer_trace.log:1
        self.failed.remove(seed);
    }

    pub fn next(&self) -> Option<&String> {
        if self.exhausted {
            return None;
        }
        self.seeds.iter().find(|s| !self.failed.contains(*s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recovered_seed_reopens_the_ring() {
        // Minimal interleaving recorded in tests/fixtures/peer_trace.log:1
        let mut r = SeedRing::new(vec!["a".into(), "b".into()]);
        r.mark_fail("a");
        r.mark_fail("b"); // the ring latches exhausted here
        r.mark_ok("a"); // a recovers
        assert_eq!(r.next(), Some(&"a".to_string()));
    }
}
