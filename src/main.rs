use concurrency::{run_channels, run_threads, run_mutexes};

fn main() {
  run_threads();
  run_channels();
  run_mutexes();
}
