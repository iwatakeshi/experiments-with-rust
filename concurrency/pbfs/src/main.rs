mod graph;

fn main() {
  let mut g = graph::Graph::new(5);
  g.randomize(1, 10);
  g.print();
  println!("Neighbors of a");
  for i in g.get_neighbors(0) {
    print!("{number:>width$}", number = i, width = 3);
  }
  println!("");
}
