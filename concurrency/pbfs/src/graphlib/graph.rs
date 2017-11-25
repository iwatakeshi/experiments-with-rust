/*
  Authors:
  Takeshi I.
*/

extern crate rand;

use self::rand::{thread_rng, Rng};
use std::option::Option;
use super::edge;

/// `Graph` is a simple graph data structure using an adjacency list and an adjacency matrix.
///
/// Here's an example of generating a random graph with undirected weighted edges:
///
/// ```
/// mod graphlib;
/// use graphlib::graph;
/// use graphlib::algorithms;
///
/// let mut g = graph::Graph::new(4);
/// g.add_directed_edge(0, 1);
/// g.add_directed_edge(0, 2);
/// g.add_directed_edge(1, 2);
/// g.add_directed_edge(2, 0);
/// g.add_directed_edge(2, 3);
/// g.add_directed_edge(3, 3);
/// g.print();
///
/// match algorithms::bfs::serial::bfs(g, 3) {
///   Some(result) => {
///     let (distance, path) = result;
///     println!("Distance: {}, Path: {:?}", distance, path);
///   }
///   None => println!("No path was found."),
/// }



pub struct Graph {
  pub size: u32,
  pub edges: Vec<edge::Edge>,
  pub vertices: Vec<u32>,
  pub adj_list: Vec<Vec<u32>>,
  pub adj_matrix: Vec<Vec<u32>>,
  pub weighted: bool,
}

#[allow(unused)]
impl Graph {
  pub fn new(size: u32) -> Graph {
    Graph {
      size: size,
      edges: {
        let e = Vec::new();
        e
      },
      vertices: {
        let v = (0..size).collect();
        v
      },
      adj_list: {
        let mut vec = vec![vec![]; size as usize];
        vec
      },
      adj_matrix: {
        let mut vec = vec![vec![0; size as usize]; size as usize];
        vec
      },
      weighted: false,
    }
  }

  pub fn add_edge(&mut self, v: u32, w: u32) {
    if !self.contains_edge(v, w) && v < self.size && w < self.size {
      self.edges.push(edge::Edge {
        v: v,
        w: w,
        distance: 0,
      });
      self.adj_matrix[v as usize][w as usize] = 1;
      self.adj_matrix[w as usize][v as usize] = 1;
      if !self.contains_vertex(v) {
        self.vertices.push(v);
      }
      if !self.contains_neighbor(v, w) {
        self.adj_list[v as usize].push(w);
      }
      if !self.contains_vertex(w) {
        self.vertices.push(w);
      }
      if !self.contains_neighbor(w, v) {
        self.adj_list[w as usize].push(v);
      }
    }
  }

  pub fn add_weighted_edge(&mut self, v: u32, w: u32, distance: u32) {
    if !self.contains_edge(v, w) && v < self.size && w < self.size {
      self.weighted = true;
      self.edges.push(edge::Edge {
        v: v,
        w: w,
        distance: distance,
      });
      self.adj_matrix[v as usize][w as usize] = distance;
      self.adj_matrix[w as usize][v as usize] = distance;
      if !self.contains_vertex(v) {
        self.vertices.push(v);
      }
      if !self.contains_neighbor(v, w) {
        self.adj_list[v as usize].push(w);
      }
      if !self.contains_vertex(w) {
        self.vertices.push(w);
      }
      if !self.contains_neighbor(w, v) {
        self.adj_list[w as usize].push(v);
      }
    }
  }

  pub fn add_directed_edge(&mut self, v: u32, w: u32) {
    if !self.contains_edge(v, w) && v < self.size && w < self.size {
      self.edges.push(edge::Edge {
        v: v,
        w: w,
        distance: 0,
      });
      self.adj_matrix[v as usize][w as usize] = 1;
      if !self.contains_vertex(v) {
        self.vertices.push(v);
      }
      if !self.contains_neighbor(v, w) {
        self.adj_list[v as usize].push(w);
      }
    }
  }

  pub fn add_directed_weighted_edge(&mut self, v: u32, w: u32, distance: u32) {
    if !self.contains_edge(v, w) && v < self.size && w < self.size {
      self.edges.push(edge::Edge {
        v: v,
        w: w,
        distance: 0,
      });
      self.adj_matrix[v as usize][w as usize] = distance;
      if !self.contains_vertex(v) {
        self.vertices.push(v);
      }
      if !self.contains_neighbor(v, w) {
        self.adj_list[v as usize].push(w);
      }
    }
  }

  pub fn get_edge(&self, v: u32, w: u32) -> Option<&edge::Edge> {
    return self
      .edges
      .iter()
      .find(|ref edge| edge.v == v && edge.w == w);
  }

  pub fn get_neighbors(&self, v: u32) -> Option<&Vec<u32>> {
    if self.adj_list[v as usize].len() > 0 {
      return Some(&(self.adj_list)[v as usize]);
    }
    None
  }

  pub fn get_distance(&self, v: u32, w: u32) -> u32 {
    if let Some(edge) = self.get_edge(v, w) {
      return edge.distance;
    }
    return 0;
  }

  pub fn contains_vertex(&self, v: u32) -> bool {
    return !self.vertices.is_empty() && self.vertices.contains(&v);
  }

  pub fn contains_edge(&self, v: u32, w: u32) -> bool {
    return !self.vertices.is_empty()
      && self.edges.iter().any(|ref edge| edge.v == v && edge.w == w);
  }

  pub fn contains_neighbor(&self, v: u32, w: u32) -> bool {
    return self.adj_list[v as usize].contains(&w);
  }

  pub fn randomize_weights(&mut self, low: u32, high: u32) {
    let mut rng = thread_rng();
    for i in 0..self.size {
      for j in 0..self.size {
        let n: u32 = rng.gen_range(low, high);
        if i != j {
          self.add_weighted_edge(i, j, n);
        }
      }
    }
  }

  pub fn print(&self) {
    for i in &self.adj_matrix {
      for j in i.iter() {
        print!("{number:>width$}", number = j, width = 3);
      }
      println!("");
    }
    println!("");
  }

  pub fn print_neighbors(&self, v: u32) {
    match self.get_neighbors(v) {
      Some(neighbors) => {
        for i in neighbors {
          print!("{number:>width$}", number = i, width = 3);
        }
        println!("");
      }
      None => println!("No neighbors found."),
    }
  }
}
