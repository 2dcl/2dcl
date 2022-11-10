use bevy::{
  prelude::*,
  reflect::TypeUuid,
  render::{render_resource::{AsBindGroup, ShaderRef, RenderPipelineDescriptor, SpecializedMeshPipelineError, BlendState, BlendComponent, BlendFactor, BlendOperation}, mesh::MeshVertexBufferLayout}, sprite::{Material2d, Material2dKey},
};

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {
    #[uniform(0)]
  pub  color: Color,
    #[texture(1)]
    #[sampler(2)]
  pub source_image: Option<Handle<Image>>,
    
}



impl Material2d for CustomMaterial {
  fn specialize(
      descriptor: &mut RenderPipelineDescriptor,
      _layout: &MeshVertexBufferLayout,
      _key: Material2dKey<Self>,
  ) -> Result<(), SpecializedMeshPipelineError> {
      if let Some(fragment) = &mut descriptor.fragment {
        if let Some(target_state) = &mut fragment.targets[0] 
        {
          target_state.blend = Some(BlendState {
            color: BlendComponent {
                src_factor: BlendFactor::SrcAlpha,
                dst_factor: BlendFactor::One,
                operation: BlendOperation::Add,
            },
            alpha: BlendComponent {
                src_factor: BlendFactor::SrcAlpha,
                dst_factor: BlendFactor::One,
                operation: BlendOperation::Add,
            },
          }); 
        }
      }
      Ok(())
  }
}