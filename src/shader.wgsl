// Vertex shader

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coord: vec3<f32>,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coord: vec3<f32>,
}

struct CameraUniform {
    view_proj: mat4x4<f32>,
}

// struct InstanceInput {
//     @location(5) model_matrix_0: vec4<f32>,
//     @location(6) model_matrix_1: vec4<f32>,
//     @location(7) model_matrix_2: vec4<f32>,
//     @location(8) model_matrix_3: vec4<f32>,
// };


@group(1) @binding(0)
var<uniform> camera: CameraUniform;

@vertex
fn vs_main(
    in: VertexInput,
    // instance: InstanceInput,
) -> VertexOutput {
    // let model_matrix = mat4x4<f32> (
    //     instance.model_matrix_0,
    //     instance.model_matrix_1,
    //     instance.model_matrix_2,
    //     instance.model_matrix_3
    // );
    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(in.position, 1.0);
    out.tex_coord = in.tex_coord;
    return out;
}

// Fragment shader

// grass side
@group(0) @binding(0)
var side_t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var side_s_diffuse: sampler;


// grass top
@group(0) @binding(2)
var top_t_diffuse: texture_2d<f32>;
@group(0) @binding(3)
var top_s_diffuse: sampler;

// dirt
@group(0) @binding(4)
var dirt_t_diffuse: texture_2d<f32>;
@group(0) @binding(5)
var dirt_s_diffuse: sampler;


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if in.tex_coord.z == 0.0 {
        return textureSample(side_t_diffuse, side_s_diffuse, in.tex_coord.xy);
    }
    else if in.tex_coord.z == 1.0{
        return textureSample(top_t_diffuse, top_s_diffuse, in.tex_coord.xy) * vec4(0.5, 1.0, 0.15, 1.0);
    }
    else {
        return textureSample(dirt_t_diffuse, dirt_s_diffuse, in.tex_coord.xy);
    }
}