//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// INTERNAL IMPORTS
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
use foreign_types::ForeignTypeRef;
use io_surface::IOSurfaceRef;

use metal::{CAMetalLayer, CoreAnimationLayer, CoreAnimationLayerRef};
use metal::{CoreAnimationDrawableRef, DeviceRef as NativeMetalDeviceRef};

use ss_pathfinder_canvas::{Canvas, CanvasFontContext, Path2D};
use ss_pathfinder_canvas::{CanvasRenderingContext2D, FillStyle, LineJoin};

use ss_pathfinder_color::ColorF;

use ss_pathfinder_geometry::rect::{RectF, RectI};
use ss_pathfinder_geometry::transform2d::Transform2F;
use ss_pathfinder_geometry::transform3d::Transform4F;
use ss_pathfinder_geometry::vector::Vector2I;
use ss_pathfinder_geometry::vector::{vec2f, vec2i};
use ss_pathfinder_geometry::vector::{Vector2F, Vector4F};

use ss_pathfinder_renderer::concurrent::rayon::RayonExecutor;
use ss_pathfinder_renderer::concurrent::scene_proxy::SceneProxy;
use ss_pathfinder_renderer::gpu::options::RendererLevel;
use ss_pathfinder_renderer::gpu::options::{DestFramebuffer, RendererMode, RendererOptions};
use ss_pathfinder_renderer::gpu::renderer::Renderer;
use ss_pathfinder_renderer::options::BuildOptions;

use ss_pathfinder_resources::embedded::EmbeddedResourceLoader;
use ss_pathfinder_resources::fs::FilesystemResourceLoader;
use ss_pathfinder_resources::ResourceLoader;

use ss_pathfinder_metal::IntoMetalDevice;
use ss_pathfinder_metal::MetalDevice;

use ss_pathfinder_color::ColorU;
use ss_pathfinder_content::fill::FillRule;
use ss_pathfinder_content::outline::ArcDirection;
use ss_pathfinder_content::stroke::LineCap;
use ss_pathfinder_gpu::Device;
// use ss_pathfinder_resources::ResourceLoader;
// use ss_pathfinder_resources::fs::FilesystemResourceLoader;
// use ss_pathfinder_resources::embedded::EmbeddedResourceLoader;
// use ss_pathfinder_renderer::concurrent::rayon::RayonExecutor;
// use ss_pathfinder_renderer::concurrent::scene_proxy::SceneProxy;
use ss_pathfinder_renderer::options::RenderTransform;
use ss_pathfinder_renderer::scene::Scene;
use ss_pathfinder_simd::default::F32x4;

use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::path::PathBuf;
use std::ptr;
use std::slice;
use std::str;

use super::backend_context::BackendContext;
use super::scene::{ShapeType, VScene, VShape};
use crate::data::basics::ViewResolution;

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// MISC
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// CONSTANTS
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// DATA TYPES
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

#[derive(Default)]
pub(crate) struct VLayer {
    context: Option<BackendContext>,
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// INIT
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// UPDATE
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// DRAW
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

impl VLayer {
    pub(crate) fn draw_scene(
        &mut self,
        resolution: ViewResolution,
        metal_device: &NativeMetalDeviceRef,
        ca_drawable: &CoreAnimationDrawableRef,
        scene: &mut VScene,
    ) {
        if let Some(context) = self.context.as_mut() {
            context.renderer.device_mut().swap_texture(ca_drawable);
            context.draw_shapes(resolution.as_vector2f(), ca_drawable, &scene.polygons);
        } else {
            let mut context =
                BackendContext::new(resolution.as_vector2i(), metal_device, ca_drawable);
            context.draw_shapes(resolution.as_vector2f(), ca_drawable, &scene.polygons);
            self.context = Some(context);
        }
    }
}

impl VLayer {
    pub(crate) fn draw_scenes(
        &mut self,
        resolution: ViewResolution,
        metal_device: &NativeMetalDeviceRef,
        ca_drawable: &CoreAnimationDrawableRef,
        scenes: &mut [VScene],
    ) {
        if let Some(context) = self.context.as_mut() {
            context.renderer.device_mut().swap_texture(ca_drawable);
            context.draw_scenes(resolution.as_vector2f(), ca_drawable, scenes);
        } else {
            let mut context =
                BackendContext::new(resolution.as_vector2i(), metal_device, ca_drawable);
            context.draw_scenes(resolution.as_vector2f(), ca_drawable, scenes);
            self.context = Some(context);
        }
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// UPDATE
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
