use binpack2d::maxrects::{pack_bins, Heuristic};
use binpack2d::{BinError, BinPacker, Dimension};
use rand::prelude::*;
use std::time::Instant;

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
            0,
        ));
    }

    for rule in rules {
        // running benchmark
        let now = Instant::now();
        let bins_result = pack_bins(&nodes, DIM, DIM, rule, false);
        let elapsed = now.elapsed();

        if let Ok(bins) = bins_result {
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
        } else if let Err(err) = bins_result {
            println!("Error: {err}");
        }
        println!();
    }
}

#[test]
fn bin_failure() {
    let mut nodes = vec![
        Dimension::with_padding(2, 4, 0),
        Dimension::with_padding(6, 8, 1),
    ];

    nodes.push(Dimension::with_padding(20, 12, 0));

    let result1 = pack_bins(&nodes, 16, 16, Heuristic::BestShortSideFit, true);
    assert_eq!(BinError::ItemTooBig, result1.err().unwrap());

    let result2 = pack_bins(&nodes, 16, 16, Heuristic::BestShortSideFit, false);
    assert_eq!(BinError::ItemTooBig, result2.err().unwrap());

    nodes.pop();
    nodes.push(Dimension::with_padding(0, 64, 0));

    let result3 = pack_bins(&nodes, 16, 16, Heuristic::BestShortSideFit, true);
    assert_eq!(BinError::ItemTooSmall, result3.err().unwrap());

    let result4 = pack_bins(&nodes, 16, 16, Heuristic::BestShortSideFit, false);
    assert_eq!(BinError::ItemTooSmall, result4.err().unwrap());
}
