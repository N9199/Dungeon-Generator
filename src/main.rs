mod graph_gen;
mod geometry;
mod kd_tree;

fn main() {
    let d = graph_gen::Dungeon::new(1, 0.5, vec![(1000, 0.4); 1]);
    println!("{}", d);
}
