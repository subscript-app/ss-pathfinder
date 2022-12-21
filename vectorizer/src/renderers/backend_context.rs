//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// INTERNAL IMPORTS
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
use foreign_types::ForeignTypeRef;
use io_surface::IOSurfaceRef;

use metal::{CAMetalLayer, CoreAnimationLayer, CoreAnimationLayerRef};
use metal::{CoreAnimationDrawableRef, DeviceRef as NativeMetalDeviceRef};

use ss_pathfinder_canvas::{Canvas, CanvasFontContext, CompositeOperation, Path2D};
use ss_pathfinder_canvas::{CanvasRenderingContext2D, FillStyle, LineJoin};

use ss_pathfinder_color::ColorF;
use ss_pathfinder_color::ColorU;

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
use ss_pathfinder_renderer::options::RenderTransform;
use ss_pathfinder_renderer::scene::Scene;

use ss_pathfinder_resources::embedded::EmbeddedResourceLoader;
use ss_pathfinder_resources::fs::FilesystemResourceLoader;
use ss_pathfinder_resources::ResourceLoader;

use ss_pathfinder_metal::IntoMetalDevice;
use ss_pathfinder_metal::MetalDevice;

use ss_pathfinder_content::fill::FillRule;
use ss_pathfinder_content::outline::ArcDirection;
use ss_pathfinder_content::stroke::LineCap;
use ss_pathfinder_gpu::Device;
use ss_pathfinder_simd::default::F32x4;

use super::scene::{ShapeType, VScene, VShape};
use crate::data::basics::ViewResolution;
use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::path::PathBuf;
use std::ptr;
use std::slice;
use std::str;

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// DATA TYPES
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// INIT
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

pub(crate) struct BackendContext {
    pub renderer: Renderer<MetalDevice>,
    pub build_options: BuildOptions,
    pub font_context: CanvasFontContext,
}

impl BackendContext {
    pub fn new(
        window_size: Vector2I,
        metal_device: &NativeMetalDeviceRef,
        ca_drawable: &CoreAnimationDrawableRef,
    ) -> Self {
        let device = unsafe { MetalDevice::new(metal_device, ca_drawable) };
        let resource_loader = unsafe { EmbeddedResourceLoader::new() };
        let dest_framebuffer = DestFramebuffer::full_window(window_size);
        // let level = RendererLevel::default_for_device(&device);
        // let render_mode = RendererMode { level: RendererLevel::D3D9 };
        let render_mode = RendererMode {
            level: RendererLevel::D3D11,
        };
        let render_options = RendererOptions {
            dest: dest_framebuffer,
            background_color: Some(ColorF::new(1.0, 1.0, 1.0, 0.0)),
            show_debug_ui: false,
        };
        let renderer = Renderer::new(device, &EmbeddedResourceLoader, render_mode, render_options);
        let build_options = BuildOptions::default();
        let font_context = CanvasFontContext::from_system_source();
        Self {
            renderer,
            build_options,
            font_context,
        }
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// DRAW
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

impl BackendContext {
    pub fn draw_shapes(
        &mut self,
        canvas_size: Vector2F,
        current_drawable: &CoreAnimationDrawableRef,
        shapes: &[VShape],
    ) {
        let mut canvas = Canvas::new(canvas_size).get_context_2d(self.font_context.clone());
        // canvas.set_global_composite_operation(CompositeOperation::SourceOut);
        // canvas.set_global_alpha(0.0);

        for VShape {
            shape_type,
            path,
            color,
        } in shapes.to_vec().into_iter()
        {
            match shape_type {
                ShapeType::Fill => {
                    canvas.set_fill_style(color);
                    canvas.fill_path(path, Default::default());
                }
                ShapeType::Stroke { line_width } => {
                    canvas.set_stroke_style(color);
                    canvas.set_line_width(line_width);
                    canvas.stroke_path(path);
                }
            }
        }

        let scene = canvas.into_canvas().into_scene();

        let mut scene_proxy =
            SceneProxy::from_scene(scene, self.renderer.mode().level, RayonExecutor);
        scene_proxy.build_and_render(&mut self.renderer, self.build_options.clone());
        let metal_device = self.renderer.device_mut();
        metal_device.present_drawable(current_drawable);
    }
}

impl BackendContext {
    pub fn draw_scenes(
        &mut self,
        canvas_size: Vector2F,
        current_drawable: &CoreAnimationDrawableRef,
        scenes: &[VScene],
    ) {
        for scene in scenes.iter() {
            self.draw_shapes(canvas_size, current_drawable, &scene.polygons);
        }
    }
}
