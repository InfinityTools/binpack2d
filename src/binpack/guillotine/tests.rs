use super::*;

#[test]
fn bin_shrink() {
    let mut bin = GuillotineBin::new(16, 16);
    bin.insert(
        &Dimension::new(6, 7),
        true,
        RectHeuristic::BestAreaFit,
        SplitHeuristic::MinimizeArea,
    );
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

    let choice = RectHeuristic::BestAreaFit;
    let method = SplitHeuristic::MinimizeArea;

    let mut bin = GuillotineBin::new(16, 16);
    println!(
        "GuillotineBin::Insert: Creating bin({}, {}) using choice {:?} and method {:?}",
        bin.width(),
        bin.height(),
        choice,
        method
    );

    for node in &nodes {
        bin.insert(node, false, choice, method);
    }
    println!(
        "{} node(s) in bin, {} node(s) rejected, occupancy: {}:\n{}",
        bin.len(),
        nodes.len() - bin.len(),
        bin.occupancy(),
        bin.visualize()
    );
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

    let choice = RectHeuristic::BestAreaFit;
    let method = SplitHeuristic::MinimizeArea;

    let mut bin = GuillotineBin::new(16, 16);
    println!(
        "GuillotineBin::Insert_list: Creating bin({}, {}) using choice {:?} and method {:?}",
        bin.width(),
        bin.height(),
        choice,
        method
    );

    bin.insert_list(&nodes, true, choice, method);
    println!(
        "{} node(s) in bin, {} node(s) rejected, occupancy: {}:\n{}",
        bin.len(),
        nodes.len(),
        bin.occupancy(),
        bin.visualize()
    );
    println!("{bin}");
}
