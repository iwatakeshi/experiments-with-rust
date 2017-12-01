/*
  Authors:
  Jeremy I.
  Takeshi I.
  Jordan Jorgensen
*/
use std::thread;

// Parallel thread entry function; all spawned threads will execute this.
fn riemanns_sum_worker(
    tid: usize,
    a: isize,
    b: isize,
    n: usize,
    threads: usize,
    func: fn(f64) -> f64, ) -> f64 {
    let mut slice = tid;
    let mut area: f64 = 0.0;
    // variables of different types must be cast appropriately. i32 as f64
    let delta: f64 = (b as f64 - a as f64).abs() / n as f64;
    //println!("delta {:.2}", delta);
    while slice < n {
        // println!("Hi, I'm thread {} doing slice {} of {}", tid, slice, NUM_RECTANGLES);
        let height: f64 = func((slice as f64) * delta + (a as f64));
        area += (height * delta) as f64;

        slice += threads; // dynamically distribute remaining work by tid
    }
    // println!("thread {} area {:.2}", tid, area);
    return area;
    // ALL threads will return an area of atleast 0.0
}


pub fn riemanns_sum(
    a: isize,
    b: isize,
    n: usize,
    threads: usize,
    func: fn(f64) -> f64,
) -> f64 {
    // vector to hold the thread results
    let mut children = vec![];

    for i in 0..threads {
        children.push(thread::spawn(move || {
            return riemanns_sum_worker(i, a, b, n, threads, func);
        }));
    }
    // join all threads back together otherwise the parent can quit before the children are done
    let mut intermediate_sums = vec![];
    for child in children {
        // collect each child thread's retval, push into the sums vector for iteration
        let intermediate_sum = child.join().unwrap();
        intermediate_sums.push(intermediate_sum);
    }

    // no reduce primitive like OMP, but can iterate through and sum vector values
    // would have to loop/iterate vector for other functions like min/max
    return intermediate_sums.iter().sum();
}


// // Formula to calculate and return
// fn formula(x: f64) -> f64 {
//     // Verify correct formula and global var output:
//     //     https://www.intmath.com/integration/riemann-sums.php
//     // uncomment desired return formula
//     x.powf(4.0) - 2.0 * x.powf(2.0) + 2.0 // x^4 - 2x^2 + 2
//                                           //2.0 + x.sin() // 2 + sin(x)
//                                           //x.powf(2.0) // x^2
// }
