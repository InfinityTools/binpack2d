use super::*;

fn bin_shrink(bin_type: BinType) {
    let mut bin = bin_new(bin_type, 16, 16);
    bin.insert(&Dimension::new(6, 7));
    bin.shrink(true);

    assert_eq!(8, bin.width());
    assert_eq!(8, bin.height());

    bin.shrink(false);
    assert_eq!(6, bin.width());
    assert_eq!(7, bin.height());
}

fn bin_insert(bin_type: BinType) {
    let nodes = vec![
        Dimension::new(2, 4),
        Dimension::new(8, 6),
        Dimension::new(12, 5),
        Dimension::new(6, 6),
        Dimension::new(8, 8),
        Dimension::new(3, 8),
        Dimension::new(10, 5),
    ];

    let mut bin = bin_new(bin_type, 16, 16);
    println!("{:?}::Insert: Creating bin({}, {})", bin_type, bin.width(), bin.height());

    for node in &nodes {
        bin.insert(node);
    }
    println!("{} node(s) in bin, {} node(s) rejected, occupancy: {}:\n{}",
             bin.len(), nodes.len() - bin.len(), bin.occupancy(), bin.visualize());
    println!("{bin}");
}

fn bin_insert_list(bin_type: BinType) {
    let mut nodes = vec![
        Dimension::new(2, 4),
        Dimension::new(8, 6),
        Dimension::new(12, 5),
        Dimension::new(6, 6),
        Dimension::new(8, 8),
        Dimension::new(3, 8),
        Dimension::new(10, 5),
    ];

    let mut bin = bin_new(bin_type, 16, 16);
    println!("{:?}::Insert_list: Creating bin({}, {})", bin_type, bin.width(), bin.height());

    bin.insert_list(&mut nodes);
    println!("{} node(s) in bin, {} node(s) rejected, occupancy: {}:\n{}",
             bin.len(), nodes.len(), bin.occupancy(), bin.visualize());
    println!("{bin}");
}


#[test]
fn bin_shrink_maxrects() {
    bin_shrink(BinType::MaxRects);
}

#[test]
fn bin_shrink_guillotine() {
    bin_shrink(BinType::Guillotine);
}

#[test]
fn bin_insert_maxrects() {
    bin_insert(BinType::MaxRects);
}

#[test]
fn bin_insert_guillotine() {
    bin_insert(BinType::Guillotine);
}

#[test]
fn bin_insert_list_maxrects() {
    bin_insert_list(BinType::MaxRects);
}

#[test]
fn bin_insert_list_guillotine() {
    bin_insert_list(BinType::Guillotine);
}
