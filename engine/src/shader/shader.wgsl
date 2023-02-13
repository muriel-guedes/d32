@vertex fn vs_main(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
    switch i32(i) {
        case 0:  { return vec4<f32>(-1.,  1., 0., 1.); }
        case 1:  { return vec4<f32>(-1., -1., 0., 1.); }
        case 2:  { return vec4<f32>( 1., -1., 0., 1.); }
        case 3:  { return vec4<f32>( 1.,  1., 0., 1.); }
        case 4:  { return vec4<f32>(-1.,  1., 0., 1.); }
        default: { return vec4<f32>( 1., -1., 0., 1.); }
    }
}

struct Camera {
    @location(0) position: vec4<f32>,
    @location(1) centre: vec4<f32>,
    @location(2) u: vec4<f32>,
    @location(3) v: vec4<f32>
};
@group(0) @binding(0)
var<uniform> camera: Camera;

struct Chunk {
    @location(0) position: vec4<f32>,
    @location(1) data: array<array<array<u32, 16>, 16>, 16>
};
@group(1) @binding(0)
var<storage, read> chunks: array<Chunk>;
@group(1) @binding(1)
var<uniform> chunks_length: vec4<u32>;

struct Ray {
    a: vec3<f32>,
    b: vec3<f32>,
    ab: vec3<f32>
}
fn get_ray_from(a: vec3<f32>, b: vec3<f32>) -> Ray {
    var ray: Ray;
    ray.a = a;
    ray.b = b;
    ray.ab = b - a;
    return ray;
}

fn camera_generate_ray(screen_coord: vec2<f32>) -> Ray {
    let world_position = camera.centre.xyz + (camera.u.xyz * screen_coord.x) + (camera.v.xyz * screen_coord.y);
    return get_ray_from(camera.position.xyz, world_position);
}

@fragment fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let ray = camera_generate_ray(position.xy);
    var chunk_id = u32(0);
    while(chunk_id < chunks_length.x) {
        let chunk = chunks[chunk_id];
        chunk_id++;
    }
    return vec4<f32>(1.);
}