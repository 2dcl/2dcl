extern crate prost_build;

fn main() {
    prost_build::compile_protos(
        &[
            "../protocol/ecs/components/Animator.proto",
            "../protocol/ecs/components/AudioSource.proto",
            "../protocol/ecs/components/AudioStream.proto",
            "../protocol/ecs/components/AvatarAttach.proto",
            "../protocol/ecs/components/AvatarShape.proto",
            "../protocol/ecs/components/Billboard.proto",
            "../protocol/ecs/components/BoxShape.proto",
            "../protocol/ecs/components/CameraModeArea.proto",
            "../protocol/ecs/components/CylinderShape.proto",
            "../protocol/ecs/components/GLTFShape.proto",
            "../protocol/ecs/components/NFTShape.proto",
            "../protocol/ecs/components/OnPointerDown.proto",
            "../protocol/ecs/components/OnPointerDownResult.proto",
            "../protocol/ecs/components/OnPointerUp.proto",
            "../protocol/ecs/components/OnPointerUpResult.proto",
            "../protocol/ecs/components/PlaneShape.proto",
            "../protocol/ecs/components/SphereShape.proto",
            "../protocol/ecs/components/TextShape.proto",
            "../protocol/ecs/components/UiTransform.proto",
            "../protocol/renderer-protocol/RendererProtocol.proto",
        ],
        &["../protocol/ecs/components/", "../protocol/"]).unwrap();
}
