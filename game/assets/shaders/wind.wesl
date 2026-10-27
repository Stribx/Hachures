// Le vent : the ink material's vertex stage. What is given some wind (`Hachure::wind`) leans downwind from its foot;
// everything else stands as Bevy's own vertex stage would put it. Bevy's stage without skinning or morph targets,
// which the model's pieces never have. `wind_prepass.wesl` does the same for the shadows and the depth.

import bevy_pbr::render::{
    mesh_functions,
    mesh_view_bindings::globals,
    forward_io::{Vertex, VertexOutput, decompress_vertex},
    view_transformations::position_world_to_clip,
};

// As in `hachure.wesl`.
struct Hachure {
    paper: vec4<f32>,
    ink: vec4<f32>,
    params: vec4<f32>,
    shore: vec4<f32>,
    gloss: f32,
    flow: f32,
    // How far a piece leans in the wind, per unit of height squared: 0 for what never moves.
    wind: f32,
}

@group(constants::MATERIAL_BIND_GROUP) @binding(100)
var<uniform> hachure: Hachure;

// The wind, from the west-south-west, flat on the land.
const WIND: vec2<f32> = vec2(0.94, 0.34);

// How far a point `world` of a piece standing at `base` is blown at time `t`: nothing at its foot, more the higher it
// stands. A gust rolls downwind across the land, and under it each piece sways in its own time, as a whole.
fn sway(world: vec3<f32>, base: vec3<f32>, t: f32) -> vec3<f32> {
    let h = max(world.y - base.y, 0.0);
    let along = dot(base.xz, WIND);
    let gust = 0.5 + 0.5 * sin(along * 0.15 - t * 1.2);
    let lean = 0.3 + gust * gust + 0.25 * sin(t * 2.1 + base.x * 3.7 + base.z * 1.3);
    let d = hachure.wind * lean * h * h;
    return vec3(WIND.x * d, 0.0, WIND.y * d);
}

@vertex
fn vertex(vertex_in: Vertex) -> VertexOutput {
    var out: VertexOutput;
    let vertex = decompress_vertex(vertex_in, vertex_in.instance_index);
    let world_from_local = mesh_functions::get_world_from_local(vertex_in.instance_index);
    let still = mesh_functions::mesh_position_local_to_world(world_from_local, vec4<f32>(vertex.position, 1.0)).xyz;
    out.world_position = vec4(still + sway(still, world_from_local[3].xyz, globals.time), 1.0);
    out.position = position_world_to_clip(out.world_position.xyz);

@if(VERTEX_NORMALS)
    out.world_normal = mesh_functions::mesh_normal_local_to_world(vertex.normal, vertex_in.instance_index);
@if(VERTEX_UVS_A)
    out.uv = vertex.uv;
@if(VERTEX_UVS_B)
    out.uv_b = vertex.uv_b;
@if(VERTEX_TANGENTS)
    out.world_tangent = mesh_functions::mesh_tangent_local_to_world(world_from_local, vertex.tangent, vertex_in.instance_index);
@if(VERTEX_COLORS)
    out.color = vertex.color;
@if(VERTEX_OUTPUT_INSTANCE_INDEX)
    out.instance_index = vertex_in.instance_index;
@if(VISIBILITY_RANGE_DITHER)
    out.visibility_range_dither = mesh_functions::get_visibility_range_dither_level(vertex_in.instance_index, world_from_local[3]);

    return out;
}
