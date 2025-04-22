#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}

struct GeothermalMaterial {
    radius: f32,
};
@group(2) @binding(0) var<uniform> material: GeothermalMaterial;
// Texture and sampler
@group(2) @binding(1)
var gradient_texture: texture_2d<f32>;

@group(2) @binding(2)
var gradient_sampler: sampler;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) height: f32,
}

@vertex
fn vertex(input: Vertex) -> VertexOutput { 
    let original_pos = input.position;

    // Calculate the distance from center (height)
    let height = length(original_pos) - material.radius;

    // Normalize the position to project onto a perfect sphere
    let normalized_pos = normalize(original_pos) * material.radius;

    var output: VertexOutput;
    // output.clip_position = material.view_proj * vec4<f32>(normalized_pos, 1.0);
    // output.clip_position = vec4<f32>(normalized_pos, 1.0);
    output.clip_position = mesh_position_local_to_clip(
        get_world_from_local(input.instance_index),
        vec4<f32>(normalized_pos, 1.0),
    );
    output.height = height;
    return output;
}

struct FragmentInput {
    @location(0) height: f32,
};

@fragment
fn fragment(input: FragmentInput) -> @location(0) vec4<f32> {
    // let color = mix(vec3<f32>(0.0, 0.0, 1.0), vec3<f32>(1.0, 0.0, 0.0), input.height);
    let color = textureSample(gradient_texture, gradient_sampler, vec2<f32>(input.height - 0.5, 0.5));
    // return vec4<f32>(color, 1.0);
    return color;
}