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
use ss_pathfinder_renderer::options::RenderTransform;
use ss_pathfinder_renderer::scene::Scene;

use ss_pathfinder_resources::embedded::EmbeddedResourceLoader;
use ss_pathfinder_resources::ResourceLoader;
use ss_pathfinder_resources::fs::FilesystemResourceLoader;

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
use crate::data::basics::{ViewResolution, RenderingMode, StaticDrawCmd, DynDrawCmd};
use crate::renderers::scene::{VShape, VScene, ShapeType};
use crate::renderers::layer::VLayer;


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// DATA TYPES
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――


/// The const generic arguments should satisfy the following rules:
/// - `∀ SN. 4 >= SN >= 1`
/// - `SN == LN` or `LN == 1`
///
/// Where,
/// - `4 >= SN >= 1 && LN == 1`:
///     * Render all scenes with only a single rendering context. 
/// - `4 >= SN >= 1 && SN == LN`:
///     * Multiple layers with their own rendering context.
pub struct Vectorizer<const SN: usize = 1, const LN: usize = 1> {
    scenes: [VScene; SN],
    layers: [VLayer; LN],
    /// Use GPU compute-based renderer (default) or a hybrid CPU-GPU renderer.
    rendering_mode: RenderingMode,
}

pub struct VectorizerRef<'a, 'b, const SN: usize = 1, const LN: usize = 1> {
    pub scenes: &'a [VScene; SN],
    pub(crate) layers: &'b [VLayer; LN],
}

pub struct VectorizerMut<'a, 'b, const SN: usize = 1, const LN: usize = 1> {
    pub scenes: &'a mut [VScene; SN],
    pub(crate) layers: &'b mut [VLayer; LN],
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// MISCELLANEOUS
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

impl<const SN: usize, const LN: usize> Vectorizer<SN, LN> {
    pub(crate) fn head_layer(&self) -> &VLayer { &self.layers[0] }
    pub(crate) fn head_layer_mut(&mut self) -> &mut VLayer { &mut self.layers[0] }
    pub(crate) fn get_layer(&self, ix: usize) -> Option<&VLayer> { self.layers.get(ix) }

    pub fn get_scene(&self, ix: usize) -> Option<&VScene> { self.scenes.get(ix) }
    pub const fn scenes(&self) -> &[VScene; SN] { &self.scenes }

    pub fn scenes_mut(&mut self) -> &mut [VScene; SN] { &mut self.scenes }

    pub fn as_vectorizer_ref(&self) -> VectorizerRef<'_, '_, SN, LN> {
        VectorizerRef { scenes: &self.scenes, layers: &self.layers }
    }
    pub fn as_vectorizer_mut(&mut self) -> VectorizerMut<'_, '_, SN, LN> {
        VectorizerMut { scenes: &mut self.scenes, layers: &mut self.layers }
    }
}

impl<const N: usize> StaticDrawCmd<'_, N> {
    pub const fn head_drawable(&self) -> &CoreAnimationDrawableRef {self.ca_drawables[0]}
}

impl<const LN: usize> Vectorizer<1, LN> {
    pub const fn scene(&self) -> &VScene { &self.scenes[0] }
}
impl<const LN: usize> Vectorizer<2, LN> {
    pub const fn scene1(&self) -> &VScene { &self.scenes[0] }
    pub const fn scene2(&self) -> &VScene { &self.scenes[1] }
}
impl<const LN: usize> Vectorizer<3, LN> {
    pub const fn scene1(&self) -> &VScene { &self.scenes[0] }
    pub const fn scene2(&self) -> &VScene { &self.scenes[1] }
    pub const fn scene3(&self) -> &VScene { &self.scenes[2] }
}
impl<const LN: usize> Vectorizer<4, LN> {
    pub const fn scene1(&self) -> &VScene { &self.scenes[0] }
    pub const fn scene2(&self) -> &VScene { &self.scenes[1] }
    pub const fn scene3(&self) -> &VScene { &self.scenes[2] }
    pub const fn scene4(&self) -> &VScene { &self.scenes[3] }
}

impl<const LN: usize> Vectorizer<1, LN> {
    pub fn scene_mut(&mut self) -> &mut VScene { &mut self.scenes[0] }
}
impl<const LN: usize> Vectorizer<2, LN> {
    pub fn scene1_mut(&mut self) -> &mut VScene { &mut self.scenes[0] }
    pub fn scene2_mut(&mut self) -> &mut VScene { &mut self.scenes[1] }
}
impl<const LN: usize> Vectorizer<3, LN> {
    pub fn scene1_mut(&mut self) -> &mut VScene { &mut self.scenes[0] }
    pub fn scene2_mut(&mut self) -> &mut VScene { &mut self.scenes[1] }
    pub fn scene3_mut(&mut self) -> &mut VScene { &mut self.scenes[2] }
}
impl<const LN: usize> Vectorizer<4, LN> {
    pub fn scene1_mut(&mut self) -> &mut VScene { &mut self.scenes[0] }
    pub fn scene2_mut(&mut self) -> &mut VScene { &mut self.scenes[1] }
    pub fn scene3_mut(&mut self) -> &mut VScene { &mut self.scenes[2] }
    pub fn scene4_mut(&mut self) -> &mut VScene { &mut self.scenes[3] }
}


impl<const SN: usize> Vectorizer<SN, 1> {
    pub(crate) const fn layer(&self) -> &VLayer { &self.layers[0] }
    pub const fn is_single_instance() -> bool { true }
    pub const fn is_multi_instance() -> bool { false }
}
impl<const SN: usize> Vectorizer<SN, 2> {
    pub(crate) const fn layer1(&self) -> &VLayer { &self.layers[0] }
    pub(crate) const fn layer2(&self) -> &VLayer { &self.layers[1] }
    pub const fn is_single_instance() -> bool { false }
    pub const fn is_multi_instance() -> bool { true }
}
impl<const SN: usize> Vectorizer<SN, 3> {
    pub(crate) const fn layer1(&self) -> &VLayer { &self.layers[0] }
    pub(crate) const fn layer2(&self) -> &VLayer { &self.layers[1] }
    pub(crate) const fn layer3(&self) -> &VLayer { &self.layers[2] }
    pub const fn is_single_instance() -> bool { false }
    pub const fn is_multi_instance() -> bool { true }
}
impl<const SN: usize> Vectorizer<SN, 4> {
    pub(crate) const fn layer1(&self) -> &VLayer { &self.layers[0] }
    pub(crate) const fn layer2(&self) -> &VLayer { &self.layers[1] }
    pub(crate) const fn layer3(&self) -> &VLayer { &self.layers[2] }
    pub(crate) const fn layer4(&self) -> &VLayer { &self.layers[3] }
    pub const fn is_single_instance() -> bool { false }
    pub const fn is_multi_instance() -> bool { true }
}

impl<const SN: usize> Vectorizer<SN, 1> {
    pub(crate) fn layer_mut(&mut self) -> &mut VLayer { &mut self.layers[0] }
}
impl<const SN: usize> Vectorizer<SN, 2> {
    pub(crate) fn layer1_mut(&mut self) -> &mut VLayer { &mut self.layers[0] }
    pub(crate) fn layer2_mut(&mut self) -> &mut VLayer { &mut self.layers[1] }
}
impl<const SN: usize> Vectorizer<SN, 3> {
    pub(crate) fn layer1_mut(&mut self) -> &mut VLayer { &mut self.layers[0] }
    pub(crate) fn layer2_mut(&mut self) -> &mut VLayer { &mut self.layers[1] }
    pub(crate) fn layer3_mut(&mut self) -> &mut VLayer { &mut self.layers[2] }
}
impl<const SN: usize> Vectorizer<SN, 4> {
    pub(crate) fn layer1_mut(&mut self) -> &mut VLayer { &mut self.layers[0] }
    pub(crate) fn layer2_mut(&mut self) -> &mut VLayer { &mut self.layers[1] }
    pub(crate) fn layer3_mut(&mut self) -> &mut VLayer { &mut self.layers[2] }
    pub(crate) fn layer4_mut(&mut self) -> &mut VLayer { &mut self.layers[3] }
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// INIT - SINGLE INSTANCES
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

impl Default for Vectorizer<1, 1> {
    fn default() -> Self {
        Vectorizer {
            scenes: [VScene::default()],
            layers: [VLayer::default()],
            rendering_mode: Default::default(),
        }
    }
}

impl Default for Vectorizer<2, 1> {
    fn default() -> Self {
        Vectorizer {
            scenes: [VScene::default(), VScene::default()],
            layers: [VLayer::default()],
            rendering_mode: Default::default(),
        }
    }
}
impl Default for Vectorizer<3, 1> {
    fn default() -> Self {
        Vectorizer {
            scenes: [VScene::default(), VScene::default(), VScene::default()],
            layers: [VLayer::default()],
            rendering_mode: Default::default(),
        }
    }
}
impl Default for Vectorizer<4, 1> {
    fn default() -> Self {
        Vectorizer {
            scenes: [VScene::default(), VScene::default(), VScene::default(), VScene::default()],
            layers: [VLayer::default()],
            rendering_mode: Default::default(),
        }
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// INIT - MULTI INSTANCES
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

impl Default for Vectorizer<2, 2> {
    fn default() -> Self {
        Vectorizer {
            scenes: [VScene::default(), VScene::default()],
            layers: [VLayer::default(), VLayer::default()],
            rendering_mode: Default::default(),
        }
    }
}
impl Default for Vectorizer<3, 3> {
    fn default() -> Self {
        Vectorizer {
            scenes: [VScene::default(), VScene::default(), VScene::default()],
            layers: [VLayer::default(), VLayer::default(), VLayer::default()],
            rendering_mode: Default::default(),
        }
    }
}
impl Default for Vectorizer<4, 4> {
    fn default() -> Self {
        Vectorizer {
            scenes: [VScene::default(), VScene::default(), VScene::default(), VScene::default()],
            layers: [VLayer::default(), VLayer::default(), VLayer::default(), VLayer::default()],
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


impl Vectorizer<1, 1> {
    pub fn draw(&mut self, draw_cmd: StaticDrawCmd<'_, 1>) { self.draw_impl(draw_cmd) }
}
impl Vectorizer<2, 1> {
    pub fn draw(&mut self, draw_cmd: StaticDrawCmd<'_, 1>) { self.draw_impl(draw_cmd) }
}
impl Vectorizer<3, 1> {
    pub fn draw(&mut self, draw_cmd: StaticDrawCmd<'_, 1>) { self.draw_impl(draw_cmd) }
}
impl Vectorizer<4, 1> {
    pub fn draw(&mut self, draw_cmd: StaticDrawCmd<'_, 1>) { self.draw_impl(draw_cmd) }
}

impl Vectorizer<2, 2> {
    pub fn draw(&mut self, draw_cmd: StaticDrawCmd<'_, 2>) { self.draw_impl(draw_cmd) }
}
impl Vectorizer<3, 3> {
    pub fn draw(&mut self, draw_cmd: StaticDrawCmd<'_, 3>) { self.draw_impl(draw_cmd) }
}
impl Vectorizer<4, 4> {
    pub fn draw(&mut self, draw_cmd: StaticDrawCmd<'_, 4>) { self.draw_impl(draw_cmd) }
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// DRAW - INTERNAL API
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

impl<const SN: usize, const LN: usize> Vectorizer<SN, LN> {
    pub(crate) fn draw_impl(&mut self, draw_cmd: StaticDrawCmd<LN>) {
        println!("DRAW IMPL");
        let is_valid = {
            let opt1 = SN >= 1 && LN == 1;
            let opt2 = SN > 1 && SN == LN && LN > 1;
            opt1 | opt2
        };
        assert!(is_valid);
        if SN >= 1 && LN == 1 {
            return unsafe {
                self.draw_impl_single(draw_cmd)
            }
        }
        if SN > 1 && SN == LN && LN > 1 {
            return unsafe {
                self.draw_impl_multi_instance(draw_cmd)
            }
        }
        panic!("Impossible!")
    }
}
impl<const SN: usize, const LN: usize> Vectorizer<SN, LN> {
    pub(crate) unsafe fn draw_impl_single(&mut self, draw_cmd: StaticDrawCmd<LN>) {
        assert!(LN == 1);
        let scenes = &mut self.scenes;
        let layer = &mut self.layers[0];
        let drawable = draw_cmd.head_drawable();
        layer.draw_scenes(draw_cmd.resolution, draw_cmd.metal_device, drawable, scenes);
    }
}

impl<const SN: usize, const LN: usize> Vectorizer<SN, LN> {
    pub(crate) unsafe fn draw_impl_multi_instance(&mut self, draw_cmd: StaticDrawCmd<LN>) {
        assert!(LN > 1);
        assert!(SN == LN);
        let iter = self.layers
            .iter_mut()
            .zip(draw_cmd.ca_drawables.iter())
            .zip(self.scenes.iter_mut())
            .map(|((x, y), z)| (x, *y, z));
        for (layer, drawable, scene) in iter {
            layer.draw_scene(draw_cmd.resolution, draw_cmd.metal_device, drawable, scene);
        }
    }
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// INTERNAL HELPERS
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――


