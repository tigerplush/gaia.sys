//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{
    color::palettes::css::WHITE,
    dev_tools::states::log_transitions,
    input::common_conditions::input_just_pressed,
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
    ui::UiDebugOptions,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use common::states::Screen;

pub fn plugin(app: &mut App) {
    app.add_plugins((WorldInspectorPlugin::new(), WireframePlugin));
    app.insert_resource(WireframeConfig {
        global: false,
        default_color: WHITE.into(),
    });
    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);

    // Toggle the debug overlay for UI.
    app.add_systems(
        Update,
        toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
    );
}

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}
