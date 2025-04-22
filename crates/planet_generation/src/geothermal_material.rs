use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

#[derive(AsBindGroup, Asset, Clone, TypePath)]
pub(crate) struct GeothermalMaterial {
    #[uniform(0)]
    pub radius: f32,
    #[texture(1)]
    #[sampler(2)]
    pub gradient_texture: Handle<Image>,
}

impl Material for GeothermalMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/geothermal.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/geothermal.wgsl".into()
    }
}
