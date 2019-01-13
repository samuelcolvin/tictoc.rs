
use std::env;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn run() -> Result<(), ()> {
  let toc = SystemTime::now();
  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
    eprintln!("tic argument required.");
    return Err(());
  }

  let tic_s = match args[1].parse::<f64>() {
      Ok(tic_s)  => tic_s,
      Err(_) => {
        eprintln!("Invalid float '{}'", args[1]);
        return Err(());
      },
  };

  let tic = UNIX_EPOCH + Duration::from_millis((tic_s * 1000.0) as u64);
  let diff_ = toc.duration_since(tic).expect("Time went backwards");
  let diff = diff_.as_secs() as f64 + diff_.subsec_millis() as f64 / 1000.0;
  let s: String;
  if diff < 1.0 {
    s = format!("{:0.0}ms", diff * 1000.0);
  } else if diff < 100.0 {
    s = format!("{:0.2}s", diff);
  } else {
    s = format!("{:0.0}s", diff);
  }
  println!("{:>5}", s);
  return Ok(());
}

fn main() {
    ::std::process::exit(match run() {
       Ok(_) => 0,
       Err(_) => 1
    });
}
