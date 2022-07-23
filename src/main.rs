use petgraph::Directed;
use petgraph::Graph;
use std::error::Error;
use std::io;
use std::process;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    source: u32,
    target: u32,
}

fn example() -> Result<(), Box<dyn Error>> {
    let mut edge_vec: Vec<(u32, u32)> = Vec::new();
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Record = result?;
        edge_vec.push((record.source, record.target));
    }
    let gr = Graph::<u32, u32, Directed, u32>::from_edges(edge_vec);
    println!("{}", gr.node_count());
    Ok(())


}



fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}