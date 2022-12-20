//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// INTERNAL IMPORTS
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
use io_surface::IOSurfaceRef;
use foreign_types::ForeignTypeRef;

use metal::{CoreAnimationDrawableRef, DeviceRef as NativeMetalDeviceRef};
use metal::{CAMetalLayer, CoreAnimationLayerRef, CoreAnimationLayer};

use ss_pathfinder_canvas::{Canvas, CanvasFontContext, Path2D};
use ss_pathfinder_canvas::{CanvasRenderingContext2D, FillStyle, LineJoin};

use ss_pathfinder_color::ColorF;
use ss_pathfinder_color::ColorU;

use ss_pathfinder_geometry::vector::{vec2f, vec2i};
use ss_pathfinder_geometry::vector::Vector2I;
use ss_pathfinder_geometry::vector::{Vector2F, Vector4F};
use ss_pathfinder_geometry::rect::{RectF, RectI};
use ss_pathfinder_geometry::transform2d::Transform2F;
use ss_pathfinder_geometry::transform3d::Transform4F;

use ss_pathfinder_renderer::concurrent::rayon::RayonExecutor;
use ss_pathfinder_renderer::concurrent::scene_proxy::SceneProxy;
use ss_pathfinder_renderer::options::BuildOptions;
use ss_pathfinder_renderer::gpu::options::{DestFramebuffer, RendererMode, RendererOptions};
use ss_pathfinder_renderer::gpu::renderer::Renderer;
use ss_pathfinder_renderer::gpu::options::RendererLevel;

use ss_pathfinder_resources::embedded::EmbeddedResourceLoader;
use ss_pathfinder_resources::ResourceLoader;
use ss_pathfinder_resources::fs::FilesystemResourceLoader;

use ss_pathfinder_metal::IntoMetalDevice;
use ss_pathfinder_metal::MetalDevice;

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
use crate::data::basics::{ViewResolution, RenderingMode, StaticDrawCmd, DynDrawCmd};
use crate::renderers::scene::{VShape, VScene, ShapeType};
use crate::renderers::layer::VLayer;


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// DATA TYPES
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――


pub struct DynVectorizer<const SN: usize = 1> {
    scenes: [VScene; SN],
    layers: Vec<VLayer>,
    /// Use GPU compute-based renderer (default) or a hybrid CPU-GPU renderer.
    rendering_mode: RenderingMode,
}


pub struct DynVectorizerRef<'a, 'b, const SN: usize = 1, const LN: usize = 1> {
    pub scenes: &'a [VScene; SN],
    pub(crate) layers: &'b [VLayer; LN],
}

pub struct DynVectorizerMut<'a, 'b, const SN: usize = 1, const LN: usize = 1> {
    pub scenes: &'a mut [VScene; SN],
    pub(crate) layers: &'b mut [VLayer; LN],
}




//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// MISCELLANEOUS
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

impl<const SN: usize> DynVectorizer<SN> {
    pub fn get_scene(&self, ix: usize) -> Option<&VScene> { self.scenes.get(ix) }
    pub const fn scenes(&self) -> &[VScene; SN] { &self.scenes }

    pub fn scenes_mut(&mut self) -> &mut [VScene; SN] { &mut self.scenes }
}

impl DynVectorizer<1> {
    pub const fn scene(&self) -> &VScene { &self.scenes[0] }
}
impl DynVectorizer<2> {
    pub const fn scene1(&self) -> &VScene { &self.scenes[0] }
    pub const fn scene2(&self) -> &VScene { &self.scenes[1] }
}
impl DynVectorizer<3> {
    pub const fn scene1(&self) -> &VScene { &self.scenes[0] }
    pub const fn scene2(&self) -> &VScene { &self.scenes[1] }
    pub const fn scene3(&self) -> &VScene { &self.scenes[2] }
}
impl DynVectorizer<4> {
    pub const fn scene1(&self) -> &VScene { &self.scenes[0] }
    pub const fn scene2(&self) -> &VScene { &self.scenes[1] }
    pub const fn scene3(&self) -> &VScene { &self.scenes[2] }
    pub const fn scene4(&self) -> &VScene { &self.scenes[3] }
}






//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// INIT
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

impl Default for DynVectorizer<1> {
    fn default() -> Self {
        DynVectorizer {
            scenes: [VScene::default()],
            layers: Vec::default(),
            rendering_mode: Default::default(),
        }
    }
}

impl Default for DynVectorizer<2> {
    fn default() -> Self {
        DynVectorizer {
            scenes: [VScene::default(), VScene::default()],
            layers: Vec::default(),
            rendering_mode: Default::default(),
        }
    }
}
impl Default for DynVectorizer<3> {
    fn default() -> Self {
        DynVectorizer {
            scenes: [VScene::default(), VScene::default(), VScene::default()],
            layers: Vec::default(),
            rendering_mode: Default::default(),
        }
    }
}
impl Default for DynVectorizer<4> {
    fn default() -> Self {
        DynVectorizer {
            scenes: [VScene::default(), VScene::default(), VScene::default(), VScene::default()],
            layers: Vec::default(),
            rendering_mode: Default::default(),
        }
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// UPDATE
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――



//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// DRAW - PUBLIC API
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――


impl DynVectorizer<1> {
    pub fn draw(&mut self, draw_cmd: StaticDrawCmd<1>) {}
}
impl DynVectorizer<2> {
    pub fn draw(&mut self, draw_cmd: StaticDrawCmd<2>) {}
}
impl DynVectorizer<3> {
    pub fn draw(&mut self, draw_cmd: StaticDrawCmd<3>) {}
}
impl DynVectorizer<4> {
    pub fn draw(&mut self, draw_cmd: StaticDrawCmd<4>) {}
}



//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// DRAW - INTERNAL API
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

// impl<const SN: usize> Vectorizer<SN> {
//     pub(crate) fn draw_impl(&mut self, draw_cmd: DrawCmd<LN>) {
//         let is_valid = {
//             let opt1 = SN >= 1 && LN == 1;
//             let opt2 = SN > 1 && SN == LN && LN > 1;
//             opt1 | opt2
//         };
//         assert!(is_valid);
//         if SN >= 1 && LN == 1 {
//             return unsafe {
//                 self.draw_impl_single(draw_cmd)
//             }
//         }
//         if SN > 1 && SN == LN && LN > 1 {
//             return unsafe {
//                 self.draw_impl_multi_instance(draw_cmd)
//             }
//         }
//         panic!("Impossible!")
//     }
// }
// impl<const SN: usize> Vectorizer<SN> {
//     pub(crate) unsafe fn draw_impl_single(&mut self, draw_cmd: DrawCmd<LN>) {
//         assert!(LN == 1);
//         let scenes = &mut self.scenes;
//         let layer = &mut self.layers[0];
//         let drawable = draw_cmd.head_drawable();
//         layer.draw_scenes(draw_cmd.resolution, draw_cmd.metal_device, drawable, scenes);
//     }
// }

// impl<const SN: usize> Vectorizer<SN> {
//     pub(crate) unsafe fn draw_impl_multi_instance(&mut self, draw_cmd: DrawCmd<LN>) {
//         assert!(LN > 1);
//         assert!(SN == LN);
//         let iter = self.layers
//             .iter_mut()
//             .zip(draw_cmd.ca_drawables.iter())
//             .zip(self.scenes.iter_mut())
//             .map(|((x, y), z)| (x, *y, z));
//         for (layer, drawable, scene) in iter {
//             layer.draw_scene(draw_cmd.resolution, draw_cmd.metal_device, drawable, scene);
//         }
//     }
// }


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// INTERNAL HELPERS
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――


