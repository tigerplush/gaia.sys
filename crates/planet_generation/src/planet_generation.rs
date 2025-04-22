use bevy::{
    asset::RenderAssetUsages,
    color::palettes::css::BLUE,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use common::states::Screen;
use noise::OpenSimplex;

use crate::noise_filter::{NoiseFilter, NoiseSettings};

#[derive(InspectorOptions, Reflect, Resource)]
#[reflect(InspectorOptions, Resource)]
struct PlanetSettings {
    #[inspector(min = 2, max = 255)]
    resolution: u32,
    color: Color,
    radius: f32,
    noise_filters: Vec<NoiseFilter>,
}

impl PlanetSettings {
    fn calculate_point_on_planet(&self, point_on_unit_sphere: Vec3) -> Vec3 {
        let mut elevation = 0.0;
        let mut first_layer_value = 0.0;
        if let Some(filter) = self.noise_filters.first() {
            first_layer_value = filter.evaluate(point_on_unit_sphere);
            elevation = first_layer_value;
        }
        for filter in self.noise_filters.iter().skip(1) {
            let mask = if filter.settings.use_first_layer_as_mask {
                first_layer_value
            } else {
                1.0
            };
            elevation += filter.evaluate(point_on_unit_sphere) * mask;
        }
        point_on_unit_sphere * self.radius * (1.0 + elevation)
    }

    fn with_layer(mut self, layer: NoiseFilter) -> Self {
        self.noise_filters.push(layer);
        self
    }
}

impl Default for PlanetSettings {
    fn default() -> Self {
        Self {
            resolution: 100,
            color: BLUE.into(),
            radius: 2.0,
            noise_filters: Vec::new(),
        }
    }
}

pub fn plugin(app: &mut App) {
    app.register_type::<PlanetSettings>()
        .register_type::<TerrainFace>()
        .insert_resource(
            PlanetSettings {
                resolution: 100,
                color: BLUE.into(),
                radius: 2.0,
                ..default()
            }
            .with_layer(NoiseFilter {
                noise: OpenSimplex::new(0),
                settings: NoiseSettings {
                    number_of_layers: 5,
                    strength: 0.2,
                    base_roughness: 0.71,
                    roughness: 1.81,
                    persistence: 0.54,
                    center: Vec3::ZERO,
                    min_value: 1.1,
                    use_first_layer_as_mask: false,
                },
            })
            .with_layer(NoiseFilter {
                noise: OpenSimplex::new(0),
                settings: NoiseSettings {
                    number_of_layers: 5,
                    strength: 10.0,
                    base_roughness: 1.08,
                    roughness: 2.34,
                    persistence: 0.53,
                    center: Vec3::ZERO,
                    min_value: 1.2,
                    use_first_layer_as_mask: true,
                },
            }),
        )
        .add_systems(OnEnter(Screen::Gameplay), spawn_planet)
        .add_systems(Update, update_face.run_if(in_state(Screen::Gameplay)));
}

fn spawn_planet(
    settings: Res<PlanetSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    // light
    commands.spawn((PointLight::default(), Transform::from_xyz(10.0, 0.0, 2.0)));

    let material_handle = materials.add(settings.color);
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
                let mesh_handle = meshes.add(terrain_face.to_mesh(&settings));
                parent.spawn((
                    terrain_face,
                    Mesh3d(mesh_handle),
                    MeshMaterial3d(material_handle.clone()),
                ));
            }
        });
}

fn update_face(
    settings: Res<PlanetSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(Entity, &TerrainFace)>,
    mut commands: Commands,
) {
    for (entity, face) in &query {
        let mesh_handle = meshes.add(face.to_mesh(&settings));
        commands.entity(entity).insert(Mesh3d(mesh_handle));
    }
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

    fn to_mesh(&self, settings: &PlanetSettings) -> Mesh {
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
