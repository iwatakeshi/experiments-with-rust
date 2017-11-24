extern crate rand;
use self::rand::{thread_rng, Rng};
mod edge;

pub struct Graph {
  pub size: u32,
  pub edges: Vec<edge::Edge>,
  pub vertices: Vec<u32>,
  pub adj_list: Vec<Vec<u32>>,
  pub adj_matrix: Vec<Vec<u32>>,
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
        let mut vec = vec![vec![0; size as usize]; size as usize];
        vec
      },
      adj_matrix: {
        let mut vec = vec![vec![0; size as usize]; size as usize];
        vec
      },
    }
  }

  pub fn add_edge(&mut self, v: u32, w: u32, distance: u32) {
    if !self.contains_edge(v, w) && v < self.size && w < self.size {
      self.edges.push(edge::Edge {
        v: v,
        w: w,
        distance: distance,
      });
      self.adj_matrix[v as usize][w as usize] = distance;
      self.adj_matrix[w as usize][v as usize] = distance;
      if !self.contains_vertex(v) {
        self.vertices.push(v);
        self.adj_list[v as usize].push(w);
      }
      if !self.contains_vertex(w) {
        self.vertices.push(w);
        self.adj_list[w as usize].push(v);
      }
    }
  }

  pub fn contains_vertex(&mut self, v: u32) -> bool {
    let t = self.vertices.len() > 0 && self.vertices.iter().any(|vertex| vertex == &v);
    t
  }

  pub fn contains_edge(&mut self, v: u32, w: u32) -> bool {
    let t = self.edges.len() > 0 && self.edges.iter().any(|ref edge| edge.v == v && edge.w == w);
    t
  }
  pub fn randomize(&mut self, low: u32, high: u32) {
    let mut rng = thread_rng();
    for i in 0..self.size {
      for j in 0..self.size {
        let n: u32 = rng.gen_range(low, high);
        if i != j {
          self.add_edge(i, j, n);
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
}
