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
        Dimension::with_padding(2, 4, 0),
        Dimension::with_padding(6, 4, 1),
        Dimension::with_padding(10, 3, 1),
        Dimension::with_padding(6, 6, 0),
        Dimension::with_padding(4, 4, 2),
        Dimension::with_padding(3, 8, 0),
        Dimension::with_padding(8, 3, 1),
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
    let nodes = vec![
        Dimension::with_padding(2, 4, 0),
        Dimension::with_padding(6, 4, 1),
        Dimension::with_padding(10, 3, 1),
        Dimension::with_padding(6, 6, 0),
        Dimension::with_padding(4, 4, 2),
        Dimension::with_padding(3, 8, 0),
        Dimension::with_padding(8, 3, 1),
    ];

    let rule = Heuristic::ContactPointRule;

    let mut bin = MaxRectsBin::new(16, 16);
    println!("MaxRectsBin::Insert_list: Creating bin({}, {}) using rule {:?}",
             bin.width(), bin.height(), rule);

    bin.insert_list(&nodes, rule);
    println!("{} node(s) in bin, {} node(s) rejected, occupancy: {}:\n{}",
             bin.len(), nodes.len(), bin.occupancy(), bin.visualize());
    println!("{bin}");
}
