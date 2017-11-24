use std::thread;
use std::time;

fn main() {
  let t = time::Duration::from_millis(3);

  thread::spawn(|| {
    for i in 1..10 {
      println!("hi number {} from the spawned thread!", i);
      //thread::sleep(time::Duration::from_millis(5000));
    }
  });

  for i in 1..20 {
    println!("hi number {} from the main thread!", i);
    thread::sleep(t);
  }
}
