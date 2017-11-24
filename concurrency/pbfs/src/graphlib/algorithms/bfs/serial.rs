use std::option::Option;
use std::collections::VecDeque;
use super::super::super::graph;

#[derive(Debug)]
enum Color {
  White,
  Gray,
  Black,
}

#[allow(unused)]
pub fn bfs(ref mut g: graph::Graph, s: u32) -> Option<(u32, Vec<u32>)> {
  // An vector of colors
  let mut color: Vec<_> = (0..g.vertices.len()).map(|_| Color::White).collect();
  let mut dist: Vec<_> = (0..g.vertices.len()).map(|_| u32::max_value()).collect();
  let mut pred: Vec<Option<u32>> = (0..g.vertices.len()).map(|_| Option::None).collect();
  let mut q: VecDeque<u32> = VecDeque::new();
  let mut path: Vec<u32> = Vec::new();

  dist[s as usize] = 0;
  color[s as usize] = Color::Gray;
  pred[s as usize] = Option::None;
  q.push_back(s);

  let mut total_dist: u32 = 0;
  while let Some(u) = q.pop_front() {
    path.push(u);
    total_dist += dist[u as usize];

    match g.get_neighbors(u) {
      Some(neighbors) => for &v in neighbors {
        match color[v as usize] {
          Color::White => {
            color[v as usize] = Color::Gray;
            dist[v as usize] = dist[u as usize] + 1;
            pred[v as usize] = Some(u);
            q.push_back(v);
          }
          _ => {}
        }
      },
      None => break,
    }
    color[u as usize] = Color::Black;
  }

  if total_dist != 0 {
    return Some((total_dist, path));
  }

  None
}
