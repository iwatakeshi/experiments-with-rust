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
  pub size: usize,
  pub edges: Vec<edge::Edge>,
  pub vertices: Vec<usize>,
  pub adj_list: Vec<Vec<usize>>,
  pub adj_matrix: Vec<Vec<usize>>,
  pub weighted: bool,
}

#[allow(unused)]
impl Graph {
  pub fn new(size: usize) -> Graph {
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
        let mut vec = vec![vec![]; size];
        vec
      },
      adj_matrix: {
        let mut vec = vec![vec![0; size]; size];
        vec
      },
      weighted: false,
    }
  }

  pub fn add_edge(&mut self, v: usize, w: usize) {
    if !self.contains_edge(v, w) && v < self.size && w < self.size {
      self.edges.push(edge::Edge {
        v: v,
        w: w,
        distance: 0,
      });
      self.adj_matrix[v][w] = 1;
      self.adj_matrix[w][v] = 1;
      if !self.contains_vertex(v) {
        self.vertices.push(v);
      }
      if !self.contains_neighbor(v, w) {
        self.adj_list[v].push(w);
      }
      if !self.contains_vertex(w) {
        self.vertices.push(w);
      }
      if !self.contains_neighbor(w, v) {
        self.adj_list[w].push(v);
      }
    }
  }

  pub fn add_weighted_edge(&mut self, v: usize, w: usize, distance: usize) {
    if !self.contains_edge(v, w) && v < self.size && w < self.size {
      self.weighted = true;
      self.edges.push(edge::Edge {
        v: v,
        w: w,
        distance: distance,
      });
      self.adj_matrix[v][w] = distance;
      self.adj_matrix[w][v] = distance;
      if !self.contains_vertex(v) {
        self.vertices.push(v);
      }
      if !self.contains_neighbor(v, w) {
        self.adj_list[v].push(w);
      }
      if !self.contains_vertex(w) {
        self.vertices.push(w);
      }
      if !self.contains_neighbor(w, v) {
        self.adj_list[w].push(v);
      }
    }
  }

  pub fn add_directed_edge(&mut self, v: usize, w: usize) {
    if !self.contains_edge(v, w) && v < self.size && w < self.size {
      self.edges.push(edge::Edge {
        v: v,
        w: w,
        distance: 0,
      });
      self.adj_matrix[v][w] = 1;
      if !self.contains_vertex(v) {
        self.vertices.push(v);
      }
      if !self.contains_neighbor(v, w) {
        self.adj_list[v].push(w);
      }
    }
  }

  pub fn add_directed_weighted_edge(&mut self, v: usize, w: usize, distance: usize) {
    if !self.contains_edge(v, w) && v < self.size && w < self.size {
      self.edges.push(edge::Edge {
        v: v,
        w: w,
        distance: 0,
      });
      self.adj_matrix[v][w] = distance;
      if !self.contains_vertex(v) {
        self.vertices.push(v);
      }
      if !self.contains_neighbor(v, w) {
        self.adj_list[v].push(w);
      }
    }
  }

  pub fn get_edge(&self, v: usize, w: usize) -> Option<&edge::Edge> {
    return self
      .edges
      .iter()
      .find(|ref edge| edge.v == v && edge.w == w);
  }

  pub fn get_neighbors(&self, v: usize) -> Option<&Vec<usize>> {
    if self.adj_list[v].len() > 0 {
      return Some(&(self.adj_list)[v]);
    }
    None
  }

  pub fn get_distance(&self, v: usize, w: usize) -> usize {
    if let Some(edge) = self.get_edge(v, w) {
      return edge.distance;
    }
    return 0;
  }

  pub fn contains_vertex(&self, v: usize) -> bool {
    return !self.vertices.is_empty() && self.vertices.contains(&v);
  }

  pub fn contains_edge(&self, v: usize, w: usize) -> bool {
    return !self.vertices.is_empty()
      && self.edges.iter().any(|ref edge| edge.v == v && edge.w == w);
  }

  pub fn contains_neighbor(&self, v: usize, w: usize) -> bool {
    return self.adj_list[v].contains(&w);
  }

  pub fn randomize_weights(&mut self, low: usize, high: usize) {
    let mut rng = thread_rng();
    for i in 0..self.size {
      for j in 0..self.size {
        let n: usize = rng.gen_range(low, high);
        if i != j {
          self.add_weighted_edge(i, j, n);
        }
      }
    }
  }

  pub fn randomize_directed_weights(&mut self, low: usize, high: usize) {
    let mut rng = thread_rng();
    for i in 0..self.size {
      for j in 0..self.size {
        let n: usize = rng.gen_range(low, high);
        self.add_directed_weighted_edge(i, j, n);
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

  pub fn print_neighbors(&self, v: usize) {
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
