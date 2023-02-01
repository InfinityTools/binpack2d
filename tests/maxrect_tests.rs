use std::time::Instant;
use rand::prelude::*;
use binpack2d::{BinPacker, Dimension};
use binpack2d::maxrects::{Heuristic, pack_bins};

#[test]
fn bin_performance_maxrect() {
    let rules = vec![
        Heuristic::BestShortSideFit,
        Heuristic::BestLongSideFit,
        Heuristic::BestAreaFit,
        Heuristic::BottomLeftRule,
        Heuristic::ContactPointRule,
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

    for rule in rules {
        // running benchmark
        let now = Instant::now();
        let bins = pack_bins(&nodes, DIM, DIM, rule, false);
        let elapsed = now.elapsed();

        // presenting statistics
        println!(
            "Packed {SIZE} nodes into {} {DIM}x{DIM} bin(s), with rule \"{rule:?}\": {} ms",
            bins.len(),
            elapsed.as_millis()
        );

        for (idx, bin) in bins.iter().enumerate() {
            println!(
                "Bin {idx} contains {} nodes (occupancy: {})...",
                bin.len(),
                bin.occupancy()
            );
        }
        println!();
    }
}
