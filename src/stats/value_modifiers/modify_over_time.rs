use crate::prelude::*;
use bevy::prelude::*;



/// Modify a float value over time.
#[derive(Debug, Clone, Component)]
pub struct ModifyOverTime {
	/// Amount per second to modify.
	pub value: f32,
}

impl ModifyOverTime {
	pub fn new(value: f32) -> Self { Self { value } }
}

pub fn modify_over_time(
	time: Res<Time>,
	mut query: Query<(&mut FloatValue, &ModifyOverTime)>,
) {
	let delta = time.delta_seconds();
	for (mut value, modifier) in query.iter_mut() {
		value.0 += modifier.value * delta;
	}
}
