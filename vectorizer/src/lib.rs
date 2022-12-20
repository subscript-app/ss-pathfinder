#![allow(unused)]
mod data;
mod renderers;

pub use data::basics::{ViewResolution, StaticDrawCmd, DynDrawCmd};
pub use renderers::scene::{VShape, VScene, ShapeType};
pub use renderers::static_core::{Vectorizer, VectorizerRef, VectorizerMut};

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// REEXPORTS
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――


pub use metal::{CAMetalLayer, CoreAnimationLayerRef, CoreAnimationLayer};
pub use metal::{CoreAnimationDrawableRef, DeviceRef as NativeMetalDeviceRef};

pub use ss_pathfinder_color::{ColorF, ColorU};

pub use ss_pathfinder_canvas::{Path2D, CanvasRenderingContext2D, FillStyle};

pub use ss_pathfinder_geometry::vector::{vec2f, vec2i};
pub use ss_pathfinder_geometry::rect::{RectF, RectI};
pub use ss_pathfinder_geometry::vector::{Vector2I, Vector2F, Vector4F};


