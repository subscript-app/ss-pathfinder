use ss_pathfinder_canvas::{Canvas, CanvasFontContext, Path2D};
use ss_pathfinder_color::{ColorF, ColorU};
use ss_pathfinder_geometry::vector::{Vector2F, Vector2I, Vector4F};
use crate::{data::basics::{ColorScheme, DualColors}, ViewInfo};

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// DATA TYPES
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

#[derive(Debug)]
pub struct VScene {
    polygons: Vec<VShape>,
    needs_redraw: bool,
    scene_type: SceneType,
}

#[derive(Debug, Clone)]
pub struct VShape {
    pub(crate) path: Path2D,
    pub(crate) color: DualColors,
    pub(crate) shape_type: ShapeType,
}

#[derive(Debug, Clone)]
pub enum ShapeType {
    Fill,
    Stroke { line_width: f32 },
}

#[derive(Debug)]
pub enum SceneType {Hot, Cold}

impl SceneType {
    pub fn is_hot(&self) -> bool {
        match self {SceneType::Hot => true, _ => false}
    }
    pub fn is_cold(&self) -> bool {
        match self {SceneType::Cold => true, _ => false}
    }
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// API
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――





//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// API
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

impl Default for VScene {
    fn default() -> Self {
        VScene { polygons: Default::default(), needs_redraw: true, scene_type: SceneType::Hot }
    }
}

impl VScene {
    pub fn set_scene_type(&mut self, scene_type: SceneType) {
        self.scene_type = scene_type;
    }
    pub fn clear(&mut self) {
        self.polygons.clear();
        self.set_needs_redraw(true);
    }
    pub fn add_shape<S: Into<VShape>>(&mut self, polygon: S) {
        self.polygons.push(polygon.into());
        self.set_needs_redraw(true);
    }
    pub fn shapes(&self) -> &[VShape] { &self.polygons }
    pub(crate) fn needs_redraw(&self) -> bool {
        if self.scene_type.is_hot() {
            return true
        }
        self.needs_redraw
    }
    pub fn set_needs_redraw(&mut self, state: bool) {
        self.needs_redraw = state;
    }
}

impl VShape {
    pub fn from_iter<T: Into<Vector2F>>(
        shape_type: ShapeType,
        color: impl Into<DualColors>,
        points: impl IntoIterator<Item = T>,
    ) -> VShape {
        let mut path = Path2D::new();
        for (ix, point) in points.into_iter().enumerate() {
            if ix == 0 {
                path.move_to(point.into());
                continue;
            }
            path.line_to(point.into());
        }
        VShape {
            path,
            color: color.into(),
            shape_type,
        }
    }

    pub fn from_iter_map<T, U: Into<Vector2F>>(
        shape_type: ShapeType,
        color: impl Into<DualColors>,
        points: impl IntoIterator<Item = T>,
        f: impl Fn(T) -> U,
    ) -> VShape {
        let color: DualColors = color.into();
        let mut path = Path2D::new();
        for (ix, point) in points.into_iter().map(f).enumerate() {
            if ix == 0 {
                path.move_to(point.into());
                continue;
            }
            path.line_to(point.into());
        }
        VShape {
            path,
            color,
            shape_type,
        }
    }
}

