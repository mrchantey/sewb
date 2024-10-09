use crate::prelude::*;
use beet::prelude::TargetAgent;
use bevy::color::palettes::tailwind;
use bevy::prelude::*;


/// When the [FloatValue] changes, update the color
/// of the [StandardMaterial] belonging to the [TargetAgent]
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct LerpColor {
	pub from: Hsla,
	pub to: Hsla,
	pub min: f32,
	pub max: f32,
}

impl Default for LerpColor {
	fn default() -> Self {
		Self {
			to: tailwind::SKY_500.into(),
			from: tailwind::RED_500.into(),
			min: 0.,
			max: 1.,
		}
	}
}

impl LerpColor {
	pub fn new(from: Hsla, to: Hsla) -> Self {
		Self {
			from,
			to,
			..default()
		}
	}
	pub fn with_min_max(mut self, min: f32, max: f32) -> Self {
		self.min = min;
		self.max = max;
		self
	}

	pub fn lerp(&self, t: f32) -> Hsla {
		// Clamp t to the min and max
		let t = t.clamp(self.min, self.max);

		// Normalize t to be between 0 and 1
		let t = (t - self.min) / (self.max - self.min);

		self.from.mix(&self.to, t)
	}
}

pub fn lerp_color(
	mut materials: ResMut<Assets<StandardMaterial>>,
	actions: Query<
		(&FloatValue, &LerpColor, &TargetAgent),
		Changed<FloatValue>,
	>,
	agents: Query<&Handle<StandardMaterial>>,
) {
	for action in actions.iter() {
		let (value, lerp_color, target) = action;
		let lerp_color = lerp_color;
		if let Ok(handle) = agents.get(**target) {
			if let Some(material) = materials.get_mut(handle) {
				material.base_color = lerp_color.lerp(**value).into();
			}
		}
	}
}
