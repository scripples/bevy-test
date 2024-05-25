use bevy::{
    asset::Asset,
    math::Vec2,
    pbr::Material,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};

use super::traits::MaterialTime;

#[derive(Asset, TypePath, Default, AsBindGroup, Debug, Clone)]
pub struct GridMaterial {
    #[uniform(0)]
    time: f32,
    #[uniform(1)]
    pub window_size: Vec2,
}

impl Material for GridMaterial {
    // fn vertex_shader() -> ShaderRef {
    //     "shaders/grid1/vertex.wgsl".into()
    // }

    fn fragment_shader() -> ShaderRef {
        "shaders/grid1/sin.wgsl".into()
    }
}

impl GridMaterial {
    pub fn update_window_size(&mut self, new_size: Vec2) {
        self.window_size = new_size;
    }
}

impl MaterialTime for GridMaterial {
    fn set_time(&mut self, new_time: f32) {
        self.time = new_time;
    }
}
