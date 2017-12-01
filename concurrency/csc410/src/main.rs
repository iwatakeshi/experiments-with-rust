/*
  Authors:
  Takeshi I.
  Jordan Jorgensen
*/
mod calclib;
use calclib::formula;
use calclib::parallel;
use calclib::serial;
use std::time::{Duration,SystemTime};
// Define number of parallel threads
static NTHREADS: usize = 4;

// Calculation globals
static START_POINT: isize = 0;
static END_POINT: isize = 4;
static NUM_RECTANGLES: usize = 50000000;

fn main() {

  println!("Riemann Sum LHS approximation for f(x) = x^2 using:");
  println!("Start x: {} to End x: {}, with {} rectangles ({} threads)\n", START_POINT, END_POINT, NUM_RECTANGLES, NTHREADS);

  let mut sys_time = SystemTime::now();
  println!("Parallel: {} square units", parallel::riemanns_sum(START_POINT, END_POINT, NUM_RECTANGLES, NTHREADS, formula::square));

  let mut difference: Duration = sys_time.elapsed().unwrap();
  println!("\tParallel time: {} seconds\n", difference.subsec_nanos() as f32 / 1000000000.0);
  
  sys_time = SystemTime::now();
  println!("Serial:   {} square units", serial::riemanns_sum(START_POINT, END_POINT, NUM_RECTANGLES, formula::square));

  difference = sys_time.elapsed().unwrap();
  println!("\tSerial time: {} seconds\n", difference.subsec_nanos() as f32 / 100000000.0);
}
