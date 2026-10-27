// L'encre : the world as an engraving. Surfaces keep their flat colour, faded toward the paper; where the light
// falls off (side faces, shadows) ink strokes come in, thicker as it gets darker, crossed in the deepest shade.
// On slopes the strokes run downhill, like the hachures of old maps. The light is read from Bevy's own PBR
// lighting, so shadows and every light in the scene count. Water is lined as on old maps: strokes along the shore,
// closer by the bank, fewer out in the open.

import bevy_pbr::render::{
    mesh_view_bindings::{view, lights, globals},
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{alpha_discard, apply_pbr_lighting, main_pass_post_lighting_processing},
    forward_io::{VertexOutput, FragmentOutput},
};

struct Hachure {
    // The sheet the world fades toward, linear rgb; alpha: how much steepness alone brings strokes (the land's
    // hachures), 0 for things that are not ground.
    paper: vec4<f32>,
    // Stroke colour, linear rgb; alpha is the strokes' strength.
    ink: vec4<f32>,
    // x: world units between strokes, y: how far colours fade toward the paper (0 to 1),
    // z: lit value of a white surface in full light, w: shade under which strokes start (0 to 1).
    params: vec4<f32>,
    // Where the shore texture lies on the map: its corner's x and z, then 1 / its width and 1 / its depth. Zero
    // for no water lines.
    shore: vec4<f32>,
    // How much a varnished surface shows its glints (light beyond what a matte one would return), 0 for matte.
    gloss: f32,
    // How much the water moves: 1 for a river or a lake, 0 for ice.
    flow: f32,
    // How far it leans in the wind (`wind.wesl`); unused here.
    wind: f32,
}

@group(constants::MATERIAL_BIND_GROUP) @binding(100)
var<uniform> hachure: Hachure;
// How far from the shore each point lies, over `SHORE_REACH`, square-rooted (`look::shore_image`).
@group(constants::MATERIAL_BIND_GROUP) @binding(101)
var shore_texture: texture_2d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(102)
var shore_sampler: sampler;

// As in `look.rs`.
const SHORE_REACH: f32 = 8.0;

fn luminance(c: vec3<f32>) -> f32 {
    return dot(c, vec3(0.2126, 0.7152, 0.0722));
}

// A lamp's colour as a tint around white, `amount` of it: the palette stays the shader's, the lamps only warm or
// cool it.
fn tint(c: vec3<f32>, amount: f32) -> vec3<f32> {
    return mix(vec3(1.0), c / max(luminance(c), 0.0001), amount);
}

fn hash(p: vec3<f32>) -> f32 {
    return fract(sin(dot(p, vec3(12.9898, 78.233, 37.719))) * 43758.5453);
}

// The wind, as in `wind.wesl`: the clouds drift with it.
const WIND: vec2<f32> = vec2(0.94, 0.34);

fn value_noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);
    let a = hash(vec3(i.x, 0.0, i.y));
    let b = hash(vec3(i.x + 1.0, 0.0, i.y));
    let c = hash(vec3(i.x, 0.0, i.y + 1.0));
    let d = hash(vec3(i.x + 1.0, 0.0, i.y + 1.0));
    return mix(mix(a, b, u.x), mix(c, d, u.x), u.y);
}

// The light left at `p` under the clouds drifting over the land: soft-edged shadows a dozen units across, passing
// in under a minute, never darker than two thirds of the sun, above where the strokes start, so no tile or marker
// loses its colour.
fn cloud(p: vec2<f32>) -> f32 {
    let q = p * 0.05 - WIND * globals.time * 0.06;
    let n = value_noise(q) * 0.65 + value_noise(q * 2.3 + 5.1) * 0.35;
    return 1.0 - 0.35 * smoothstep(0.5, 0.66, n);
}

// Coverage of parallel strokes `width` wide (0 to 1 of the spacing) across coordinate `u`, anti-aliased. Where
// they get finer than a pixel, they turn into their average tone instead of shimmering.
fn strokes(u: f32, width: f32) -> f32 {
    let w = max(fwidth(u), 0.0001);
    let line = clamp((width * 0.5 - abs(fract(u) - 0.5)) / w + 0.5, 0.0, 1.0);
    return mix(line, width, smoothstep(0.3, 0.6, w)) * step(0.001, width);
}

// Ink of strokes across `u`, `dark` deep, crossed along `u + v` in the deepest shade.
fn hatch(u: f32, v: f32, dark: f32) -> f32 {
    let first = strokes(u, clamp(dark * 0.9, 0.0, 0.65));
    let second = strokes((u + v) * 0.7071, clamp((dark - 0.55) * 1.4, 0.0, 0.5));
    return max(first, second);
}

// Ink of straight strokes running along `dir` (unit, on the map): the cuts of the land by upright planes, so they
// stay straight however the ground turns.
fn land_hatch(pos: vec3<f32>, dir: vec2<f32>, dark: f32) -> f32 {
    let tremor = 0.06 * sin(dot(pos.xz, dir) * 9.0 + sin(dot(pos.xz, vec2(1.7, 2.3))) * 3.0);
    let u = dot(pos.xz, vec2(-dir.y, dir.x)) / hachure.params.x + tremor;
    let v = (dot(pos.xz, dir) + pos.y) / hachure.params.x;
    return hatch(u, v, dark);
}

@fragment
fn fragment(in: VertexOutput, @builtin(front_facing) is_front: bool) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(in, is_front);
    pbr_input.material.base_color = alpha_discard(
        pbr_input.material.flags,
        pbr_input.material.alpha_cutoff,
        pbr_input.material.base_color,
    );
    let albedo = pbr_input.material.base_color.rgb;
    let lit = apply_pbr_lighting(pbr_input);

    // How lit this point is, from 0 (black) to 1 (full light), whatever its colour.
    // Where pieces meet the ground and in the folds (ambient occlusion, desktop only), the shade deepens beyond
    // what the ambient light alone loses: strokes gather there, as an engraver marks a contact.
    // Bevy's occlusion is faint (0.85 at a contact): squared, it reads.
    let occlusion = pow(luminance(pbr_input.diffuse_occlusion), 2.0);
    let pos = in.world_position.xyz;
    let shade =
        clamp(luminance(lit.rgb) / max(luminance(albedo), 0.02) / hachure.params.z, 0.0, 1.0) * occlusion * cloud(pos.xz);

    // Flat colour faded toward the paper, a little darker in the shade so faces still read apart.
    // Plaster grain, faded out where its specks get finer than a pixel (it would shimmer as the camera moves).
    let speck = 0.97 + 0.06 * hash(floor(pos * 24.0));
    let grain = mix(speck, 1.0, smoothstep(0.3, 1.0, length(fwidth(pos * 24.0))));
    var color = mix(albedo, hachure.paper.rgb * luminance(albedo) / luminance(hachure.paper.rgb), hachure.params.y);
    color = color * mix(0.72, 1.0, shade) * grain;
    // A model under a lamp: lit faces take the sun's warmth, the shade the sky's cool (`look::SUNLIGHT`).
    var sun = normalize(vec3(-4.0, 9.0, -3.0));
    var sunlight = vec3(1.0);
    if lights.n_directional_lights > 0u {
        sun = lights.directional_lights[0].direction_to_light;
        sunlight = tint(lights.directional_lights[0].color.rgb, 0.3);
    }
    color = color * mix(tint(lights.ambient_color.rgb, 0.3), sunlight, shade);
    let to_eye = normalize(view.world_position - pos);
    let n = normalize(in.world_normal);
    var water_ink = 0.0;
    // Varnish: glints of the sun on a surface rippled like poured resin, the ripples drifting on running water.
    // The surface itself stays matte to Bevy (its ambient would pale all of it); the glint is the sun's alone.
    if hachure.gloss > 0.0 {
        let t = globals.time * hachure.flow;
        let ripple = vec3(
            sin(pos.x * 11.0 + sin(pos.z * 7.0 + t * 0.7) * 2.0 + t * 0.9),
            0.0,
            sin(pos.z * 13.0 + pos.x * 3.1 - t * 1.1),
        ) * 0.07;
        let normal = normalize(n + ripple);
        let half_way = normalize(sun + to_eye);
        let glint = smoothstep(0.5, 0.95, pow(max(dot(normal, half_way), 0.0), 300.0)) * hachure.gloss;
        color = mix(color, vec3(1.0, 0.98, 0.93) * sunlight, glint * 0.75);
    }
    // Water lines, on the water's face only.
    if hachure.shore.z > 0.0 && n.y > 0.5 {
        let uv = (pos.xz - hachure.shore.xy) * hachure.shore.zw;
        let root = textureSampleLevel(shore_texture, shore_sampler, uv, 0.0).r;
        let d = root * root * SHORE_REACH;
        // Shallows by the bank are paler, the open water deeper.
        let open = smoothstep(0.0, hachure.params.x * 12.0, d);
        color = color * mix(1.08, 0.9, open);
        // Line k lies k² spacings out: tight by the bank, wider and fainter toward the middle. A slight tremor
        // of the hand, and on running water a slow swell, as if the lines were breathing.
        let t = globals.time * hachure.flow;
        let swell = 0.12 * sin(t * 0.6 + dot(pos.xz, vec2(0.7, 0.4))) * hachure.flow;
        let tremor = 0.05 * sin(dot(pos.xz, vec2(3.1, 2.3)) + sin(pos.x * 1.3) * 2.0);
        let u = root * sqrt(SHORE_REACH / (hachure.params.x * 0.25)) + tremor + swell - 0.35;
        // Gone before the open water, where the distance stops telling, and where shores meet in the middle (the
        // distance flattens there, and a line would swell).
        let slope = fwidth(root) / max(fwidth(pos.x) + fwidth(pos.z), 0.00001);
        let even = slope * 2.0 * sqrt(max(d, 0.01) * SHORE_REACH);
        let fade = (1.0 - smoothstep(4.0, 7.0, u)) * (1.0 - smoothstep(0.4, 0.7, root)) * smoothstep(0.25, 0.6, even)
            * step(0.0, u);
        water_ink = strokes(u, mix(0.14, 0.08, open)) * fade;
        // The sky in the resin, more as the eye grazes it.
        let fresnel = pow(1.0 - max(dot(n, to_eye), 0.0), 4.0);
        color = mix(color, hachure.paper.rgb * sunlight, fresnel * 0.6);
    }

    // Strokes run downhill on slopes and walls, along a fixed diagonal on flat ground.
    let steep = smoothstep(0.06, 0.4, 1.0 - n.y) * hachure.paper.a;
    let dark = max(clamp((hachure.params.w - shade) / hachure.params.w, 0.0, 1.0), steep);
    let lean = smoothstep(0.2, 0.35, length(n.xz));
    var ink = 0.0;
    if hachure.paper.a > 0.0 {
        // The land is smooth: strokes turning with it would shear into moiré far from the origin. So they come
        // from eight fixed directions, the two nearest downhill blended. Directions are mixed as doubled angles,
        // since a stroke has no way.
        let down = n.xz / max(length(n.xz), 0.0001);
        let doubled = mix(vec2(0.0, -1.0), vec2(down.x * down.x - down.y * down.y, 2.0 * down.x * down.y), lean);
        // An eighth of a half turn.
        let turn = 0.3926991;
        let f = 0.5 * atan2(doubled.y, doubled.x) / turn;
        let a = floor(f) * turn;
        let b = a + turn;
        let near = land_hatch(pos, vec2(cos(a), sin(a)), dark);
        let far = land_hatch(pos, vec2(cos(b), sin(b)), dark);
        ink = mix(near, far, smoothstep(0.25, 0.75, fract(f)));
    } else {
        let uphill = vec3(0.0, 1.0, 0.0) - n * n.y;
        let diagonal = normalize(vec3(1.0, 0.0, -1.0) - n * dot(vec3(1.0, 0.0, -1.0), n));
        let along = normalize(mix(diagonal, uphill / max(length(uphill), 0.0001), lean));
        let across = normalize(cross(n, along));
        // A slight hand tremor, smooth everywhere so no stroke breaks.
        let wobble = 0.06 * sin(dot(pos, along) * 9.0 + sin(dot(pos.xz, vec2(1.7, 2.3))) * 3.0);
        ink = hatch(dot(pos, across) / hachure.params.x + wobble, dot(pos, along) / hachure.params.x, dark);
    }
    // In the shade the ink goes cooler and a little lighter: a shadow is a wash with strokes, not a black hole.
    let shadow = 1.0 - shade;
    let stroke = mix(hachure.ink.rgb, hachure.ink.rgb * vec3(0.8, 0.92, 1.25), shadow);
    color = mix(color, stroke, ink * hachure.ink.a * mix(1.0, 0.85, shadow));
    // Water lines in a cooler ink, as on a coloured map.
    color = mix(color, hachure.ink.rgb * vec3(0.7, 0.95, 1.5), water_ink * 0.6);

    var out: FragmentOutput;
    out.color = main_pass_post_lighting_processing(pbr_input, vec4(color, lit.a));
    return out;
}
