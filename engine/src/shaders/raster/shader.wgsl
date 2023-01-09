@group(0) @binding(0)
var<storage, read_write> chunk: array<u32>;

@group(1) @binding(0) 
var output_texture: texture_storage_2d<rgba8unorm, write>;
@group(1) @binding(1)
var<storage, read_write> depth_buffer: array<f32>;

struct Camera {
    @location(0) projection: mat4x4<f32>,
    @location(1) position: vec4<f32>
};
@group(2) @binding(0)
var<uniform> camera: Camera;

@compute @workgroup_size(16, 16, 1)
fn main(@builtin(global_invocation_id) i: vec3<u32>) {
    let texsize = textureDimensions(output_texture);

    let y = i.y * u32(16);
    let z = i.z * u32(256);
    let c = chunk[i.x + y + z];
    let a = (c << u32(24)) >> u32(24);
    if(a == u32(0)){ return; }
    
    var pos = camera.projection * vec4<f32>((vec3<f32>(i) - vec3<f32>(8., 8., 8.)), 1.);
    pos.x /= pos.w;
    pos.y /= pos.w;
    pos.z /= pos.w;
    if(pos.x >= 1. || pos.y >= 1. || pos.z <= 0. || pos.z >= 1.) { return; }
    pos.x = (pos.x * 0.5 + 0.5) * f32(texsize.x);
    pos.y = (pos.y * 0.5 + 0.5) * f32(texsize.y);
    
    let color = vec4<f32>(
        f32(c >> u32(24)) / 255.,
        f32(((c << u32(8)) >> u32(24))) / 255.,
        f32((c << u32(16)) >> u32(24)) / 255.,
        f32(a) / 255.
    );

    let s = i32((1. / pos.w) * 500.);
    var ps = vec2<i32>(s, s);
    while(ps.y >= 0) {
        ps.x = s;
        while(ps.x >= 0) {
            let texpos = vec2<i32>(pos.xy) + ps;
            let i = texpos.x + (texpos.y*texsize.x);
            ps.x -= 1;
            if(
                depth_buffer[i] < pos.w ||
                texpos.x >= texsize.x || texpos.x <= 0 ||
                texpos.y >= texsize.y || texpos.y <= 0
            ) { continue; }
            depth_buffer[i] = pos.w;
            textureStore(output_texture, texpos, color);
        }
        ps.y -= 1;
    }
}