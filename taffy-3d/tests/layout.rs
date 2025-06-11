use taffy_3d::layout::{Node3D, Style3D, compute_layout};
use taffy::style::AlignItems;
use taffy_3d::geometry::{Size3, Point3};
use taffy::style::Dimension;

#[test]
fn flex_row_positions_children() {
    let mut root = Node3D::new(Style3D::default(), vec![
        Node3D::new(Style3D { size: Size3 { width: Dimension::length(10.0), height: Dimension::length(5.0), depth: Dimension::length(1.0) }, ..Style3D::default() }, vec![]),
        Node3D::new(Style3D { size: Size3 { width: Dimension::length(4.0), height: Dimension::length(5.0), depth: Dimension::length(1.0) }, ..Style3D::default() }, vec![]),
    ]);

    compute_layout(&mut root);
    assert_eq!(root.children[0].layout.location, Point3 { x: 0.0, y: 0.0, z: 0.0 });
    assert_eq!(root.children[1].layout.location.x, 10.0);
    assert_eq!(root.layout.size.width, 14.0);
}

#[test]
fn wrap_places_child_on_new_line() {
    let mut root = Node3D::new(
        Style3D {
            size: Size3 { width: Dimension::length(10.0), height: Dimension::auto(), depth: Dimension::auto() },
            wrap: true,
            ..Style3D::default()
        },
        vec![
            Node3D::new(Style3D { size: Size3 { width: Dimension::length(6.0), height: Dimension::length(2.0), depth: Dimension::length(1.0) }, ..Style3D::default() }, vec![]),
            Node3D::new(Style3D { size: Size3 { width: Dimension::length(6.0), height: Dimension::length(2.0), depth: Dimension::length(1.0) }, ..Style3D::default() }, vec![]),
        ],
    );

    compute_layout(&mut root);
    assert_eq!(root.children[1].layout.location.y > 0.0, true);
}

#[test]
fn align_center_positions_child() {
    let mut root = Node3D::new(
        Style3D {
            size: Size3 { width: Dimension::length(10.0), height: Dimension::length(10.0), depth: Dimension::length(2.0) },
            align_items: AlignItems::Center,
            ..Style3D::default()
        },
        vec![Node3D::new(Style3D { size: Size3 { width: Dimension::length(2.0), height: Dimension::length(2.0), depth: Dimension::length(1.0) }, ..Style3D::default() }, vec![])],
    );

    compute_layout(&mut root);
    assert_eq!(root.children[0].layout.location.y, 4.0);
    assert_eq!(root.children[0].layout.location.z, 0.5);
}
