use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
  let now = SystemTime::now();
  let unix = now.duration_since(UNIX_EPOCH).unwrap();
  let diff_ms = unix.as_secs() * 1000 + unix.subsec_millis() as u64;
  println!("{}", diff_ms);
}
