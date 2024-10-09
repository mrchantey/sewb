use crate::prelude::*;
use beet::prelude::*;
use bevy::prelude::*;


/// Marker for Wellbeing [`FloatValue`] entity.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Wellbeing;



/// Every wellbeing point = 1 m/s max speed
pub fn wellbeing_is_speed(
	trigger: Trigger<GetFloatValue<Wellbeing>>,
	mut query: Query<&mut MaxSpeed, With<Wellbeing>>,
) {
	for mut speed in query.iter_mut() {
		**speed = *trigger.event().value;
	}
}
