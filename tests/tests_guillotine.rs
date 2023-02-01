use binpack2d::guillotine::{pack_bins, RectHeuristic, SplitHeuristic};
use binpack2d::{BinError, BinPacker, Dimension};
use rand::prelude::*;
use std::time::Instant;

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
            0,
        ));
    }

    for choice in &choices {
        for method in &methods {
            // running benchmark
            let now = Instant::now();
            let bins_result = pack_bins(&nodes, DIM, DIM, true, *choice, *method, false);
            let elapsed = now.elapsed();

            if let Ok(bins) = bins_result {
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
            } else if let Err(err) = bins_result {
                println!("Error: {err}");
            }
            println!();
        }
        println!();
    }
}

#[test]
fn bin_failure() {
    let choice = RectHeuristic::BestAreaFit;
    let method = SplitHeuristic::MinimizeArea;

    let mut nodes = vec![
        Dimension::with_padding(2, 4, 0),
        Dimension::with_padding(6, 8, 1),
    ];

    nodes.push(Dimension::with_padding(20, 12, 0));

    let result1 = pack_bins(&nodes, 16, 16, true, choice, method, true);
    assert_eq!(BinError::ItemTooBig, result1.err().unwrap());

    let result2 = pack_bins(&nodes, 16, 16, true, choice, method, false);
    assert_eq!(BinError::ItemTooBig, result2.err().unwrap());

    nodes.pop();
    nodes.push(Dimension::with_padding(0, 64, 0));

    let result3 = pack_bins(&nodes, 16, 16, true, choice, method, true);
    assert_eq!(BinError::ItemTooSmall, result3.err().unwrap());

    let result4 = pack_bins(&nodes, 16, 16, true, choice, method, false);
    assert_eq!(BinError::ItemTooSmall, result4.err().unwrap());
}
