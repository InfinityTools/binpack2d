use std::time::Instant;
use rand::prelude::*;
use binpack2d::{BinPacker, Dimension};
use binpack2d::guillotine::{pack_bins, RectHeuristic, SplitHeuristic};

#[test]
fn bin_performance_guillotine() {
    let choices = vec![
        RectHeuristic::BestShortSideFit,
        RectHeuristic::BestLongSideFit,
        RectHeuristic::BestAreaFit,
        RectHeuristic::WorstShortSideFit,
        RectHeuristic::WorstLongSideFit,
        RectHeuristic::WorstAreaFit,
    ];

    let methods = vec![
        SplitHeuristic::ShorterLeftoverAxis,
        SplitHeuristic::LongerLeftoverAxis,
        SplitHeuristic::MinimizeArea,
        SplitHeuristic::MaximizeArea,
        SplitHeuristic::ShorterAxis,
        SplitHeuristic::LongerAxis,
    ];

    const DIM: i32 = 512;
    const SIZE: usize = 1_000;

    let mut rng = StdRng::seed_from_u64(123456789);
    let mut nodes = Vec::with_capacity(SIZE);
    for i in 1..=SIZE {
        nodes.push(Dimension::with_id(
            i as isize,
            rng.gen_range((DIM / 128).max(1)..(DIM / 16).max(2)),
            rng.gen_range((DIM / 128).max(1)..(DIM / 16).max(2)),
            0
        ));
    }

    for choice in &choices {
        for method in &methods {
            // running benchmark
            let now = Instant::now();
            let bins = pack_bins(&nodes, DIM, DIM, true, *choice, *method, false);
            let elapsed = now.elapsed();

            // presenting statistics
            println!("Packed {SIZE} nodes into {} {DIM}x{DIM} bin(s), with choice \"{choice:?}\" and method \"{method:?}\": {} ms",
                     bins.len(), elapsed.as_millis());

            for (idx, bin) in bins.iter().enumerate() {
                println!(
                    "Bin {idx} contains {} nodes (occupancy: {})...",
                    bin.len(),
                    bin.occupancy()
                );
            }
            println!();
        }
        println!();
    }
}
