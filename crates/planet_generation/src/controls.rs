use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{GeothermalOverlay, Planet};

#[derive(Actionlike, Clone, Debug, Eq, Hash, PartialEq, Reflect)]
pub(crate) enum PlanetActions {
    ToggleGeothermalOverlay,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct PlanetState {
    pub geothermal_overlay: bool,
}

#[derive(Event)]
struct StateChange(bool);

pub(crate) fn setup(mut commands: Commands) {
    let input_map = InputMap::new([(PlanetActions::ToggleGeothermalOverlay, KeyCode::KeyG)]);
    commands
        .spawn((
            Name::new("PlanetControls"),
            InputManagerBundle::with_map(input_map),
            PlanetState::default(),
        ))
        .observe(on_state_change);
}

pub(crate) fn check(
    query: Single<(Entity, &mut PlanetState, &ActionState<PlanetActions>)>,
    mut commands: Commands,
) {
    let (entity, mut planet_state, action_state) = query.into_inner();
    if action_state.just_pressed(&PlanetActions::ToggleGeothermalOverlay) {
        planet_state.geothermal_overlay = !planet_state.geothermal_overlay;
        commands
            .entity(entity)
            .trigger(StateChange(planet_state.geothermal_overlay));
    }
}

fn on_state_change(
    trigger: Trigger<StateChange>,
    planet: Single<&mut Visibility, (With<Planet>, Without<GeothermalOverlay>)>,
    geothermal: Single<&mut Visibility, (Without<Planet>, With<GeothermalOverlay>)>,
) {
    let state_change = trigger.event();
    let mut planet_visibility = planet.into_inner();
    let mut geothermal_visibility = geothermal.into_inner();
    match state_change.0 {
        true => {
            *planet_visibility = Visibility::Hidden;
            *geothermal_visibility = Visibility::Inherited;
        }
        false => {
            *planet_visibility = Visibility::Inherited;
            *geothermal_visibility = Visibility::Hidden;
        }
    }
}
