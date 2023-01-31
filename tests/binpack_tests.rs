use std::time::Instant;
use rand::prelude::*;
use binpack2d::{pack_bins, BinType, Dimension};

#[test]
fn bin_performance_binpack() {
    let bin_types = vec![BinType::MaxRects, BinType::Guillotine];

    const DIM: u32 = 512;
    const SIZE: usize = 1_000;

    let mut rng = StdRng::seed_from_u64(123456789);
    let mut nodes = Vec::with_capacity(SIZE);
    for i in 1..=SIZE {
        nodes.push(Dimension::with_id(
            i as isize,
            rng.gen_range((DIM / 128).max(1)..(DIM / 16).max(2)),
            rng.gen_range((DIM / 128).max(1)..(DIM / 16).max(2)),
        ));
    }

    for bin_type in &bin_types {
        // running benchmark
        let now = Instant::now();
        let bins = pack_bins(*bin_type, &nodes, DIM, DIM, true);
        let elapsed = now.elapsed();

        // presenting statistics
        println!(
            "Packed {SIZE} nodes into {} {DIM}x{DIM} bin(s), with bin type \"{bin_type:?}\": {} ms",
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
