//! Helper traits for creating common widgets.

use bevy::{
    ecs::{
        relationship::{RelatedSpawnerCommands, Relationship},
        system::EntityCommands,
    },
    prelude::*,
    ui::Val::*,
};

use crate::{
    palette::{
        BUTTON_HOVERED_BACKGROUND, BUTTON_PRESSED_BACKGROUND, BUTTON_TEXT, HEADER_TEXT, LABEL_TEXT,
        NODE_BACKGROUND,
    },
    prelude::InteractionPalette,
};

/// An extension trait for spawning UI widgets.
pub trait Widgets {
    /// Spawn a simple button with text.
    fn button(&mut self, text: impl Into<String>) -> EntityCommands;

    /// Spawn a simple header label. Bigger than [`Widgets::label`].
    fn header(&mut self, text: impl Into<String>) -> EntityCommands;

    /// Spawn a simple text label.
    fn label(&mut self, text: impl Into<String>) -> EntityCommands;
}

impl<T: Spawn> Widgets for T {
    fn button(&mut self, text: impl Into<String>) -> EntityCommands {
        self.spawn((
            Name::new("Button"),
            Button,
            Node {
                width: Px(250.0),
                height: Px(65.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(NODE_BACKGROUND),
            InteractionPalette {
                none: NODE_BACKGROUND,
                hovered: BUTTON_HOVERED_BACKGROUND,
                pressed: BUTTON_PRESSED_BACKGROUND,
            },
            children![(
                Name::new("Button Text"),
                Text(text.into()),
                TextFont::from_font_size(40.0),
                TextColor(BUTTON_TEXT),
            )],
        ))
    }

    fn header(&mut self, text: impl Into<String>) -> EntityCommands {
        self.spawn((
            Name::new("Header"),
            Node {
                width: Px(500.0),
                height: Px(65.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(NODE_BACKGROUND),
            children![(
                Name::new("Header Text"),
                Text(text.into()),
                TextFont::from_font_size(40.0),
                TextColor(HEADER_TEXT),
            )],
        ))
    }

    fn label(&mut self, text: impl Into<String>) -> EntityCommands {
        self.spawn((
            Name::new("Label"),
            Text(text.into()),
            TextFont::from_font_size(24.0),
            TextColor(LABEL_TEXT),
            Node {
                width: Px(500.0),
                ..default()
            },
        ))
    }
}

/// An extension trait for spawning UI containers.
pub trait Containers {
    /// Spawns a root node that covers the full screen
    /// and centers its content horizontally and vertically.
    fn ui_root(&mut self) -> EntityCommands;
}

impl Containers for Commands<'_, '_> {
    fn ui_root(&mut self) -> EntityCommands {
        self.spawn((
            Name::new("UI Root"),
            Node {
                width: Percent(100.0),
                height: Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Px(10.0),
                position_type: PositionType::Absolute,
                ..default()
            },
        ))
    }
}

/// An internal trait for types that can spawn entities.
/// This is here so that [`Widgets`] can be implemented on all types that
/// are able to spawn entities.
/// Ideally, this trait should be [part of Bevy itself](https://github.com/bevyengine/bevy/issues/14231).
trait Spawn {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands;
}

impl Spawn for Commands<'_, '_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}

impl<R: Relationship> Spawn for RelatedSpawnerCommands<'_, R> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}
