use super::*;

fn bin_shrink(bin_type: BinType) {
    let mut bin = bin_new(bin_type, 16, 16);
    bin.insert(&Dimension::with_padding(4, 5, 1));
    bin.shrink(true);

    assert_eq!(8, bin.width());
    assert_eq!(8, bin.height());

    bin.shrink(false);
    assert_eq!(6, bin.width());
    assert_eq!(7, bin.height());
}

fn bin_grow(bin_type: BinType) {
    let mut bin = bin_new(bin_type, 8, 8);
    assert_eq!(8, bin.width());
    assert_eq!(8, bin.height());

    let result = bin.insert(&Dimension::with_padding(5, 5, 1));
    assert!(result.is_some());

    let result1 = bin.insert(&Dimension::new(8, 8));
    assert!(result1.is_none());

    bin.grow(8, 8);
    let result2 = bin.insert(&Dimension::new(8, 8));
    assert!(result2.is_some());
}

fn bin_insert(bin_type: BinType) {
    let nodes = vec![
        Dimension::with_padding(2, 4, 0),
        Dimension::with_padding(6, 4, 1),
        Dimension::with_padding(10, 3, 1),
        Dimension::with_padding(6, 6, 0),
        Dimension::with_padding(4, 4, 2),
        Dimension::with_padding(3, 8, 0),
        Dimension::with_padding(8, 3, 1),
    ];

    let mut bin = bin_new(bin_type, 16, 16);
    assert_eq!(16, bin.width());
    assert_eq!(16, bin.height());

    for node in &nodes {
        bin.insert(node);
    }

    for rect1 in bin.iter() {
        for rect2 in bin.iter() {
            if rect1 != rect2 {
                assert!(!rect1.intersects(rect2));
            }
        }
    }
}

fn bin_insert_list(bin_type: BinType) {
    let nodes = vec![
        Dimension::with_padding(2, 4, 0),
        Dimension::with_padding(6, 4, 1),
        Dimension::with_padding(10, 3, 1),
        Dimension::with_padding(6, 6, 0),
        Dimension::with_padding(4, 4, 2),
        Dimension::with_padding(3, 8, 0),
        Dimension::with_padding(8, 3, 1),
    ];

    let mut bin = bin_new(bin_type, 16, 16);
    assert_eq!(16, bin.width());
    assert_eq!(16, bin.height());

    let (inserted, rejected) = bin.insert_list(&nodes);
    assert_eq!(nodes.len(), inserted.len() + rejected.len());

    for rect1 in bin.iter() {
        for rect2 in bin.iter() {
            if rect1 != rect2 {
                assert!(!rect1.intersects(rect2));
            }
        }
    }
}

fn bin_find_by_id(bin_type: BinType) {
    let mut bin = bin_new(bin_type, 16, 16);
    bin.insert(&Dimension::with_id(1, 4, 4, 0));
    bin.insert(&Dimension::with_id(2, 1, 1, 1));

    let result = bin.find_by_id(2);
    assert!(result.is_some());
    assert_eq!(1, result.unwrap().dim().padding());
}

fn bin_iter_slice(bin_type: BinType) {
    let mut bin = bin_new(bin_type, 64, 64);
    for i in 1..5 {
        let result = bin.insert(&Dimension::with_id(i, i as i32, i as i32 * 2, i as i32 / 2));
        assert!(result.is_some());
    }

    assert_eq!(4, bin.len());

    for rect in bin.iter() {
        assert!(rect.id() > 0);
    }

    let rects = &bin.as_slice()[1..3];
    assert_eq!(2, rects.len());
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
fn bin_grow_maxrects() {
    bin_grow(BinType::MaxRects);
}

#[test]
fn bin_grow_guillotine() {
    bin_grow(BinType::Guillotine);
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

#[test]
fn bin_find_by_id_maxrects() {
    bin_find_by_id(BinType::MaxRects);
}

#[test]
fn bin_find_by_id_guillotine() {
    bin_find_by_id(BinType::Guillotine);
}

#[test]
fn bin_iter_slice_maxrects() {
    bin_iter_slice(BinType::MaxRects);
}

#[test]
fn bin_iter_slice_guillotine() {
    bin_iter_slice(BinType::Guillotine);
}
