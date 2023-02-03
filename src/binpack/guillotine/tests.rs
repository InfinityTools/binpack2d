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
    assert_eq!(16, bin.width());
    assert_eq!(16, bin.height());

    for node in &nodes {
        bin.insert(node, false, choice, method);
    }

    for rect1 in bin.iter() {
        for rect2 in bin.iter() {
            if rect1 != rect2 {
                assert!(!rect1.intersects(rect2));
            }
        }
    }
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
    assert_eq!(16, bin.width());
    assert_eq!(16, bin.height());

    let (inserted, rejected) = bin.insert_list(&nodes, true, choice, method);
    assert_eq!(nodes.len(), inserted.len() + rejected.len());

    for rect1 in bin.iter() {
        for rect2 in bin.iter() {
            if rect1 != rect2 {
                assert!(!rect1.intersects(rect2));
            }
        }
    }
}
