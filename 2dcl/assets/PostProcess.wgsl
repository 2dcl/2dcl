#import bevy_sprite::mesh2d_view_bindings  view
#import bevy_sprite::mesh2d_vertex_output MeshVertexOutput
@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var texture_sampler: sampler;

@fragment
fn fragment(
    mesh: MeshVertexOutput
) -> @location(0) vec4<f32> {

    var PIXELSIZE = 2.0;
    var SLICES = floor(512.0/PIXELSIZE);
    var COLORSPERCHANNEL = 32.0;
    
    let viewport_uv = floor(coords_to_viewport_uv(mesh.position.xy, view.viewport)*SLICES)/SLICES;
    var output_color = textureSample(texture, texture_sampler, viewport_uv);
    output_color[3] = step(0.5,output_color[3]);

    output_color = trunc(output_color * COLORSPERCHANNEL) / COLORSPERCHANNEL;
    
    return output_color;
}

fn coords_to_viewport_uv(position: vec2<f32>, viewport: vec4<f32>) -> vec2<f32> {
    return (position - viewport.xy) / viewport.zw;
}
