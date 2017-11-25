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
pub fn bfs(mut g: graph::Graph, s: usize) -> Option<(usize, Vec<usize>)> {
  // Mark all vertices as not visited
  let mut visited: Vec<_> = (0..(g.vertices.len())).map(|_| false).collect();
  let mut q = VecDeque::new();
  let mut path: Vec<usize> = Vec::new();

  // Check if the start vertex exists
  if !g.contains_vertex(s) {
    return None;
  }
  // Insert s into queue until it's neighbor's are marked
  q.push_back(s);
  // Mark s as visited
  visited[s] = true;

  let mut total_dist = 0;
  // Choose vertex u from queue
  while let Some(v) = q.pop_front() {
    path.push(v);
    // Get the neighbors of w
    if let Some(neighbor) = g.get_neighbors(v) {
      // For each unvisited w
      for &w in neighbor {
        if visited[w] == false {
          // Mark w
          visited[w] = true;
          // Add w to the queue
          q.push_back(w);
        }
      }
    }
  }
  println!("");

  for i in 0..(path.len() - 1) {
    total_dist += g.get_distance(path[i], path[(i + 1)]);
  }

  if total_dist != 0 || !g.weighted {
    return Some((total_dist, path));
  }

  None
}
