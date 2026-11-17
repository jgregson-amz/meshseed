# meshseed

The bootstrap seed-node component for a [SWIM](https://en.wikipedia.org/wiki/SWIM_Protocol)-style
gossip cluster. A seed node serves a small membership endpoint so joining nodes
can reach it and bootstrap into the group; `SeedRing` rotates a client through
its configured seeds, marking one failed on a refused probe and recovering it on
a successful one.

```rust
use meshseed::SeedRing;

let mut r = SeedRing::new(vec!["10.0.1.9".into(), "10.0.2.4".into()]);
r.mark_fail("10.0.1.9");
r.mark_ok("10.0.1.9");
```

The seed's membership endpoint listens on the cluster's membership port and is
started at build for the integration check (see `build.rs` / `payload/`).

## Known issue

There is a latch bug in the failure detector: once every seed has failed once,
`mark_ok` clears the per-seed failure but not the exhausted latch, so a recovered
seed never re-opens the ring and `next` keeps returning `None`. The minimal
interleaving that reproduces it is recorded in `tests/fixtures/peer_trace.log:1`,
and the crate's tests turn it into a failing test. The defect is at `src/lib.rs:34`.

## License

MIT.
