use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn run(toc: SystemTime) -> Result<(), String> {
  let args: Vec<String> = ::std::env::args().collect();
  if args.len() != 2 {
    return Err("tic argument required.".to_string());
  }

  let tic_ms = match args[1].parse::<u64>() {
    Ok(tic_s) => tic_s,
    Err(_) => return Err(format!("Invalid float '{}'", args[1]))
  };

  let tic = UNIX_EPOCH + Duration::from_millis(tic_ms);
  let diff = match toc.duration_since(tic) {
    Ok(d) => d,
    Err(_) => return Err("tic after toc".to_string())
  };
  let s: String;
  if diff < Duration::from_secs(1) {
    s = format!("{:0.0}ms", diff.subsec_millis());
  } else if diff < Duration::from_secs(100) {
    let diff_s = diff.as_secs() as f64 + diff.subsec_millis() as f64 / 1000.0;
    s = format!("{:0.2}s", diff_s);
  } else {
    s = format!("{}s", diff.as_secs());
  }
  println!("{:>5}", s);
  return Ok(());
}

fn main() {
  let toc = SystemTime::now();
  ::std::process::exit(match run(toc) {
    Ok(_) => 0,
    Err(e) => {
      eprintln!("{}.", e);
      1
    }
  });
}
