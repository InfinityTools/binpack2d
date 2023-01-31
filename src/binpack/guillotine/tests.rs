use crate::binpack::BinPacker;
use crate::dimension::Dimension;
use super::{RectHeuristic, SplitHeuristic, GuillotineBin};

#[test]
fn bin_shrink() {
    let mut bin = GuillotineBin::new(16, 16);
    bin.insert(&Dimension::new(6, 7), true,
               RectHeuristic::BestAreaFit, SplitHeuristic::MinimizeArea);
    bin.shrink(true);
    assert_eq!(8, bin.width());
    assert_eq!(8, bin.height());

    bin.shrink(false);
    assert_eq!(6, bin.width());
    assert_eq!(7, bin.height());
}

#[test]
fn bin_insert() {
    let nodes = vec![
        Dimension::new(2, 4),
        Dimension::new(8, 6),
        Dimension::new(12, 5),
        Dimension::new(6, 6),
        Dimension::new(8, 8),
        Dimension::new(3, 8),
        Dimension::new(10, 5),
    ];

    let choice = RectHeuristic::BestAreaFit;
    let method = SplitHeuristic::MinimizeArea;

    let mut bin = GuillotineBin::new(16, 16);
    println!("GuillotineBin::Insert: Creating bin({}, {}) using choice {:?} and method {:?}",
             bin.width(), bin.height(), choice, method);

    for node in &nodes {
        bin.insert(node, false, choice, method);
    }
    println!("{} node(s) in bin, {} node(s) rejected, occupancy: {}:\n{}",
             bin.len(), nodes.len() - bin.len(), bin.occupancy(), bin.visualize());
    println!("{bin}");
}

#[test]
fn bin_insert_list() {
    let mut nodes = vec![
        Dimension::new(2, 4),
        Dimension::new(8, 6),
        Dimension::new(12, 5),
        Dimension::new(6, 6),
        Dimension::new(8, 8),
        Dimension::new(3, 8),
        Dimension::new(10, 5),
    ];

    let choice = RectHeuristic::BestAreaFit;
    let method = SplitHeuristic::MinimizeArea;

    let mut bin = GuillotineBin::new(16, 16);
    println!("GuillotineBin::Insert_list: Creating bin({}, {}) using choice {:?} and method {:?}",
             bin.width(), bin.height(), choice, method);

    bin.insert_list(&mut nodes, true, choice, method);
    println!("{} node(s) in bin, {} node(s) rejected, occupancy: {}:\n{}",
             bin.len(), nodes.len(), bin.occupancy(), bin.visualize());
    println!("{bin}");
}
