use crate::{GUILLOTINE_CHOICES, GUILLOTINE_METHODS, MAXRECTS_RULES, prepare_nodes_list};
use binpack2d::guillotine::GuillotineBin;
use binpack2d::maxrects::MaxRectsBin;
use binpack2d::BinPacker;
use std::time::Instant;

/// Measures performance for every heuristic rule of the MaxRects algorithm.
pub fn benchmark_maxrects() {
    let height: i32 = 1024;
    let width: i32 = height;

    let nodes = prepare_nodes_list(width, height);
    let mut bin = MaxRectsBin::with_capacity(width, height, nodes.len());

    println!("*** Benchmark for MaxRectsBin ***");
    for rule in MAXRECTS_RULES {
        bin.clear_with(nodes.len());

        let now = Instant::now();
        for node in &nodes {
            bin.insert(node, *rule);
        }
        let elapsed = now.elapsed();

        println!("{} of {} nodes inserted into {width}x{height} bin (rule: {rule:?}, occupancy: {:.2}): {} ms",
                 bin.len(), nodes.len(), bin.occupancy() * 100.0, elapsed.as_millis());
    }
    println!();
}

/// Measures performance for every heuristic rule of the Guillotine algorithm.
pub fn benchmark_guillotine() {
    let height: i32 = 1024;
    let width: i32 = height;

    let nodes = prepare_nodes_list(width, height);
    let mut bin = GuillotineBin::with_capacity(width, height, nodes.len());

    for choice in GUILLOTINE_CHOICES {
        println!("*** Benchmark for GuillotineBin: (choice: {choice:?}) ***");
        for method in GUILLOTINE_METHODS {
            bin.clear_with(nodes.len());

            let now = Instant::now();
            for node in &nodes {
                bin.insert(node, true, *choice, *method);
            }
            let elapsed = now.elapsed();

            println!("{} of {} nodes inserted into {width}x{height} bin (method: {method:?}, occupancy: {:.2}): {} ms",
                     bin.len(), nodes.len(), bin.occupancy() * 100.0, elapsed.as_millis());
        }
        println!();
    }
}
