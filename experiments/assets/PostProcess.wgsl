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
    // Get screen position with coordinates from 0 to 1


    var ROUNDING_PREC = 0.99;
    var PIXELSIZE = 4.0;
    var Slices = floor(512.0/PIXELSIZE);
    
    let uv = floor(coords_to_viewport_uv(position.xy, view.viewport)*Slices)/Slices;
    var output_color = textureSample(texture, our_sampler, uv);
    output_color[3] = step(0.5,output_color[3]);

    return output_color;
}
