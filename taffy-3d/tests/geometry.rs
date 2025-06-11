use taffy_3d::prelude::*;

#[test]
fn size3_conversions() {
    let size3 = Size3 { width: 1u32, height: 2u32, depth: 3u32 };
    let size2: taffy::prelude::Size<u32> = size3.into();
    assert_eq!(size2.width, 1);
    assert_eq!(size2.height, 2);
    let back: Size3<u32> = size2.into();
    assert_eq!(back, Size3 { width: 1, height: 2, depth: 0 });
}

#[test]
fn point3_conversions() {
    use taffy::geometry::Point;

    let pt3 = Point3 { x: 4i32, y: 5i32, z: 6i32 };
    let pt2: Point<i32> = pt3.into();
    assert_eq!(pt2.x, 4);
    assert_eq!(pt2.y, 5);
    let back: Point3<i32> = pt2.into();
    assert_eq!(back, Point3 { x: 4, y: 5, z: 0 });
}

#[test]
fn zero_constants() {
    assert_eq!(Size3::ZERO, Size3 { width: 0.0, height: 0.0, depth: 0.0 });
    assert_eq!(Point3::ZERO, Point3 { x: 0.0, y: 0.0, z: 0.0 });
}
