#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct PostProcessSettings {
    intensity: f32,
#ifdef SIXTEEN_BYTE_ALIGNMENT// WebGL2 structs must be 16 byte aligned.
    _webgl2_padding: vec3<f32>
#endif
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;
@group(0) @binding(3) var depth_texture: texture_depth_2d;

fn rgb_to_hsv(c: vec3<f32>) -> vec3<f32> {
    let k = vec4<f32>(0.0, -1.0 / 3.0, 2.0 / 3.0, -1.0);
    let p = mix(vec4<f32>(c.zy, k.wz), vec4<f32>(c.yz, k.xy), step(c.z, c.y));
    let q = mix(vec4<f32>(p.xyw, c.x), vec4<f32>(c.x, p.yzx), step(p.x, c.x));

    let d = q.x - min(q.w, q.y);
    let e = 1.0e-10;
    return vec3<f32>(abs(q.z + (q.w - q.y) / (6.0 * d + e)), d / (q.x + e), q.x);
}

fn hsv_to_rgb(c: vec3<f32>) -> vec3<f32> {
    let k = vec4<f32>(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    let p = abs(fract(c.xxx + k.xyz) * 6.0 - k.www);
    return c.z * mix(k.xxx, clamp(p - k.xxx, vec3<f32>(0.0), vec3<f32>(1.0)), c.y);
}

fn discretize_value(color: vec3<f32>, resolution: f32) -> vec3<f32> {
    let c = rgb_to_hsv(color);
    let c_2 = vec3<f32>(c.xy, floor(c.z * resolution) / resolution);
    return hsv_to_rgb(c_2);
}

fn pixelize_uvs(uv: vec2<f32>, resolution: f32) -> vec2<f32> {
    let new_uv = floor(uv * resolution) / resolution;
    return new_uv;
}

fn get_depth(uv: vec2<f32>) -> f32 {
    return textureSample(depth_texture, texture_sampler, uv);
}

fn detect_depth_edges(uv: vec2<f32>, texel_size: vec2<f32>) -> f32 {
    let center = get_depth(uv);
    
    // Sample 4 neighbors (1 texel offset)
    let left  = get_depth(uv + vec2<f32>(-texel_size.x, 0.0));
    let right = get_depth(uv + vec2<f32>(texel_size.x, 0.0));
    let up    = get_depth(uv + vec2<f32>(0.0, texel_size.y));
    let down  = get_depth(uv + vec2<f32>(0.0, -texel_size.y));

    // Calculate the absolute difference
    let diff = (abs(left - center) + abs(right - center) + abs(up - center) + abs(down - center))/4.;

    // Threshold determines sensitivity. 
    // Since depth is non-linear, a very small value is usually needed.
    let threshold = 0.8;
    
    return 1. - step(0.001, diff);
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let uv = pixelize_uvs(in.uv, 300);
    let texel = 1./300.;
    let depth = textureSample(depth_texture, texture_sampler, uv);
    let outline = detect_depth_edges(uv, vec2(texel, texel));
    let primitive_color = textureSample(screen_texture, texture_sampler, uv);

    let color = discretize_value(primitive_color.rgb, 50.0);
    return vec4<f32>(color.rgb * outline, 1.0);
}
