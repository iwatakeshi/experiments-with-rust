/*
  Authors:
  Takeshi I.
*/
mod graphlib;
use graphlib::graph;
use graphlib::algorithms;

fn main() {
  let mut g = graph::Graph::new(4);

  g.add_directed_edge(0, 1);
  g.add_directed_edge(0, 2);
  g.add_directed_edge(1, 2);
  g.add_directed_edge(2, 0);
  g.add_directed_edge(2, 3);
  g.add_directed_edge(3, 3);
  g.print();

  match algorithms::bfs::serial::bfs(g, 3) {
    Some(result) => {
      let (distance, path) = result;
      println!("Distance: {}, Path: {:?}", distance, path);
    }
    None => println!("No path was found."),
  }
}
