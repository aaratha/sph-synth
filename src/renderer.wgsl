// Instanced quad-per-particle vertex/fragment shader.

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) local_pos: vec2<f32>,
};

const POINT_RADIUS: f32 = 0.01;

@vertex
fn vs_main(
    @location(0) corner: vec2<f32>,
    @location(1) instance_pos: vec2<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    let world_pos = instance_pos + corner * POINT_RADIUS;
    out.clip_position = vec4<f32>(world_pos, 0.0, 1.0);
    out.local_pos = corner;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if (dot(in.local_pos, in.local_pos) > 1.0) {
        discard;
    }
    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}
