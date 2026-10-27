//! `InkMaterial`, the surface drawn as an engraving, and the makers of its plain, varnished and water versions.

use bevy::asset::RenderAssetUsages;
use bevy::image::ImageSampler;
use bevy::pbr::{ExtendedMaterial, MaterialExtension};
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, Extent3d, ShaderType, TextureDimension, TextureFormat};
use bevy::shader::ShaderRef;

use crate::theme::{SHEET, WORLD_INK};

pub type InkMaterial = ExtendedMaterial<StandardMaterial, Hachure>;

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct Hachure {
    #[uniform(100)]
    pub uniform: HachureUniform,
    /// On water: how far each point lies from the shore (`shore_image`), which the engraver's water lines follow.
    #[texture(101)]
    #[sampler(102)]
    pub shore: Option<Handle<Image>>,
}

#[derive(ShaderType, Reflect, Debug, Clone)]
pub struct HachureUniform {
    pub paper: LinearRgba,
    pub ink: LinearRgba,
    /// World units between strokes, fade toward the paper, lit value of white in full light, shade where strokes start.
    pub params: Vec4,
    /// Where `Hachure::shore` lies on the map: its corner's x and z, then 1 / its width and 1 / its depth. Zero
    /// without water lines.
    pub shore: Vec4,
    /// How much a varnished surface (the model's water, its waxed wood) shows its glints, 0 for matte plaster.
    pub gloss: f32,
    /// How much the water moves: 1 for a river or a lake, 0 for ice.
    pub flow: f32,
}

impl Default for Hachure {
    fn default() -> Self {
        Hachure {
            uniform: HachureUniform {
                paper: SHEET.to_linear().with_alpha(0.0),
                ink: WORLD_INK.to_linear().with_alpha(0.85),
                params: Vec4::new(0.13, 0.35, LIT_WHITE, 0.5),
                shore: Vec4::ZERO,
                gloss: 0.0,
                flow: 0.0,
            },
            shore: None,
        }
    }
}

impl Hachure {
    /// Finer, lighter strokes for small models (towers, props): coarse ones would stripe them.
    pub fn fine() -> Self {
        let mut hachure = Hachure::default();
        hachure.uniform.ink.alpha = 0.55;
        hachure.uniform.params.x = 0.05;
        hachure.uniform.params.y = 0.45;
        hachure
    }
}

/// What a white surface lights up to under the sun and the sky, in the shader's units (tuned on screenshots).
const LIT_WHITE: f32 = 1.0;

impl MaterialExtension for Hachure {
    fn fragment_shader() -> ShaderRef {
        "shaders/hachure.wesl".into()
    }
}

/// Ink from a plain colour.
pub fn ink(color: Color) -> InkMaterial {
    ExtendedMaterial {
        base: StandardMaterial { base_color: color, perceptual_roughness: 1.0, ..default() },
        extension: default(),
    }
}

/// Ink on a varnished surface: the plan-relief's resin water and waxed wood catch the sun in glints (drawn by the
/// shader; to Bevy the surface stays matte).
pub fn varnished(color: Color, gloss: f32) -> InkMaterial {
    let mut material = ink(color);
    material.extension.uniform.gloss = gloss;
    material
}

/// Water lined like an old map's: the varnished resin, with lines along the shore drawn from `shore` (made by
/// `shore_image`, laid over `rect`), drifting as much as `flow` says.
pub fn water(color: Color, shore: Handle<Image>, rect: Vec4, flow: f32) -> InkMaterial {
    let mut material = varnished(color, 0.8);
    material.extension.uniform.shore = rect;
    material.extension.uniform.flow = flow;
    material.extension.shore = Some(shore);
    material
}

/// The farthest from the shore `shore_image` tells apart, in world units: beyond, it is all open water.
const SHORE_REACH: f32 = 8.0;

/// How far from the shore each point of the map rectangle `min`..`min + size` lies, at `per_unit` texels a world
/// unit, for `water`: land where `wet` is false. Beyond the rectangle the water goes on, as off the edge of a sheet: a
/// river leaving it is not closed by a bank. Returns the image and its rect.
pub fn shore_image(min: Vec2, size: Vec2, per_unit: f32, wet: impl Fn(Vec2) -> bool) -> (Image, Vec4) {
    let (w, h) = ((size.x * per_unit).ceil() as usize, (size.y * per_unit).ceil() as usize);
    let at = |x: usize, y: usize| min + (Vec2::new(x as f32, y as f32) + 0.5) / per_unit;
    // Two-pass chamfer distance, in texels, from the nearest dry texel.
    let mut d: Vec<f32> = (0..w * h).map(|i| if wet(at(i % w, i / w)) { f32::MAX } else { 0.0 }).collect();
    let diagonal = std::f32::consts::SQRT_2;
    for (pass, steps) in [
        (0, [(-1, 0, 1.0), (0, -1, 1.0), (-1, -1, diagonal), (1, -1, diagonal)]),
        (1, [(1, 0, 1.0), (0, 1, 1.0), (1, 1, diagonal), (-1, 1, diagonal)]),
    ] {
        for k in 0..w * h {
            let i = if pass == 0 { k } else { w * h - 1 - k };
            let (x, y) = (i % w, i / w);
            let mut best = d[i];
            for (dx, dy, cost) in steps {
                let (nx, ny) = (x as i64 + dx, y as i64 + dy);
                if nx >= 0 && ny >= 0 && (nx as usize) < w && (ny as usize) < h {
                    best = best.min(d[ny as usize * w + nx as usize] + cost);
                }
            }
            d[i] = best;
        }
    }
    // No bank at all: open water everywhere (and no overflow in the blur).
    for t in &mut d {
        *t = t.min(SHORE_REACH * per_unit);
    }
    // Blurred a little, so the lines round the corners as an engraver's hand would.
    let radius = (0.4 * per_unit).round().max(1.0) as usize;
    for _ in 0..2 {
        for (step, len) in [(1, w), (w, h)] {
            let blurred: Vec<f32> = (0..w * h)
                .map(|i| {
                    let at = if step == 1 { i % w } else { i / w };
                    let (lo, hi) = (at.saturating_sub(radius), (at + radius).min(len - 1));
                    (lo..=hi).map(|k| d[i - at * step + k * step]).sum::<f32>() / (hi - lo + 1) as f32
                })
                .collect();
            d = blurred;
        }
    }
    // Stored as a square root: finest by the bank, where the lines are closest.
    let data = d.iter().map(|t| ((t / per_unit / SHORE_REACH).min(1.0).sqrt() * 255.0) as u8).collect();
    let mut image = Image::new(
        Extent3d { width: w as u32, height: h as u32, depth_or_array_layers: 1 },
        TextureDimension::D2,
        data,
        TextureFormat::R8Unorm,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.sampler = ImageSampler::linear();
    (image, Vec4::new(min.x, min.y, per_unit / w as f32, per_unit / h as f32))
}
