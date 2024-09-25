use crate::prelude::*;
use beet::prelude::TargetAgent;
use bevy::color::palettes::basic;
use bevy::prelude::*;


/// When the [FloatValue] changes, update the color
/// of the [StandardMaterial] belonging to the [TargetAgent]
#[derive(Component)]
pub struct LerpColor {
	pub from: Srgba,
	pub to: Srgba,
	// pub min: f32,
	// pub max: f32,
}

impl Default for LerpColor {
	fn default() -> Self {
		Self {
			from: basic::RED,
			to: basic::BLUE,
			// min: 0.,
			// max: 1.,
		}
	}
}

impl LerpColor {
	pub fn new(from: Srgba, to: Srgba) -> Self { Self { from, to } }

	/// Linearly interpolates between two sRGBA colors.
	pub fn lerp(&self, t: f32) -> Srgba {
		// Clamp t to the range [0.0, 1.0]
		let t = t.clamp(0.0, 1.0);

		// Interpolate each component
		let red = self.from.red + t * (self.to.red - self.from.red);
		let green = self.from.green + t * (self.to.green - self.from.green);
		let blue = self.from.blue + t * (self.to.blue - self.from.blue);
		let alpha = self.from.alpha + t * (self.to.alpha - self.from.alpha);

		Srgba {
			red,
			green,
			blue,
			alpha,
		}
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
