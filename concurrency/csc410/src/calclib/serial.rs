/*
  Author:
  Takeshi I.
*/
pub fn riemanns_sum(a: isize, b: isize, n: usize, func: fn(f64) -> f64) -> f64 {
  let mut area: f64 = 0.0;
  let delta: f64 = (b as f64 - a as f64).abs() / n as f64;

  for i in 0..n {
    let x = (a as f64) + (i as f64) * delta;
    area = area + (func(x) * delta);
  }
  return area;
}
