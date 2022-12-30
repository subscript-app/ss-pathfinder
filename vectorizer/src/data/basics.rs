use serde::{Serialize, Deserialize};

use metal::{CAMetalLayer, CoreAnimationLayer, CoreAnimationLayerRef};
use metal::{CoreAnimationDrawableRef, DeviceRef as NativeMetalDeviceRef};
use ss_pathfinder_canvas::{Canvas, CanvasFontContext, Path2D};
use ss_pathfinder_gpu::Device;
use ss_pathfinder_metal::MetalDevice;
use ss_pathfinder_renderer::concurrent::rayon::RayonExecutor;
use ss_pathfinder_renderer::concurrent::scene_proxy::SceneProxy;
use ss_pathfinder_renderer::gpu::renderer::Renderer;
use ss_pathfinder_renderer::options::BuildOptions;
use ss_pathfinder_geometry::vector::{Vector2F, Vector2I, Vector4F};
use ss_pathfinder_color::{ColorF, ColorU};

use super::*;

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// VIEW METADATA
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ViewResolution {
    pub width: f32,
    pub height: f32,
}

impl ViewResolution {
    pub(crate) fn as_vector2i(self) -> Vector2I {
        Vector2I::new(self.width as i32, self.height as i32)
    }
    pub(crate) fn as_vector2f(self) -> Vector2F {
        Vector2F::new(self.width, self.height)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum ColorScheme {
    Dark,
    Light,
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// VIEW INFO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ViewInfo {
    pub resolution: ViewResolution,
    pub scale_factor: f32,
    pub color_scheme: ColorScheme,
}

impl ViewInfo {
    pub fn unchanged(left: &ViewInfo, right: &ViewInfo) -> bool {
        let equal_width = left.resolution.width == right.resolution.width;
        let equal_height = left.resolution.height == right.resolution.height;
        let equal_scale_factor = left.scale_factor == right.scale_factor;
        let equal_color_scheme = left.color_scheme == right.color_scheme;

        equal_width &&
        equal_height &&
        equal_scale_factor &&
        equal_color_scheme
    }
}



//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// RENDERERING
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

/// The default is `RenderingMode::Compute`
#[derive(Debug, Clone, Copy)]
pub enum RenderingMode {
    /// A GPU compute-based renderer.
    Compute,
    /// A hybrid CPU-GPU renderer.
    Raster,
}

impl Default for RenderingMode {
    fn default() -> Self { RenderingMode::Raster }
}

#[repr(C)]
pub struct DynDrawCmd<'a, const N: usize> {
    pub view_info: ViewInfo,
    pub content_scale_factor: f32,
    pub metal_device: &'a NativeMetalDeviceRef,
    pub ca_drawables: Vec<&'a CoreAnimationDrawableRef>,
}

#[repr(C)]
pub struct StaticDrawCmd<'a, const N: usize> {
    pub view_info: ViewInfo,
    pub metal_device: &'a NativeMetalDeviceRef,
    pub ca_drawables: [&'a CoreAnimationDrawableRef; N],
}

impl<'a, const N: usize> StaticDrawCmd<'a, N> {
    pub fn is_valid(&self) -> bool {
        let valid_view_info = {
            let width = self.view_info.resolution.width;
            let height = self.view_info.resolution.height;
             width.is_finite() && height.is_finite() && width > 0.0 && height > 0.0
        };
        let valid_content_scale_factor = {
            self.view_info.scale_factor > 0.0
        };
        valid_view_info && valid_content_scale_factor
    }
}



#[derive(Debug, Clone)]
pub struct DualColors {
    pub light_mode: ColorU,
    pub dark_mode: ColorU,
}

impl DualColors {
    pub fn get(&self, color_scheme: ColorScheme) -> &ColorU {
        match color_scheme {
            ColorScheme::Dark => &self.dark_mode,
            ColorScheme::Light => &self.light_mode,
        }
    }
}

