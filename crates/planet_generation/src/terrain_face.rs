use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

use crate::planet_settings::PlanetSettings;

#[derive(Component, Reflect)]
pub(crate) struct TerrainFace {
    local_up: Vec3,
    axis_a: Vec3,
    axis_b: Vec3,
}

impl TerrainFace {
    pub(crate) fn new(local_up: Vec3) -> Self {
        let axis_a = Vec3::new(local_up.y, local_up.z, local_up.x);
        let axis_b = local_up.cross(axis_a);
        Self {
            local_up,
            axis_a,
            axis_b,
        }
    }

    pub(crate) fn to_mesh(&self, settings: &PlanetSettings) -> Mesh {
        let resolution = settings.resolution;
        let mut vertices: Vec<Vec3> = Vec::with_capacity((resolution * resolution) as usize);
        let mut indices: Vec<u32> =
            Vec::with_capacity((resolution - 1) as usize * (resolution - 1) as usize);
        let mut uvs: Vec<Vec2> = Vec::with_capacity((resolution * resolution) as usize);

        for y in 0..resolution {
            for x in 0..resolution {
                let percent = UVec2::new(x, y).as_vec2() / (resolution - 1) as f32;
                let point_on_unit_cube = self.local_up
                    + (percent.x - 0.5) * 2.0 * self.axis_a
                    + (percent.y - 0.5) * 2.0 * self.axis_b;
                let point_on_unit_sphere = point_on_unit_cube.normalize();
                let point_on_planet = settings.calculate_point_on_planet(point_on_unit_sphere);
                vertices.push(point_on_planet);
                uvs.push(percent);

                if x != resolution - 1 && y != resolution - 1 {
                    let index = x + y * resolution;
                    indices.push(index);
                    indices.push(index + resolution + 1);
                    indices.push(index + resolution);

                    indices.push(index);
                    indices.push(index + 1);
                    indices.push(index + resolution + 1);
                }
            }
        }
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
        .with_inserted_indices(Indices::U32(indices))
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_computed_normals()
    }
}
