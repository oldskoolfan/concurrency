use std::{thread, time::Duration, vec};

pub fn run_threads() {
  // 16.1, 16.2
  let handle = thread::spawn(|| {
    for i in 1..=10 {
      println!("hi this is number {} from the spawned thread!", i);
      thread::sleep(Duration::from_millis(1));
    }
  });

  for i in 1..=5 {
    println!("hi this is number {} from the main thread!", i);
    thread::sleep(Duration::from_millis(1));
  }

  handle.join().unwrap();

  // 16.3, 16.5
  let v = vec![1, 2, 3];

  let handle = thread::spawn(move || {
    println!("Here's a vector: {:?}", v);
  });

  handle.join().unwrap();
}
