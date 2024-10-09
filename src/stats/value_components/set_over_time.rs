use crate::prelude::*;
use bevy::prelude::*;
use std::time::Duration;



/// Modify a float value over time.
#[derive(Debug, Clone, Component,Reflect)]
#[reflect(Component)]
pub struct SetOverTime {
	/// Amount to modify.
	pub value: f32,
	pub op: Op,
	pub timer: Timer,
}

impl SetOverTime {
	pub fn new(op: Op, value: f32, interval: Duration) -> Self {
		Self {
			op,
			value,
			timer: Timer::new(interval, TimerMode::Repeating),
		}
	}
}

pub fn set_over_time(
	time: Res<Time>,
	mut query: Query<(&mut FloatValue, &mut SetOverTime)>,
) {
	for (mut value, mut setter) in query.iter_mut() {
		if setter.timer.tick(time.delta()).just_finished() {
			setter.op.apply(&mut value.0, setter.value);
		}
	}
}
