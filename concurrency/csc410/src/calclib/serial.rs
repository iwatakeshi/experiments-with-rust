/*
  Author:
  Takeshi I.
*/
pub fn riemanns_sum(a: isize, b: isize, n: usize, func: fn(f32) -> f32) -> f32 {
  let mut area: f32 = 0.0;
  let delta: f32 = (b as f32 - a as f32).abs() / n as f32;
  for i in 0..n {
    let x = (a as f32) + (i as f32) * delta;
    area += func(x) * delta;
  }
  return area;
}
