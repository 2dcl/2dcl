#import bevy_sprite::mesh2d_view_bindings
#import bevy_pbr::utils

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {

    var PIXELSIZE = 2.0;
    var SLICES = floor(512.0/PIXELSIZE);
    var COLORSPERCHANNEL = 32.0;
    
    let uv = floor(coords_to_viewport_uv(position.xy, view.viewport)*SLICES)/SLICES;
    var output_color = textureSample(texture, our_sampler, uv);
    output_color[3] = step(0.5,output_color[3]);

    output_color = trunc(output_color * COLORSPERCHANNEL) / COLORSPERCHANNEL;
    
    return output_color;
}
