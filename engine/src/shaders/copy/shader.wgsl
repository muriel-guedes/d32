@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
    switch i32(i) {
        case 0: {
            return vec4<f32>(-1., 1., 0., 1.);
        }
        case 1: {
            return vec4<f32>(-1., -1., 0., 1.);
        }
        case 2: {
            return vec4<f32>(1., -1., 0., 1.);
        }
        case 3: {
            return vec4<f32>(1., 1., 0., 1.);
        }
        case 4: {
            return vec4<f32>(-1., 1., 0., 1.);
        }
        default: {
            return vec4<f32>(1., -1., 0., 1.);
        }
    }
}

@group(0) @binding(0)
var<storage, read_write> output_buffer: array<u32>;
struct Size {
    @location(0) width: f32,
    @location(1) height: f32,
    @location(2) width_i32: i32,
    @location(3) height_i32: i32
};
@group(0) @binding(1)
var<storage, read> size: Size;

@fragment
fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let color = output_buffer[i32(position.x - 0.5) + i32(position.y - 0.5) * size.width_i32];
    return vec4<f32>(
        f32(color >> u32(24)) / 255.,
        f32((color << u32(8)) >> u32(24)) / 255.,
        f32((color << u32(16)) >> u32(24)) / 255.,
        1.
    );
}