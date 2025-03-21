// Vertex shader

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coord: vec3<f32>,
    @location(1) distance: f32,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coord: vec3<f32>,
}

struct CameraUniform {
    view_proj: mat4x4<f32>,
    view_pos: vec3<f32>,
}

@group(1) @binding(0)
var<uniform> camera: CameraUniform;

@vertex
fn vs_main(
    in: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(in.position, 1.0);
    out.tex_coord = in.tex_coord;
    out.distance = distance(camera.view_pos, in.position);
    return out;
}

// Fragment shader

// textures
@group(0) @binding(0)
var side_t_diffuse: texture_3d<f32>;
@group(0) @binding(1)
var side_s_diffuse: sampler;


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var sample = textureSample(side_t_diffuse, side_s_diffuse, in.tex_coord); 
    // grass top id
    if in.tex_coord.z == 0.44929785 {
        sample = sample * vec4<f32>(0.3, 1.0, 0.15, 1.0);
    }
    // sample += vec4(1-pow(0.999, in.distance));
    return sample;
}