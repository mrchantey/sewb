use crate::prelude::*;
use bevy::prelude::*;
use std::marker::PhantomData;

/// Multipurpose wrapper for a float value
#[derive(
	Debug, Clone, PartialEq, PartialOrd, Component, Deref, DerefMut, Reflect,
)]
#[reflect(Component)]
pub struct FloatValue(pub f32);


/// Global trigger emitted when a [`FloatValue`] with this marker changes
#[derive(Debug, Clone, Event, Reflect)]
pub struct GetFloatValue<M> {
	pub value: FloatValue,
	#[reflect(ignore)]
	phantom: PhantomData<M>,
}


pub fn get_float_value<M: Component>(
	mut commands: Commands,
	query: Query<&FloatValue, (With<M>, Changed<FloatValue>)>,
) {
	for value in query.iter() {
		commands.trigger(GetFloatValue::<M> {
			value: value.clone(),
			phantom: default(),
		});
	}
}

/// Global trigger emitted to set every [`FloatValue`] with this marker
#[derive(Debug, Clone, Event, Reflect)]
pub struct SetFloatValue<M> {
	pub op: Op,
	pub value: f32,
	phantom: PhantomData<M>,
}

impl<M> SetFloatValue<M> {
	pub fn apply(&self, value: &mut FloatValue) {
		self.op.apply(&mut value.0, self.value);
	}

	pub fn new(op: Op, value: f32) -> Self {
		Self {
			op,
			value,
			phantom: default(),
		}
	}
}

pub fn set_float_value<M: Component>(
	trigger: Trigger<SetFloatValue<M>>,
	mut query: Query<&mut FloatValue, With<M>>,
) {
	for mut value in query.iter_mut() {
		trigger.event().apply(value.as_mut());
	}
}
