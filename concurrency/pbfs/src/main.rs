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

  match algorithms::bfs::serial::bfs(g, 5) {
    Some(result) => {
      // let (distance, path) = result;
      println!("Path: {:?}", result.1);
    }
    None => println!("No path was found."),
  }
}
