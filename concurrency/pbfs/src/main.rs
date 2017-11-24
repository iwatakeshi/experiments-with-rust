mod graph;

fn main() {
  let mut g = graph::Graph::new(5);
  g.randomize(1, 10);
  g.print();
}
