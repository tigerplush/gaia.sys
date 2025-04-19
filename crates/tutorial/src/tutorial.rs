use std::sync::{Arc, atomic::AtomicBool};

use bevy::prelude::*;
use bevy_yarnspinner::prelude::{YarnProject, YarnSpinnerPlugin};
use bevy_yarnspinner_example_dialogue_view::ExampleYarnSpinnerDialogueViewPlugin;
use common::states::Screen;

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
    info!("{}", input);
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
        _ => Arc::new(true.into()),
    }
}

fn observe_input(input_observer: ResMut<InputObserver>) {
    match input_observer.into_inner() {
        InputObserver::Zoom(zoom) => {
            zoom.set_done();
        }
        InputObserver::Pan(pan) => {
            pan.set_done();
        }
    }
}
