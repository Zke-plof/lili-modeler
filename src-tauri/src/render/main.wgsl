struct Uniforms {
    view: mat4x4<f32>,
    projection: mat4x4<f32>,
    camera_pos: vec4<f32>,
    light_dir: vec4<f32>,
    light_color: vec4<f32>,
    ambient: vec4<f32>,
};

struct PushConstants {
    model: mat4x4<f32>,
    color: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) color: vec4<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let world_pos = uniforms.view * vec4<f32>(in.position, 1.0);
    out.clip_position = uniforms.projection * world_pos;
    out.world_position = in.position;
    out.world_normal = in.normal;
    out.uv = in.uv;
    out.color = in.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let normal = normalize(in.world_normal);
    let light_dir = normalize(uniforms.light_dir.xyz);
    let view_dir = normalize(uniforms.camera_pos.xyz - in.world_position);
    let half_dir = normalize(light_dir + view_dir);

    let diffuse = max(dot(normal, light_dir), 0.0);
    let specular = pow(max(dot(normal, half_dir), 0.0), 32.0);

    let ambient = uniforms.ambient.xyz;
    let diffuse_color = in.color.rgb * diffuse;
    let specular_color = uniforms.light_color.rgb * specular;

    let color = ambient * in.color.rgb + diffuse_color + specular_color;
    return vec4<f32>(color, in.color.a);
}

struct GridVertexInput {
    @location(0) position: vec3<f32>,
};

@vertex
fn vs_grid(in: GridVertexInput) -> VertexOutput {
    var out: VertexOutput;
    let world_pos = vec4<f32>(in.position, 1.0);
    out.clip_position = uniforms.projection * uniforms.view * world_pos;
    out.world_position = in.position;
    out.world_normal = vec3<f32>(0.0, 1.0, 0.0);
    out.uv = vec2<f32>(0.0);
    out.color = vec4<f32>(0.4, 0.4, 0.4, 0.5);
    return out;
}

@fragment
fn fs_grid(in: VertexOutput) -> @location(0) vec4<f32> {
    let alpha = 0.3;
    return vec4<f32>(in.color.rgb, alpha);
}

struct GizmoVertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

@vertex
fn vs_gizmo(in: GizmoVertexInput) -> VertexOutput {
    var out: VertexOutput;
    let world_pos = vec4<f32>(in.position, 1.0);
    out.clip_position = uniforms.projection * uniforms.view * world_pos;
    out.world_position = in.position;
    out.world_normal = vec3<f32>(0.0, 1.0, 0.0);
    out.uv = vec2<f32>(0.0);
    out.color = vec4<f32>(in.color, 1.0);
    return out;
}

@fragment
fn fs_gizmo(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color.rgb, 0.9);
}