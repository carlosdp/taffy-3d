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

/// 3-D layout types and a very small flexbox implementation
pub mod layout {
    use crate::geometry::{Point3, Size3};
    use taffy::style::{AlignItems, Dimension, JustifyContent};

    /// Which axis children should be layed out along
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum FlexDirection3D {
        Row,
        Column,
        Depth,
    }

    impl Default for FlexDirection3D {
        fn default() -> Self {
            Self::Row
        }
    }

    /// The style used for 3-D layout nodes
    #[derive(Debug, Clone)]
    pub struct Style3D {
        pub size: Size3<Dimension>,
        pub flex_direction: FlexDirection3D,
        pub gap: f32,
        pub wrap: bool,
        pub justify_content: JustifyContent,
        pub align_items: AlignItems,
    }

    impl Default for Style3D {
        fn default() -> Self {
            Self {
                size: Size3 { width: Dimension::auto(), height: Dimension::auto(), depth: Dimension::auto() },
                flex_direction: FlexDirection3D::Row,
                gap: 0.0,
                wrap: false,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Stretch,
            }
        }
    }

    /// Output layout of a node
    #[derive(Debug, Default, Clone)]
    pub struct Layout3D {
        pub location: Point3<f32>,
        pub size: Size3<f32>,
    }

    /// Node in the 3-D layout tree
    #[derive(Debug, Default, Clone)]
    pub struct Node3D {
        pub style: Style3D,
        pub children: Vec<Node3D>,
        pub layout: Layout3D,
    }

    impl Node3D {
        pub fn new(style: Style3D, children: Vec<Node3D>) -> Self {
            Self { style, children, layout: Layout3D::default() }
        }
    }

    /// Compute layout using a naive flexbox algorithm extended to a third axis
    pub fn compute_layout(node: &mut Node3D) {
        for child in &mut node.children {
            compute_layout(child);
        }

        match node.style.flex_direction {
            FlexDirection3D::Row => layout_row(node),
            FlexDirection3D::Column => layout_column(node),
            FlexDirection3D::Depth => layout_depth(node),
        }
    }

    fn resolve(dim: Dimension) -> Option<f32> {
        if dim.is_auto() { None } else { Some(dim.value()) }
    }

    fn layout_row(node: &mut Node3D) {
        let width_limit = resolve(node.style.size.width).unwrap_or(f32::INFINITY);
        let mut lines: Vec<Vec<usize>> = vec![Vec::new()];
        let mut line_width: f32 = 0.0;
        let mut line_height: f32 = 0.0;
        let mut max_depth: f32 = 0.0;

        for (idx, child) in node.children.iter().enumerate() {
            let w = child.layout.size.width;
            let h = child.layout.size.height;
            max_depth = max_depth.max(child.layout.size.depth);

            if node.style.wrap && line_width > 0.0 && line_width + w > width_limit {
                lines.push(Vec::new());
                line_width = 0.0;
                line_height = 0.0;
            }

            lines.last_mut().unwrap().push(idx);
            line_width += w + node.style.gap;
            line_height = line_height.max(h);
        }

        let mut container_width = resolve(node.style.size.width).unwrap_or_else(|| {
            lines
                .iter()
                .map(|line| {
                    line.iter().fold(0.0_f32, |acc, &i| acc + node.children[i].layout.size.width)
                        + node.style.gap * (line.len().saturating_sub(1) as f32)
                })
                .fold(0.0_f32, f32::max)
        });

        let mut container_height = resolve(node.style.size.height).unwrap_or(0.0);

        if container_height == 0.0 {
            let mut h: f32 = 0.0;
            for line in &lines {
                let lh = line.iter().fold(0.0_f32, |acc, &i| acc.max(node.children[i].layout.size.height));
                h += lh + node.style.gap;
            }
            if !lines.is_empty() {
                h -= node.style.gap;
            }
            container_height = h;
        }

        let depth = resolve(node.style.size.depth).unwrap_or(max_depth);
        node.layout.size = Size3 { width: container_width, height: container_height, depth };

        let mut y = 0.0;
        for line in lines {
            let line_height_val = line.iter().fold(0.0_f32, |acc, &i| acc.max(node.children[i].layout.size.height));
            let line_width_total = line.iter().fold(0.0_f32, |acc, &i| acc + node.children[i].layout.size.width)
                + node.style.gap * (line.len().saturating_sub(1) as f32);
            let free = container_width - line_width_total;
            let (mut cursor_x, between_gap) = match node.style.justify_content {
                JustifyContent::FlexStart => (0.0, node.style.gap),
                JustifyContent::Center => (free / 2.0, node.style.gap),
                JustifyContent::FlexEnd => (free, node.style.gap),
                JustifyContent::SpaceBetween if line.len() > 1 => (0.0, free / (line.len() - 1) as f32),
                _ => (0.0, node.style.gap),
            };

            for &i in &line {
                let child = &mut node.children[i];
                let cross_free = container_height - child.layout.size.height;
                let stack_free = depth - child.layout.size.depth;

                let y_off = match node.style.align_items {
                    AlignItems::FlexStart => 0.0,
                    AlignItems::Center => cross_free / 2.0,
                    AlignItems::FlexEnd => cross_free,
                    AlignItems::Stretch => {
                        child.layout.size.height = line_height_val;
                        0.0
                    }
                    _ => 0.0,
                };

                let z_off = match node.style.align_items {
                    AlignItems::FlexStart => 0.0,
                    AlignItems::Center => stack_free / 2.0,
                    AlignItems::FlexEnd => stack_free,
                    AlignItems::Stretch => {
                        child.layout.size.depth = depth;
                        0.0
                    }
                    _ => 0.0,
                };

                child.layout.location.x = cursor_x;
                child.layout.location.y = y + y_off;
                child.layout.location.z = z_off;
                cursor_x += child.layout.size.width + between_gap;
            }
            y += line_height_val + node.style.gap;
        }
    }

    fn layout_column(node: &mut Node3D) {
        let height_limit = resolve(node.style.size.height).unwrap_or(f32::INFINITY);
        let mut lines: Vec<Vec<usize>> = vec![Vec::new()];
        let mut line_height: f32 = 0.0;
        let mut line_width: f32 = 0.0;
        let mut max_depth: f32 = 0.0;

        for (idx, child) in node.children.iter().enumerate() {
            let w = child.layout.size.width;
            let h = child.layout.size.height;
            max_depth = max_depth.max(child.layout.size.depth);

            if node.style.wrap && line_height > 0.0 && line_height + h > height_limit {
                lines.push(Vec::new());
                line_height = 0.0;
                line_width = 0.0;
            }

            lines.last_mut().unwrap().push(idx);
            line_height += h + node.style.gap;
            line_width = line_width.max(w);
        }

        let mut container_height = resolve(node.style.size.height).unwrap_or_else(|| {
            lines
                .iter()
                .map(|line| {
                    line.iter().fold(0.0_f32, |acc, &i| acc + node.children[i].layout.size.height)
                        + node.style.gap * (line.len().saturating_sub(1) as f32)
                })
                .fold(0.0_f32, f32::max)
        });

        let mut container_width = resolve(node.style.size.width).unwrap_or(0.0);
        if container_width == 0.0 {
            let mut w: f32 = 0.0;
            for line in &lines {
                let lw = line.iter().fold(0.0_f32, |acc, &i| acc.max(node.children[i].layout.size.width));
                w += lw + node.style.gap;
            }
            if !lines.is_empty() {
                w -= node.style.gap;
            }
            container_width = w;
        }

        let depth = resolve(node.style.size.depth).unwrap_or(max_depth);
        node.layout.size = Size3 { width: container_width, height: container_height, depth };

        let mut x = 0.0;
        for line in lines {
            let line_width_val = line.iter().fold(0.0_f32, |acc, &i| acc.max(node.children[i].layout.size.width));
            let line_height_total = line.iter().fold(0.0_f32, |acc, &i| acc + node.children[i].layout.size.height)
                + node.style.gap * (line.len().saturating_sub(1) as f32);
            let free = container_height - line_height_total;
            let (mut cursor_y, between_gap) = match node.style.justify_content {
                JustifyContent::FlexStart => (0.0, node.style.gap),
                JustifyContent::Center => (free / 2.0, node.style.gap),
                JustifyContent::FlexEnd => (free, node.style.gap),
                JustifyContent::SpaceBetween if line.len() > 1 => (0.0, free / (line.len() - 1) as f32),
                _ => (0.0, node.style.gap),
            };

            for &i in &line {
                let child = &mut node.children[i];
                let cross_free = line_width_val - child.layout.size.width;
                let stack_free = depth - child.layout.size.depth;

                let x_off = match node.style.align_items {
                    AlignItems::FlexStart => 0.0,
                    AlignItems::Center => cross_free / 2.0,
                    AlignItems::FlexEnd => cross_free,
                    AlignItems::Stretch => {
                        child.layout.size.width = line_width_val;
                        0.0
                    }
                    _ => 0.0,
                };

                let z_off = match node.style.align_items {
                    AlignItems::FlexStart => 0.0,
                    AlignItems::Center => stack_free / 2.0,
                    AlignItems::FlexEnd => stack_free,
                    AlignItems::Stretch => {
                        child.layout.size.depth = depth;
                        0.0
                    }
                    _ => 0.0,
                };

                child.layout.location.x = x + x_off;
                child.layout.location.y = cursor_y;
                child.layout.location.z = z_off;
                cursor_y += child.layout.size.height + between_gap;
            }
            x += line_width_val + node.style.gap;
        }
    }

    fn layout_depth(node: &mut Node3D) {
        let depth_limit = resolve(node.style.size.depth).unwrap_or(f32::INFINITY);

        let mut cursor_z: f32 = 0.0;
        let mut width: f32 = 0.0;
        let mut height: f32 = 0.0;
        for child in &mut node.children {
            if node.style.wrap && cursor_z > 0.0 && cursor_z + child.layout.size.depth > depth_limit {
                // wrapping in depth direction is not yet supported; start from zero
                cursor_z = 0.0;
                width += child.layout.size.width + node.style.gap;
                height = height.max(child.layout.size.height);
            }

            child.layout.location.z = cursor_z;
            cursor_z += child.layout.size.depth + node.style.gap;
            width = width.max(child.layout.size.width);
            height = height.max(child.layout.size.height);
        }

        if cursor_z > 0.0 {
            cursor_z -= node.style.gap;
        }

        node.layout.size.width = resolve(node.style.size.width).unwrap_or(width);
        node.layout.size.height = resolve(node.style.size.height).unwrap_or(height);
        node.layout.size.depth = resolve(node.style.size.depth).unwrap_or(cursor_z);
    }

    /// Simple block layout: stacks children in the Y axis and keeps maximum width/depth
    pub fn compute_block_layout(node: &mut Node3D) {
        for child in &mut node.children {
            compute_block_layout(child);
        }

        let mut y: f32 = 0.0;
        let mut width: f32 = 0.0;
        let mut depth: f32 = 0.0;
        for child in &mut node.children {
            child.layout.location.y = y;
            y += child.layout.size.height + node.style.gap;
            width = width.max(child.layout.size.width);
            depth = depth.max(child.layout.size.depth);
        }
        node.layout.size = Size3 { width, height: if y > 0.0 { y - node.style.gap } else { 0.0 }, depth };
    }

    /// Simple grid layout dividing the container equally
    pub fn compute_grid_layout(node: &mut Node3D, cols: usize, rows: usize, layers: usize) {
        let cell_w = node.layout.size.width / cols as f32;
        let cell_h = node.layout.size.height / rows as f32;
        let cell_d = node.layout.size.depth / layers as f32;

        for (i, child) in node.children.iter_mut().enumerate() {
            let c = i % cols;
            let r = (i / cols) % rows;
            let l = i / (cols * rows);
            child.layout.location = Point3 { x: c as f32 * cell_w, y: r as f32 * cell_h, z: l as f32 * cell_d };
            child.layout.size = Size3 { width: cell_w, height: cell_h, depth: cell_d };
        }
    }
}
