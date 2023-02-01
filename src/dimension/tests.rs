use super::Dimension;

#[test]
fn dimension_get() {
    let dim = Dimension::with_id(12, 50, 30, 3);
    assert_eq!(12, dim.id());
    assert_eq!(50, dim.width());
    assert_eq!(30, dim.height());
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
