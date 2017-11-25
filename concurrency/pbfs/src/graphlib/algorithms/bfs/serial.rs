/*
  Authors:
  Takeshi I.
*/
use std::option::Option;
use std::collections::VecDeque;
use super::super::super::graph;

#[allow(unused)]
/// `bfs` searches the tree using the starting vertex *s* and explores
/// the neighbor verticies first, before moving to the next level neighbors.
/// Once the search has been completed, `bfs` returns a tuple containing
/// the distance and the path taken respectively.
pub fn bfs(mut g: graph::Graph, s: u32) -> Option<(u32, Vec<u32>)> {
  // Mark all vertices as not visited
  let mut visited: Vec<_> = (0..(g.vertices.len())).map(|_| false).collect();
  let mut q = VecDeque::new();
  let mut path: Vec<u32> = Vec::new();

  // Insert s into queue until it's neighbor's are marked
  q.push_back(s);
  // Mark s as visited
  visited[s as usize] = true;

  let mut total_dist = 0;
  // Choose vertex u from queue
  while let Some(v) = q.pop_front() {
    path.push(v);
    // Get the neighbors of w
    if let Some(neighbor) = g.get_neighbors(v) {
      // For each unvisited w
      for &w in neighbor {
        if visited[w as usize] == false {
          // Mark w
          visited[w as usize] = true;
          // print!("{number:>width$}", number=w, width=3);
          q.push_back(w);
        }
      }
    }
  }
  println!("");

  for i in 0..(path.len() - 1) {
    total_dist += g.get_distance(path[i as usize], path[(i + 1) as usize]);
  }

  if total_dist != 0 || !g.weighted {
    return Some((total_dist, path));
  }

  None
}
