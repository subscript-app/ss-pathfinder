#![allow(unused)]
mod data;
mod renderers;

pub use data::basics::{DynDrawCmd, StaticDrawCmd, ViewResolution, ColorScheme, ViewInfo, DualColors};
pub use renderers::scene::{ShapeType, VScene, VShape, SceneType};
pub use renderers::static_core::{Vectorizer, VectorizerMut, VectorizerRef};

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// REEXPORTS
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

pub use metal::{CAMetalLayer, CoreAnimationLayer, CoreAnimationLayerRef};
pub use metal::{CoreAnimationDrawableRef, DeviceRef as NativeMetalDeviceRef};

pub use ss_pathfinder_color::{ColorF, ColorU};

pub use ss_pathfinder_canvas::{CanvasRenderingContext2D, FillStyle, Path2D};

pub use ss_pathfinder_geometry::rect::{RectF, RectI};
pub use ss_pathfinder_geometry::vector::{vec2f, vec2i};
pub use ss_pathfinder_geometry::vector::{Vector2F, Vector2I, Vector4F};
