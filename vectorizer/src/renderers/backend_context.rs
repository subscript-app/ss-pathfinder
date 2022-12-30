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

use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::path::PathBuf;
use std::ptr;
use std::slice;
use std::str;


use super::scene::{ShapeType, VScene, VShape};
use crate::data::basics::{ColorScheme, ViewInfo};
use crate::data::basics::{ViewResolution, RenderingMode};

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
        rendering_mode: RenderingMode
    ) -> Self {
        let device = unsafe { MetalDevice::new(metal_device, ca_drawable) };
        let resource_loader = unsafe { EmbeddedResourceLoader::new() };
        let dest_framebuffer = DestFramebuffer::full_window(window_size);
        let renderer_level = match rendering_mode {
            RenderingMode::Compute => RendererLevel::D3D11,
            RenderingMode::Raster => RendererLevel::D3D9,
        };
        let render_mode = RendererMode { level: renderer_level };
        let render_options = RendererOptions {
            dest: dest_framebuffer,
            background_color: Some(ColorF::new(1.0, 1.0, 1.0, 0.0)),
            show_debug_ui: false,
        };
        let renderer = Renderer::new(device, &EmbeddedResourceLoader, render_mode, render_options);
        let build_options = BuildOptions{
            // dilation: Vector2F::new(2.0, 2.0),
            // subpixel_aa_enabled: true,
            ..BuildOptions::default()
        };
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
    pub fn draw_scene(
        &mut self,
        view_info: ViewInfo,
        current_drawable: &CoreAnimationDrawableRef,
        scene: &mut VScene,
    ) {
        // println!("BackendContext.draw_scene!");
        // if !scene.needs_redraw() { return }
        scene.set_needs_redraw(false);
        let mut canvas = Canvas::new(view_info.resolution.as_vector2f()).get_context_2d(self.font_context.clone());
        for VShape {shape_type, path, color} in scene.shapes() {
            let path = path.clone();
            match shape_type {
                ShapeType::Fill => {
                    canvas.set_fill_style(*color.get(view_info.color_scheme));
                    canvas.fill_path(path, Default::default());
                }
                ShapeType::Stroke { line_width } => {
                    canvas.set_stroke_style(*color.get(view_info.color_scheme));
                    canvas.set_line_width(*line_width);
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
        view_info: ViewInfo,
        current_drawable: &CoreAnimationDrawableRef,
        scenes: &mut [VScene],
    ) {
        for scene in scenes.iter_mut() {
            self.draw_scene(view_info, current_drawable, scene);
        }
    }
}
