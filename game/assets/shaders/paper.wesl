// Le papier : the board filmed as a photograph of a model on the cartographer's table. A still grain of laid
// paper over the whole frame, and the corners sinking into a warm sepia, as an old print's do.

import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;

struct Paper {
    // How far the corners sink toward sepia, 0 for none.
    vignette: f32,
    // How strong the paper's grain is, 0 for none.
    grain: f32,
    _padding: vec2<f32>,
}

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var screen_sampler: sampler;
@group(0) @binding(2) var<uniform> paper: Paper;

fn hash(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453);
}

fn noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);
    return mix(mix(hash(i), hash(i + vec2(1.0, 0.0)), u.x), mix(hash(i + vec2(0.0, 1.0)), hash(i + vec2(1.0, 1.0)), u.x), u.y);
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    var color = textureSample(screen_texture, screen_sampler, in.uv);
    // Fixed to the screen like the sheet under the lens: it never crawls as the camera moves. Fibres run along
    // the sheet (stretched in x), with a finer tooth over them.
    let px = in.position.xy;
    let fibres = noise(px * vec2(0.05, 0.4));
    let tooth = noise(px * 0.7);
    let grain = (fibres * 0.6 + tooth * 0.4 - 0.5) * paper.grain;
    // Corners: measured on the frame's shape, so a wide window darkens its sides, not a circle in the middle.
    let size = vec2<f32>(textureDimensions(screen_texture));
    let from_middle = (in.uv - 0.5) * vec2(size.x / size.y, 1.0);
    let corner = smoothstep(0.3, 1.0, length(from_middle)) * paper.vignette;
    // Warm ochre, not grey: the corners of a hand-coloured sheet yellow, they don't dull.
    let sepia = vec3(0.9, 0.75, 0.52);
    color = vec4(color.rgb * (1.0 + grain) * mix(vec3(1.0), sepia, corner), color.a);
    return color;
}
