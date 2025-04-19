use bevy::prelude::*;
use common::states::Screen;

#[derive(Clone, Resource, Reflect)]
#[reflect(Resource)]
pub struct CameraSettings {
    pub zoom_speed: f32,
    pub zoom_min: f32,
    pub zoom_max: f32,
    pub pan_speed: f32,
}

#[derive(Component)]
pub struct CameraPlugin(pub CameraSettings);

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CameraSettings>()
            .insert_resource(self.0.clone())
            .add_systems(Startup, setup_camera)
            .add_systems(OnExit(Screen::Gameplay), reset_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        // Render all UI to this camera.
        // Not strictly necessary since we only use one camera,
        // but if we don't use this component, our UI will disappear as soon
        // as we add another camera. This includes indirect ways of adding cameras like using
        // [ui node outlines](https://bevyengine.org/news/bevy-0-14/#ui-node-outline-gizmos)
        // for debugging. So it's good to have this here for future-proofing.
        IsDefaultUiCamera,
    ));
}

fn reset_camera(camera: Single<Entity, With<Camera3d>>, mut commands: Commands) {
    commands
        .entity(camera.into_inner())
        .insert(Camera3d::default());
}
