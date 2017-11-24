mod graphlib;
use graphlib::graph;
use graphlib::algorithms;

fn main() {
  let mut g = graph::Graph::new(5);
  g.randomize(1, 10);
  g.print();
  println!("Neighbors of b");
  g.print_neighbors(1);
  match algorithms::bfs::serial::bfs(g, 0) {
    Some(result) => {
      let (distance, path) = result;
      println!("Distance: {}, path: {:?}", distance, path);
    },
    None => println!("No path was found."),
  }
}
