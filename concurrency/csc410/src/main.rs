/*
  Authors:
  Takeshi I.
*/
mod calclib;
use calclib::formula;
use calclib::parallel;
use calclib::serial;
// Define number of parallel threads
static NTHREADS: usize = 4;

// Calculation globals
static START_POINT: isize = 0;
static END_POINT: isize = 4;
static NUM_RECTANGLES: usize = 15;

fn main() {

  println!("Parallel: {}", parallel::riemanns_sum(START_POINT, END_POINT, NUM_RECTANGLES, NTHREADS, formula::square));
  println!("Serial: {}", serial::riemanns_sum(START_POINT, END_POINT, NUM_RECTANGLES, formula::square));
}
