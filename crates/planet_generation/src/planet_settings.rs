use bevy::{color::palettes::css::BLUE, prelude::*};
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;

use crate::noise_filter::NoiseFilter;

#[derive(InspectorOptions, Reflect, Resource)]
#[reflect(InspectorOptions, Resource)]
pub(crate) struct PlanetSettings {
    #[inspector(min = 2, max = 255)]
    pub(crate) resolution: u32,
    pub(crate) color: Color,
    pub(crate) radius: f32,
    pub(crate) noise_filters: Vec<NoiseFilter>,
}

impl PlanetSettings {
    pub(crate) fn calculate_point_on_planet(&self, point_on_unit_sphere: Vec3) -> Vec3 {
        let mut elevation = 0.0;
        let mut first_layer_value = 0.0;
        if let Some(filter) = self.noise_filters.first() {
            first_layer_value = filter.evaluate(point_on_unit_sphere);
            elevation = first_layer_value;
        }
        for filter in self.noise_filters.iter().skip(1) {
            let mask = if filter.settings.use_first_layer_as_mask {
                first_layer_value
            } else {
                1.0
            };
            elevation += filter.evaluate(point_on_unit_sphere) * mask;
        }
        point_on_unit_sphere * self.radius * (1.0 + elevation)
    }

    pub(crate) fn with_layer(mut self, layer: NoiseFilter) -> Self {
        self.noise_filters.push(layer);
        self
    }
}

impl Default for PlanetSettings {
    fn default() -> Self {
        Self {
            resolution: 100,
            color: BLUE.into(),
            radius: 1.0,
            noise_filters: Vec::new(),
        }
    }
}
