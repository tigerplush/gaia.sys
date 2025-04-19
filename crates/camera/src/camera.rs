use bevy::prelude::*;
use common::states::Screen;
use leafwing_input_manager::{
    Actionlike, InputManagerBundle,
    plugin::InputManagerPlugin,
    prelude::{ActionState, InputMap, MouseMove, MouseScrollAxis},
};

#[derive(Clone, Default, Reflect, Resource)]
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
            .register_type::<CameraMovementIntentions>()
            .insert_resource(self.0.clone())
            .insert_resource(CameraMovementIntentions::default())
            .insert_resource(CameraPosition::default())
            .add_plugins(InputManagerPlugin::<CameraActions>::default())
            .add_systems(Startup, setup_camera)
            .add_systems(Update, (record_intentions, apply_intentions).run_if(in_state(Screen::Gameplay)))
            .add_systems(OnExit(Screen::Gameplay), reset_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    let input_map = InputMap::default()
        .with_axis(CameraActions::Zoom, MouseScrollAxis::Y)
        .with(CameraActions::PanActivate, MouseButton::Middle)
        .with_dual_axis(CameraActions::Pan, MouseMove::default());

    commands.spawn((
        Name::new("Camera"),
        camera(),
        // Render all UI to this camera.
        // Not strictly necessary since we only use one camera,
        // but if we don't use this component, our UI will disappear as soon
        // as we add another camera. This includes indirect ways of adding cameras like using
        // [ui node outlines](https://bevyengine.org/news/bevy-0-14/#ui-node-outline-gizmos)
        // for debugging. So it's good to have this here for future-proofing.
        IsDefaultUiCamera,
        InputManagerBundle::with_map(input_map),
    ));
}

fn reset_camera(camera_entity: Single<Entity, With<Camera3d>>, mut commands: Commands) {
    commands.entity(camera_entity.into_inner()).insert(camera());
}

fn camera() -> impl Bundle {
    (
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    )
}

#[derive(Actionlike, Clone, Debug, Eq, Hash, PartialEq, Reflect)]
enum CameraActions {
    #[actionlike(Axis)]
    Zoom,
    PanActivate,
    #[actionlike(DualAxis)]
    Pan,
}

fn record_intentions(
    mut intentions: ResMut<CameraMovementIntentions>,
    query: Single<&ActionState<CameraActions>>,
) {
    let action_state = query.into_inner();

    let mouse_move = action_state.axis_pair(&CameraActions::Pan);
    if action_state.pressed(&CameraActions::PanActivate) {
        intentions.pan = mouse_move;
    }
    else {
        intentions.pan = Vec2::ZERO;
    }

    intentions.zoom = action_state.value(&CameraActions::Zoom);
}

fn apply_intentions(
    time: Res<Time>,
    intentions: Res<CameraMovementIntentions>,
    settings: Res<CameraSettings>,
    mut position: ResMut<CameraPosition>,
    camera: Single<&mut Transform, With<Camera>>,
) {
    position.distance += intentions.zoom * settings.zoom_speed * time.delta_secs();
    position.distance = position.distance.clamp(settings.zoom_min, settings.zoom_max);
    position.longitude += intentions.pan.x * settings.pan_speed * time.delta_secs();
    if position.longitude < -180. {
        position.longitude += 360.;
    }
    else if position.longitude > 180. {
        position.longitude -= 360.;
    }
    position.latitude = (position.latitude + intentions.pan.y * settings.pan_speed * time.delta_secs()).clamp(-80., 80.);

    let mut transform = camera.into_inner();
    transform.translation = position.as_vec3();
    transform.look_at(Vec3::ZERO, Vec3::Y);
}

#[derive(Default, Reflect, Resource)]
#[reflect(Resource)]
pub struct CameraMovementIntentions {
    pub zoom: f32,
    pub pan: Vec2,
}

#[derive(Reflect, Resource)]
#[reflect(Resource)]
struct CameraPosition {
    /// east to west
    longitude: f32,
    /// north to south
    latitude: f32,
    distance: f32,
}

impl Default for CameraPosition {
    fn default() -> Self {
        Self {
            longitude: 0.,
            latitude: 0.,
            distance: 5.0,
        }
    }
}

impl CameraPosition {
    fn as_vec3(&self) -> Vec3 {
        let x = self.distance * self.latitude.to_radians().cos() * self.longitude.to_radians().cos();
        let y = self.distance * self.latitude.to_radians().sin();
        let z = self.distance * self.latitude.to_radians().cos() * self.longitude.to_radians().sin();
        Vec3 {
            x,
            y,
            z,
        }
    }
}
