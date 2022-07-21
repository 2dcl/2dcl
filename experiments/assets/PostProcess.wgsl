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
var texture: texture_2d<f32>;
[[group(1), binding(1)]]
var our_sampler: sampler;

[[stage(fragment)]]
fn fragment(input: VertexOutput) -> [[location(0)]] vec4<f32> {

    
    var ROUNDING_PREC = 0.99;
    var PIXELSIZE = 4.0;
    var Slices = floor(512.0/PIXELSIZE);
    
    let uv = floor(input.uv*Slices)/Slices;

    //    
    //let uv =input.uv;
   // var alpha_in = clamp(round(output_color[3]),0.0,1.0);
    //var xfactor = 
    var output_color = textureSample(texture, our_sampler, uv);
   
   output_color[3] = step(0.5,output_color[3]);
    //output_color[0] =testValue;
    //output_color[1] =testValue;
    //output_color[2] =testValue;

    return output_color;
}


