@group(0) @binding(0)
var<storage, read> chunk_data: array<u32>;
struct Chunk {
    @location(0) position: vec4<f32>,
    @location(1) size: vec4<u32>
};
@group(0) @binding(1)
var<storage, read> chunk: Chunk;

@group(1) @binding(0)
var<storage, write> output_buffer: array<u32>;
@group(1) @binding(1)
var<storage, read_write> depth_buffer: array<f32>;
struct Size {
    @location(0) width: f32,
    @location(1) height: f32,
    @location(2) width_i32: i32,
    @location(3) height_i32: i32
};
@group(1) @binding(2)
var<storage, read> size: Size;

struct Camera {
    @location(0) start_projection: mat4x4<f32>,
    @location(1) final_projection: mat4x4<f32>
};
@group(2) @binding(0)
var<uniform> camera: Camera;

@compute @workgroup_size(16, 16, 1)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    if(id.x >= chunk.size.x || id.y >= chunk.size.y){ return; }
    let color = chunk_data[id.x + (id.y * chunk.size.x) + (id.z * chunk.size.w)];
    if(color == u32(0)){ return; }
    if(
        id.x != u32(0) && id.x < chunk.size.x - u32(1) &&
        id.y != u32(0) && id.y < chunk.size.y - u32(1) &&
        id.z != u32(0) && id.z < chunk.size.z - u32(1) &&
        chunk_data[id.x + u32(1) + (id.y * chunk.size.x) + (id.z * chunk.size.w)] != u32(0) &&
        chunk_data[id.x - u32(1) + (id.y * chunk.size.x) + (id.z * chunk.size.w)] != u32(0) &&
        chunk_data[id.x + ((id.y - u32(1)) * chunk.size.x) + (id.z * chunk.size.w)] != u32(0) &&
        chunk_data[id.x + ((id.y + u32(1)) * chunk.size.x) + (id.z * chunk.size.w)] != u32(0) &&
        chunk_data[id.x + (id.y * chunk.size.x) + ((id.z + u32(1)) * chunk.size.w)] != u32(0) &&
        chunk_data[id.x + (id.y * chunk.size.x) + ((id.z - u32(1)) * chunk.size.w)] != u32(0)
    ) { return; }
    
    let position = vec3<f32>(id) + chunk.position.xyz;
    
    var pixel = camera.start_projection * vec4<f32>(position, 1.);
    pixel.x /= pixel.w;
    if(pixel.x >= 1.){ return; }
    pixel.y /= pixel.w;
    if(pixel.y >= 1.){ return; }
    pixel.z /= pixel.w;
    if(pixel.z <= 0. || pixel.z >= 1.){ return; }
    
    var final_pixel = camera.final_projection * vec4<f32>(position, 1.);
    final_pixel.x /= final_pixel.w;
    if(final_pixel.x <= -1.){ return; }
    final_pixel.y /= final_pixel.w;
    if(final_pixel.y <= -1.){ return; }

    var fx = i32((final_pixel.x * 0.5 + 0.5) * size.width);
    var fy = i32((final_pixel.y * 0.5 + 0.5) * size.height);
    if(fx >= size.width_i32){ fx = size.width_i32 - 1; }
    if(fy >= size.height_i32){ fy = size.height_i32 - 1; }

    var x = i32((pixel.x * 0.5 + 0.5) * size.width);
    var y = i32((pixel.y * 0.5 + 0.5) * size.height);
    if(x < 0){ x = 0; }
    if(y < 0){ y = 0; }

    if(fx < x || fy < y){ return; }

    let startx = x;
    var i = x + (y * size.width_i32);
    let y_step = size.width_i32 + x - fx;
    loop {
        loop {
            if(depth_buffer[i] > final_pixel.z) {
                depth_buffer[i] = final_pixel.z;
                output_buffer[i] = color;
            }
            if(x == fx){ break; }
            x++;
            i++;
        }
        if(y == fy){ return; }
        y++;
        i += y_step;
        x = startx;
    }
}