use bevy::prelude::*;
use common::states::Screen;

pub fn plugin(app: &mut App) {
    app.init_state::<Screen>()
        .enable_state_scoped_entities::<Screen>()
        .add_plugins((
            splash::plugin,
            loading::plugin,
            title::plugin,
            gameplay::plugin,
            credits::plugin,
        ));
}
