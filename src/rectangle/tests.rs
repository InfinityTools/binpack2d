use crate::dimension::Dimension;
use super::Rectangle;

#[test]
fn rectangle_get() {
    let rect = Rectangle::new(5, 8, Dimension::with_id(1, 6, 7, 2));
    assert_eq!(5, rect.x());
    assert_eq!(8, rect.y());
    assert_eq!(3, rect.x_total());
    assert_eq!(6, rect.y_total());
    assert_eq!(6, rect.width());
    assert_eq!(7, rect.height());
    assert_eq!(6 * 7, rect.dim().area());
}

#[test]
fn rectangle_set() {
    let mut rect = Rectangle::new(0, 0, Dimension::with_id(1, 3, 4, 0));

    rect.set_x(1);
    assert_eq!(1, rect.x());

    rect.set_y(2);
    assert_eq!(2, rect.y());

    rect.set_location(4, 5);
    assert_eq!(4, rect.x());
    assert_eq!(5, rect.y());

    rect.dim_mut().set_dimension(7, 8);
    assert_eq!(7, rect.width());
    assert_eq!(8, rect.height());

    rect.translate(6, 5);
    assert_eq!(10, rect.x());
    assert_eq!(10, rect.y());

    rect.translate(-8, -8);
    assert_eq!(2, rect.x());
    assert_eq!(2, rect.y());

    rect.dim_mut().set_padding(2);
    assert_eq!(0, rect.x_total());
    assert_eq!(0, rect.y_total());
}

#[test]
fn rectangle_intersects() {
    let r1 = Rectangle::new(2, 2, Dimension::new(5, 5));
    let r2 = Rectangle::new(4, 4, Dimension::new(3, 3));
    let r3 = Rectangle::new(7, 7, Dimension::new(4, 4));

    assert!(r1.intersects(&r2));
    assert!(!r1.intersects(&r3));
}

#[test]
fn rectangle_contains() {
    let r1 = Rectangle::new(2, 2, Dimension::new(5, 5));
    let r2 = Rectangle::new(3, 3, Dimension::new(2, 2));
    let r3 = Rectangle::new(3, 3, Dimension::new(5, 5));
    let r4 = Rectangle::new(7, 7, Dimension::new(1, 1));
    let r5 = Rectangle::new(2, 2, Dimension::new(5, 5));

    assert!(r1.contains(&r2));
    assert!(!r1.contains(&r3));
    assert!(!r1.contains(&r4));

    assert!(r1.contains(&r5));
    assert!(r5.contains(&r1));

    let r6 = Rectangle::new(2, 2, Dimension::with_padding(5, 5, 2));
    let r7 = Rectangle::new(1, 1, Dimension::with_padding(3, 3, 1));
    assert!(!r6.contains(&r7));
    assert!(r6.contains_total(&r7));
}

#[test]
fn rectangle_union() {
    let r1 = Rectangle::new(2, 2, Dimension::new(5, 5));
    let r2 = Rectangle::new(7, 4, Dimension::new(2, 2));
    let r3 = Rectangle::new(10, 10, Dimension::new(5, 5));

    assert_eq!(Rectangle::new(2, 2, Dimension::with_id(1, 7, 5, 0)),
               r1.union(&r2, Some(1)));
    assert_eq!(Rectangle::new(2, 2, Dimension::with_id(2, 13, 13, 0)),
               r1.union(&r3, Some(2)));
}
