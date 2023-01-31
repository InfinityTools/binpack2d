use crate::binpack::BinPacker;
use crate::dimension::Dimension;
use super::{Heuristic, MaxRectsBin};

#[test]
fn bin_shrink() {
    let mut bin = MaxRectsBin::new(16, 16);
    bin.insert(&Dimension::new(6, 7), Heuristic::BestShortSideFit);
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

    let rule = Heuristic::ContactPointRule;

    let mut bin = MaxRectsBin::new(16, 16);
    println!("MaxRectsBin::Insert: Creating bin({}, {}) using rule {:?}",
             bin.width(), bin.height(), rule);

    for node in nodes.iter() {
        bin.insert(node, rule);
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

    let rule = Heuristic::ContactPointRule;

    let mut bin = MaxRectsBin::new(16, 16);
    println!("MaxRectsBin::Insert_list: Creating bin({}, {}) using rule {:?}",
             bin.width(), bin.height(), rule);

    bin.insert_list(&mut nodes, rule);
    println!("{} node(s) in bin, {} node(s) rejected, occupancy: {}:\n{}",
             bin.len(), nodes.len(), bin.occupancy(), bin.visualize());
    println!("{bin}");
}
