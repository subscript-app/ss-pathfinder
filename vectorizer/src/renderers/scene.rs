use ss_pathfinder_color::{ColorF, ColorU};
use ss_pathfinder_canvas::{Canvas, CanvasFontContext, Path2D};
use ss_pathfinder_geometry::vector::{Vector2I, Vector2F, Vector4F};


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// DATA TYPES
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

#[derive(Default, Debug)]
pub struct VScene {
    pub(crate) polygons: Vec<VShape>,
}

#[derive(Debug, Clone)]
pub struct VShape {
    pub path: Path2D,
    pub color: ColorU,
    pub shape_type: ShapeType,
}

#[derive(Debug, Clone)]
pub enum ShapeType {
    Fill,
    Stroke {line_width: f32},
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// API
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

impl VScene {
    pub fn clear(&mut self) {
        self.polygons.clear();
    }
    pub fn add_polygon(&mut self, polygon: impl Into<VShape>) {
        self.polygons.push(polygon.into());
    }
}

impl VShape {
    pub fn from_iter<T: Into<Vector2F>>(
        shape_type: ShapeType,
        color: ColorU,
        points: impl IntoIterator<Item = T>
    ) -> VShape {
        let mut path = Path2D::new();
        for (ix, point) in points.into_iter().enumerate() {
            if ix == 0 {
                path.move_to(point.into());
                continue;
            }
            path.line_to(point.into());
        }
        VShape { path, color, shape_type }
    }

    pub fn from_mapped_iter<T, U: Into<Vector2F>>(
        shape_type: ShapeType,
        color: ColorU,
        points: impl IntoIterator<Item = T>,
        f: impl Fn(T) -> U,
    ) -> VShape {
        let mut path = Path2D::new();
        for (ix, point) in points.into_iter().map(f).enumerate() {
            if ix == 0 {
                path.move_to(point.into());
                continue;
            }
            path.line_to(point.into());
        }
        VShape { path, color, shape_type }
    }
}
