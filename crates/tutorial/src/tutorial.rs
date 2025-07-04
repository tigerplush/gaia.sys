use std::sync::{Arc, atomic::AtomicBool};

use bevy::prelude::*;
use bevy_yarnspinner::prelude::{YarnProject, YarnSpinnerPlugin};
use bevy_yarnspinner_example_dialogue_view::ExampleYarnSpinnerDialogueViewPlugin;
use camera::CameraMovementIntentions;
use common::states::Screen;
use planet_generation::PlanetState;

use crate::input_observer::*;

#[derive(Resource, Reflect)]
pub struct Tutorial;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        YarnSpinnerPlugin::new(),
        ExampleYarnSpinnerDialogueViewPlugin::new(),
    ))
    .add_systems(
        Update,
        (
            spawn_dialog_runner.run_if(
                in_state(Screen::Gameplay)
                    .and(resource_exists::<Tutorial>)
                    .and(resource_added::<YarnProject>),
            ),
            observe_input.run_if(
                in_state(Screen::Gameplay)
                    .and(resource_exists::<Tutorial>)
                    .and(resource_exists::<InputObserver>),
            ),
        ),
    );
}

fn spawn_dialog_runner(project: Res<YarnProject>, mut commands: Commands) {
    // Create a dialogue runner from the project.
    let mut dialogue_runner = project.create_dialogue_runner();
    dialogue_runner
        .commands_mut()
        .add_command("wait_input", wait_input);
    // Immediately start showing the dialogue to the player
    dialogue_runner.start_node("HelloWorld");
    commands.spawn(dialogue_runner);
}

fn wait_input(In(input): In<String>, mut commands: Commands) -> Arc<AtomicBool> {
    match input.as_str() {
        "zoom" => {
            let zoom_observer = ZoomObserver::new();
            let done = zoom_observer.done.clone();
            commands.insert_resource(InputObserver::Zoom(zoom_observer));
            done
        }
        "pan" => {
            let pan_observer = PanObserver::new();
            let done = pan_observer.done.clone();
            commands.insert_resource(InputObserver::Pan(pan_observer));
            done
        }
        "geothermal" => {
            let geothermal_observer = OverlayObserver::new();
            let done = geothermal_observer.done.clone();
            commands.insert_resource(InputObserver::Overlay(geothermal_observer));
            done
        }
        _ => Arc::new(true.into()),
    }
}

fn observe_input(
    input_observer: ResMut<InputObserver>,
    camera_intentions: Res<CameraMovementIntentions>,
    single: Single<&PlanetState>,
) {
    match input_observer.into_inner() {
        InputObserver::Zoom(zoom) => {
            if camera_intentions.zoom != 0. {
                zoom.set_done();
            }
        }
        InputObserver::Pan(pan) => {
            if camera_intentions.pan != Vec2::ZERO {
                pan.set_done();
            }
        }
        InputObserver::Overlay(overlay) => {
            if single.into_inner().geothermal_overlay {
                overlay.set_done();
            }
        }
    }
}
