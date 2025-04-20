use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};
use common::states::Screen;

#[derive(Reflect, Resource)]
#[reflect(Resource)]
struct PlanetSettings {
    resolution: u32,
}

pub fn plugin(app: &mut App) {
    app.register_type::<PlanetSettings>()
        .register_type::<TerrainFace>()
        .insert_resource(PlanetSettings { resolution: 10 })
        .add_systems(OnEnter(Screen::Gameplay), spawn_planet);
}

fn spawn_planet(
    settings: Res<PlanetSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    // light
    commands.spawn((PointLight::default(), Transform::from_xyz(2.0, 4.0, 2.0)));

    let material_handle = materials.add(StandardMaterial::default());
    // cube
    commands
        .spawn((
            Transform::default(),
            Visibility::Inherited,
            Name::new("Planet"),
        ))
        .with_children(|parent| {
            let directions = [
                Vec3::X,
                Vec3::Y,
                Vec3::Z,
                Vec3::NEG_X,
                Vec3::NEG_Y,
                Vec3::NEG_Z,
            ];
            for local_up in directions {
                let terrain_face = TerrainFace::new(local_up);
                let mesh_handle = meshes.add(terrain_face.to_mesh(settings.resolution));
                parent.spawn((
                    terrain_face,
                    Mesh3d(mesh_handle),
                    MeshMaterial3d(material_handle.clone()),
                ));
            }
        });
}

#[derive(Component, Reflect)]
struct TerrainFace {
    local_up: Vec3,
    axis_a: Vec3,
    axis_b: Vec3,
}

impl TerrainFace {
    fn new(local_up: Vec3) -> Self {
        let axis_a = Vec3::new(local_up.y, local_up.z, local_up.x);
        let axis_b = local_up.cross(axis_a);
        Self {
            local_up,
            axis_a,
            axis_b,
        }
    }

    fn to_mesh(&self, resolution: u32) -> Mesh {
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
                vertices.push(point_on_unit_sphere);
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
