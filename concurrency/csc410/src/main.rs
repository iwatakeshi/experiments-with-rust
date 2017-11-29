/*
  Authors:
  Takeshi I.
*/
mod graphlib;
use graphlib::graph::Graph;
use graphlib::algorithms::bfs;

fn main() {
  let mut g = Graph::new(4);

  g.add_directed_edge(0, 1);
  g.add_directed_edge(0, 2);
  g.add_directed_edge(1, 2);
  g.add_directed_edge(2, 0);
  g.add_directed_edge(2, 3);
  g.add_directed_edge(3, 3);
  g.print();

  match bfs::serial::search(&g, 2) {
    Some(result) => {
      // let (distance, path) = result;
      println!("Path: {:?}", result.1);
    }
    None => println!("No path was found."),
  }

  match bfs::parallel::search(&g, 2) {
    Some(result) => {
      // let (distance, path) = result;
      println!("Path: {:?}", result.1);
    }
    None => println!("No path was found."),
  }
}
