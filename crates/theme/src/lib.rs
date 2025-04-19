pub mod interaction;
pub mod palette;
pub mod widgets;

pub mod prelude {
    pub use super::{
        interaction::InteractionPalette,
        palette as ui_palette,
        widgets::{Containers as _, Widgets as _},
    };
}

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(interaction::plugin);
}
