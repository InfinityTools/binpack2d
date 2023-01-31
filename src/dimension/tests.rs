use super::Dimension;

#[test]
fn dimension_get() {
    let dim = Dimension::with_id(12, 50, 30);
    assert_eq!(12, dim.id());
    assert_eq!(50, dim.width());
    assert_eq!(30, dim.height());
}

#[test]
fn dimension_set() {
    let mut dim = Dimension::new(1, 1);

    dim.set_id(9);
    assert_eq!(9, dim.id());

    dim.set_dimension(33, 44);
    assert_eq!(33, dim.width());
    assert_eq!(44, dim.height());
}

#[test]
fn dimension_area() {
    let mut dim = Dimension::new(5, 8);
    assert_eq!(5 * 8, dim.area());

    dim.set_dimension(0, 99);
    assert_eq!(0, dim.area());
}
