use bevy::prelude::*;
use noise::{NoiseFn, OpenSimplex};

#[derive(Reflect)]
pub(crate) struct NoiseFilter {
    #[reflect(ignore)]
    pub(crate) noise: OpenSimplex,
    pub(crate) settings: NoiseSettings,
}

impl NoiseFilter {
    pub(crate) fn evaluate(&self, point_on_unit_sphere: Vec3) -> f32 {
        let mut value = 0.0;
        let mut frequency = self.settings.base_roughness;
        let mut amplitude = 1.0;
        for _index in 0..self.settings.number_of_layers {
            let point = point_on_unit_sphere * frequency + self.settings.center;
            let noise_value = self.noise.get(point.as_dvec3().into());
            value += noise_value.remap(-1.0, 1.0, 0.0, 1.0) * amplitude;
            frequency *= self.settings.roughness;
            amplitude *= self.settings.persistence;
        }
        value = 0.0_f64.max(value - self.settings.min_value);
        value as f32 * self.settings.strength
    }
}

#[derive(Reflect)]
pub(crate) struct NoiseSettings {
    pub(crate) number_of_layers: u32,
    pub(crate) strength: f32,
    pub(crate) base_roughness: f32,
    pub(crate) roughness: f32,
    pub(crate) persistence: f64,
    pub(crate) center: Vec3,
    pub(crate) min_value: f64,
    pub(crate) use_first_layer_as_mask: bool,
}
