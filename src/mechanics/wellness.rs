use crate::prelude::*;
use beet::prelude::*;
use bevy::prelude::*;


/// Marker for Wellness [`FloatValue`] entity.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Wellness;



/// Every wellness point = 1 m/s max speed
pub fn wellness_is_speed(
	trigger: Trigger<GetFloatValue<Wellness>>,
	mut query: Query<&mut MaxSpeed, With<Wellness>>,
) {
	for mut speed in query.iter_mut() {
		**speed = *trigger.event().value;
	}
}
