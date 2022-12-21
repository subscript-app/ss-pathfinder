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

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// RENDERERING
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

/// The default is `RenderingMode::Compute`
pub enum RenderingMode {
    /// A GPU compute-based renderer.
    Compute,
    /// A hybrid CPU-GPU renderer.
    Raster,
}

impl Default for RenderingMode {
    fn default() -> Self {
        RenderingMode::Compute
    }
}

#[repr(C)]
pub struct DynDrawCmd<'a, const N: usize> {
    pub resolution: ViewResolution,
    pub metal_device: &'a NativeMetalDeviceRef,
    pub ca_drawables: Vec<&'a CoreAnimationDrawableRef>,
}

#[repr(C)]
pub struct StaticDrawCmd<'a, const N: usize> {
    pub resolution: ViewResolution,
    pub metal_device: &'a NativeMetalDeviceRef,
    pub ca_drawables: [&'a CoreAnimationDrawableRef; N],
}
