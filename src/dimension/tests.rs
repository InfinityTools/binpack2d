use super::Dimension;

#[test]
fn dimension_get() {
    let dim = Dimension::with_id(12, 50, 30, 3);
    assert_eq!(12, dim.id());
    assert_eq!(50, dim.width());
    assert_eq!(30, dim.height());
    assert_eq!(3, dim.padding());
    assert_eq!(56, dim.width_total());
    assert_eq!(36, dim.height_total());
}

#[test]
fn dimension_set() {
    let mut dim = Dimension::new(1, 1);

    dim.set_id(9);
    assert_eq!(9, dim.id());

    dim.set_dimension(33, 44);
    assert_eq!(33, dim.width());
    assert_eq!(44, dim.height());

    dim.set_padding(3);
    assert_eq!(39, dim.width_total());
    assert_eq!(50, dim.height_total());
}

#[test]
fn dimension_area() {
    let mut dim = Dimension::with_padding(5, 8, 3);
    assert_eq!(5 * 8, dim.area());
    assert_eq!(11 * 14, dim.area_total());

    dim.set_dimension(0, 99);
    assert_eq!(0, dim.area());
    assert_eq!(6 * 105, dim.area_total());
}

#[test]
fn dimension_flip() {
    let mut dim1 = Dimension::with_padding(3, 5, 1);

    dim1.flip();
    assert_eq!(5, dim1.width());
    assert_eq!(3, dim1.height());

    let dim2 = dim1.to_flipped();
    assert_eq!(3, dim2.width());
    assert_eq!(5, dim2.height());
}

#[test]
fn dimension_empty() {
    let dim1 = Dimension::new(0, 0);
    assert!(dim1.is_empty());
    assert!(dim1.is_empty_total());

    let dim2 = Dimension::with_padding(0, 0, 1);
    assert!(dim2.is_empty());
    assert!(!dim2.is_empty_total());

    let dim3 = Dimension::new(5, 0);
    assert!(dim3.is_empty());
    assert!(dim3.is_empty_total());

    let dim4 = Dimension::with_padding(5, 0, 1);
    assert!(dim4.is_empty());
    assert!(!dim4.is_empty_total());
}

#[test]
fn dimension_default() {
    let dim = Dimension::default();
    assert_eq!(0, dim.width());
    assert_eq!(0, dim.height());
    assert_eq!(0, dim.width_total());
    assert_eq!(0, dim.height_total());
    assert!(dim.is_empty());
    assert!(dim.is_empty_total());
}
