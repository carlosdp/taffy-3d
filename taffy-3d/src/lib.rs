pub use taffy;

pub mod geometry {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
    pub struct Size3<T> {
        pub width: T,
        pub height: T,
        pub depth: T,
    }

    impl Size3<f32> {
        pub const ZERO: Self = Self { width: 0.0, height: 0.0, depth: 0.0 };
    }

    impl<T> Size3<T> {
        pub fn map<R, F>(self, f: F) -> Size3<R>
        where
            F: Fn(T) -> R,
        {
            Size3 { width: f(self.width), height: f(self.height), depth: f(self.depth) }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
    pub struct Point3<T> {
        pub x: T,
        pub y: T,
        pub z: T,
    }

    impl Point3<f32> {
        pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0 };
    }

    impl<T> Point3<T> {
        pub fn map<R, F>(self, f: F) -> Point3<R>
        where
            F: Fn(T) -> R,
        {
            Point3 { x: f(self.x), y: f(self.y), z: f(self.z) }
        }
    }

    use taffy::geometry::{Point, Size};

    impl<T> From<Size3<T>> for Size<T> {
        fn from(value: Size3<T>) -> Self {
            Size { width: value.width, height: value.height }
        }
    }

    impl<T: Default> From<Size<T>> for Size3<T> {
        fn from(value: Size<T>) -> Self {
            Size3 { width: value.width, height: value.height, depth: T::default() }
        }
    }

    impl<T> From<Point3<T>> for Point<T> {
        fn from(value: Point3<T>) -> Self {
            Point { x: value.x, y: value.y }
        }
    }

    impl<T: Default> From<Point<T>> for Point3<T> {
        fn from(value: Point<T>) -> Self {
            Point3 { x: value.x, y: value.y, z: T::default() }
        }
    }
}

pub mod prelude {
    pub use crate::geometry::{Point3, Size3};
    pub use taffy::prelude::*;
}
