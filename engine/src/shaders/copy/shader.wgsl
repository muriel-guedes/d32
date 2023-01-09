struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>
};

@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> Output {
    var out: Output;
    switch i32(i) {
        case 0: {
            out.position = vec4<f32>(-1., 1., 0.0, 1.);
            out.uv = vec2<f32>(0., 0.);
        }
        case 1: {
            out.position = vec4<f32>(-1., -1., 0.0, 1.);
            out.uv = vec2<f32>(0., 1.);
        }
        case 2: {
            out.position = vec4<f32>(1., -1., 0.0, 1.);
            out.uv = vec2<f32>(1., 1.);
        }
        case 3: {
            out.position = vec4<f32>(1., 1., 0.0, 1.);
            out.uv = vec2<f32>(1., 0.);
        }
        case 4: {
            out.position = vec4<f32>(-1., 1., 0.0, 1.);
            out.uv = vec2<f32>(0., 0.);
        }
        default: {
            out.position = vec4<f32>(1., -1., 0.0, 1.);
            out.uv = vec2<f32>(1., 1.);
        }
    }
    return out;
}

@group(0) @binding(0)
var texture: texture_2d<f32>;
@group(0) @binding(1)
var texture_sampler: sampler;

@fragment
fn fs_main(in: Output) -> @location(0) vec4<f32> {
    return textureSample(texture, texture_sampler, in.uv);
}