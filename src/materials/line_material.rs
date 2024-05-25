use bevy::{
    asset::Asset,
    pbr::{Material, MaterialPipeline, MaterialPipelineKey},
    reflect::TypePath,
    render::{
        color::Color,
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, PolygonMode, RenderPipelineDescriptor, ShaderRef,
            SpecializedMeshPipelineError,
        },
    },
};

use super::traits::MaterialTime;

#[derive(Asset, TypePath, Default, AsBindGroup, Debug, Clone)]
pub struct LineMaterial {
    #[uniform(0)]
    pub color: Color,
    #[uniform(1)]
    pub time: f32,
}

impl Material for LineMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/line_vertex_shader.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/line_material.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // This is the important part to tell bevy to render this material as a line between vertices
        descriptor.primitive.polygon_mode = PolygonMode::Line;
        Ok(())
    }
}

impl MaterialTime for LineMaterial {
    fn set_time(&mut self, new_time: f32) {
        self.time = new_time;
    }
}

#[derive(Asset, TypePath, Default, AsBindGroup, Debug, Clone)]
pub struct LineMaterialWhite {
    #[uniform(0)]
    pub color: Color,
    #[uniform(1)]
    pub time: f32,
}

impl Material for LineMaterialWhite {
    fn vertex_shader() -> ShaderRef {
        "shaders/line_vertex_shader.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/line_material_white.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // This is the important part to tell bevy to render this material as a line between vertices
        descriptor.primitive.polygon_mode = PolygonMode::Line;
        Ok(())
    }
}

impl MaterialTime for LineMaterialWhite {
    fn set_time(&mut self, new_time: f32) {
        self.time = new_time;
    }
}
