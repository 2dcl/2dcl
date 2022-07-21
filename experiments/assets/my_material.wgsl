//https://github.com/bevyengine/bevy/blob/c2da7800e3671ad92e775529070a814d0bc2f5f8/crates/bevy_sprite/src/mesh2d/mesh2d.wgsl
struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};

struct MyMat {
    alpha: f32;
    color: vec4<f32>;
};

[[group(1), binding(0)]]
var<uniform> uniform_data: MyMat;

[[group(1), binding(1)]]
var texture: texture_2d<f32>;
[[group(1), binding(2)]]
var our_sampler: sampler;

[[stage(fragment)]]
fn fragment(input: VertexOutput) -> [[location(0)]] vec4<f32> {
//#define ROUNDING_PREC 0.999
//#define PIXELSIZE 5.0
//inline void PixelClipAlpha_float(float4 posCS, float alpha_in, out float alpha_out) {
//  alpha_in = clamp(round(alpha_in), 0.0, 1.0);
//  xfactor = step(fmod(abs(floor(posCS.x)), PIXELSIZE), ROUNDING_PREC);
//  yfactor = step(fmod(abs(floor(posCS.y - PIXELSIZE)), PIXELSIZE), ROUNDING_PREC);
//  alpha_out = alpha_in * xfactor * yfactor * alpha_in;
//}
//fn textureSample(t: texture_1d<f32>,
 //                s: sampler,
 //                coords: f32) -> vec4<f32>


    var output_color = vec4<f32>(input.uv,0.0,uniform_data.alpha);
    output_color = output_color * textureSample(texture, our_sampler, input.uv);
    output_color = output_color * uniform_data.color;
    var ROUNDING_PREC = 0.999;
    var PIXELSIZE = 5.0;
    var alpha_in = clamp(round(output_color[3]),0.0,1.0);
    //var xfactor = step(1.0%5.0,0.99);
    var xfactor = step(abs(floor(input.uv[0]))% PIXELSIZE, ROUNDING_PREC);
    var yfactor = step(abs(floor(input.uv[1] - PIXELSIZE))% PIXELSIZE, ROUNDING_PREC);
    var alpha_out = alpha_in * xfactor * yfactor * alpha_in;
    output_color[3] = alpha_in;
    return output_color;
}


