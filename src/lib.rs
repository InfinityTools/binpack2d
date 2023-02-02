use binpack2d::Dimension;
use binpack2d::guillotine::{RectHeuristic, SplitHeuristic};
use binpack2d::maxrects::Heuristic;
use rand::prelude::{Rng, StdRng, SeedableRng};

pub mod benchmarks;
pub mod bin_svg;

/// List of available MaxRects heuristic rules
const MAXRECTS_RULES: &'static [Heuristic] = &[
    Heuristic::BestShortSideFit,
    Heuristic::BestLongSideFit,
    Heuristic::BestAreaFit,
    Heuristic::BottomLeftRule,
    Heuristic::ContactPointRule,
];

/// List of available Guillotine heuristic choices
const GUILLOTINE_CHOICES: &'static [RectHeuristic] = &[
    RectHeuristic::BestShortSideFit,
    RectHeuristic::BestLongSideFit,
    RectHeuristic::BestAreaFit,
    RectHeuristic::WorstShortSideFit,
    RectHeuristic::WorstLongSideFit,
    RectHeuristic::WorstAreaFit,
];

/// List of available Guillotine split methods
const GUILLOTINE_METHODS: &'static [SplitHeuristic] = &[
    SplitHeuristic::ShorterLeftoverAxis,
    SplitHeuristic::LongerLeftoverAxis,
    SplitHeuristic::MinimizeArea,
    SplitHeuristic::MaximizeArea,
    SplitHeuristic::ShorterAxis,
    SplitHeuristic::LongerAxis,
];

/// Prepares a list of items for the bin packing benchmarks.
fn prepare_nodes_list(width: i32, height: i32) -> Vec<Dimension> {
    let size = (width + height) as usize;
    let min: u32 = 2;
    let max: u32 = (width.min(height) as u32 / 16).max(min + 1);

    let mut rng = StdRng::seed_from_u64(123456789);
    let mut list = Vec::with_capacity(size);
    for i in 0..size {
        list.push(Dimension::with_id(
            i as isize,
            rng.gen_range(min..max) as i32,
            rng.gen_range(min..max) as i32,
            0,
        ));
    }

    list
}
