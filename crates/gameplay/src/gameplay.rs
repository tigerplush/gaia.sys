use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((tutorial::plugin, planet_generation::plugin));
}
