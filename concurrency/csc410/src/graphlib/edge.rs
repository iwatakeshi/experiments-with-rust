/*
  Authors:
  Takeshi I.
*/
use std::cmp::Ordering;
#[derive(Eq, Clone, Copy)]
#[allow(unused)]
pub struct Edge {
  pub v: usize,
  pub w: usize,
  pub distance: usize
}

#[allow(unused)]
impl Edge {
  pub fn new() -> Edge {
    Edge {
      v: 0,
      w: 0,
      distance: 0,
    }
  }
}


impl PartialOrd for Edge {
  fn partial_cmp(&self, other: &Edge) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Edge {
  fn cmp(&self, other: &Edge) -> Ordering {
    self.distance.cmp(&other.distance)
  }
}

impl PartialEq for Edge {
  fn eq(&self, other: &Edge) -> bool {
    self.distance == other.distance
  }
}