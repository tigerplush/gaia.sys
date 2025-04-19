use bevy::{
    asset::AssetMetaCheck,
    audio::{AudioPlugin, Volume},
    prelude::*,
};
use camera::{CameraPlugin, CameraSettings};
use common::system_sets::AppSet;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Order new `AppStep` variants by adding them here:
        app.configure_sets(
            Update,
            (AppSet::TickTimers, AppSet::RecordInput, AppSet::Update).chain(),
        );

        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "gaia.sys".to_string(),
                        fit_canvas_to_parent: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(AudioPlugin {
                    global_volume: GlobalVolume {
                        volume: Volume::Linear(0.3),
                    },
                    ..default()
                }),
        );

        // Add other plugins.
        app.add_plugins((
            asset_tracking::plugin,
            CameraPlugin(CameraSettings {
                zoom_speed: 10.,
                zoom_min: 1.,
                zoom_max: 100.,
                pan_speed: 10.,
            }),
            screens::plugin,
            theme::plugin,
        ));

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins(dev_tools::plugin);
    }
}
