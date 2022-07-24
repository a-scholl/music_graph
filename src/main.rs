use petgraph::Directed;
use petgraph::EdgeDirection::Outgoing;
use petgraph::Graph;
use petgraph::algo;
use petgraph::graph::NodeIndex;
// /use core::num;
use std::env;
use std::error::Error;
use std::io;
use std::process;

use serde::Deserialize;
use std::io::Write;                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  
                                 

fn write(data: Vec<NodeIndex>) -> () {
    // let data: Vec<Vec<u32>> = vec![vec![1, 2], vec![3, 4, 5]];                                                                                                        
    let strings: Vec<String> = data.iter().map(|n| n.index().to_string()).collect();   
    writeln!(io::stdout(), "{}", strings.join(", ")).expect("oops");                                                   
}

#[derive(Debug, Deserialize)]
struct Record {
    source: u32,
    target: u32,
}

fn example() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let path_to_csv = &args[1];
    let min_total_songs = args[2].parse::<usize>().unwrap();
    let max_total_songs = args[3].parse::<usize>().unwrap();
    let max_paths_per_source = args[4].parse::<usize>().unwrap();
    // let max_paths_per_source_and_target = args[5].parse::<usize>().unwrap();
    

    let mut edge_vec: Vec<(u32, u32)> = Vec::new();
    let mut rdr = csv::Reader::from_path(path_to_csv).unwrap();
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Record = result?;
        edge_vec.push((record.source, record.target));
    }
    let gr = Graph::<u32, u32, Directed, u32>::from_edges(edge_vec);



    for node_ix in gr.node_indices() {
        let mut num_found_for_current: usize = 0;
        'neighbor: for (neighbor_idx, neighbor) in gr.neighbors_directed(node_ix, Outgoing).enumerate() {
            // if neighbor_idx >= max_paths_per_source {
            //     break 'neighbor;
            // }
            for num_intermediate in (min_total_songs-2)..(max_total_songs-2) {
                let mut paths_back = algo::all_simple_paths::<Vec<_>, _>(
                    &gr, 
                    neighbor, 
                    node_ix,
                    num_intermediate,
                    Some(num_intermediate)
                );
                // just take first one
                match paths_back.next() {
                    None => continue,
                    Some(v) => {
                        num_found_for_current += 1;
                        write(v);
                    }
                }
                if num_found_for_current == max_paths_per_source {
                    break 'neighbor;
                }
            }

            // 'inner: for (path_number, path) in paths_back.enumerate() {
            //     if path_number >= max_paths_per_source_and_target {
            //         break 'inner;
            //     }
            //     write(path);
            // }
        }
    }
    
    Ok(())
}



fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}