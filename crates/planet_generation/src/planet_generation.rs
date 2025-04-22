use bevy::{color::palettes::css::BLUE, prelude::*};
use common::states::Screen;
use leafwing_input_manager::plugin::InputManagerPlugin;
use noise::OpenSimplex;

use crate::{
    controls::{self, PlanetActions},
    noise_filter::{NoiseFilter, NoiseSettings},
    planet_settings::PlanetSettings,
    terrain_face::TerrainFace,
};

#[derive(Component)]
pub(crate) struct Planet;

#[derive(Component)]
pub(crate) struct GeothermalOverlay;

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
        .add_plugins(InputManagerPlugin::<PlanetActions>::default())
        .add_systems(OnEnter(Screen::Gameplay), (spawn_planet, controls::setup))
        .add_systems(Update, controls::check.run_if(in_state(Screen::Gameplay)));
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

    commands
        .spawn((
            Transform::default(),
            Visibility::Inherited,
            Name::new("Planet"),
            Planet,
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

    let geothermal_settings = PlanetSettings { ..default() }.with_layer(NoiseFilter {
        noise: OpenSimplex::new(0),
        settings: NoiseSettings {
            number_of_layers: 1,
            strength: 1.0,
            base_roughness: 2.0,
            roughness: 1.0,
            persistence: 0.0,
            center: Vec3::ZERO,
            min_value: 0.0,
            use_first_layer_as_mask: false,
        },
    });
    commands
        .spawn((
            Transform::default(),
            Visibility::Hidden,
            Name::new("Geothermal Overlay"),
            GeothermalOverlay,
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
                let mesh_handle = meshes.add(terrain_face.to_mesh(&geothermal_settings));
                parent.spawn((
                    terrain_face,
                    Mesh3d(mesh_handle),
                    MeshMaterial3d(material_handle.clone()),
                ));
            }
        });
}

// fn update_face(
//     settings: Res<PlanetSettings>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     query: Query<(Entity, &TerrainFace)>,
//     mut commands: Commands,
// ) {
//     for (entity, face) in &query {
//         let mesh_handle = meshes.add(face.to_mesh(&settings));
//         commands.entity(entity).insert(Mesh3d(mesh_handle));
//     }
// }
