use beet::prelude::*;
use bevy::prelude::*;
use std::marker::PhantomData;




/// Place this component on an agent with behaviors that expect a
/// [`SteerTarget`]. If the agent does not have a [`SteerTarget`], it will be
/// set to the nearest entity with the provided component.
#[derive(Debug, PartialEq, Component, Reflect)]
#[reflect(Default, Component)]
pub struct TargetNearest<T: Component> {
	phantom: PhantomData<T>,
}

impl<T: Component> Default for TargetNearest<T> {
	fn default() -> Self {
		Self {
			phantom: PhantomData,
		}
	}
}

pub fn target_nearest<T: Component>(
	mut commands: Commands,
	transforms: Query<(Entity, &GlobalTransform), With<T>>,
	query: Query<
		(Entity, &GlobalTransform),
		(With<TargetNearest<T>>, Without<SteerTarget>),
	>,
) {
	for (agent_entity, agent_transform) in query.iter() {
		let mut targets = transforms
			.iter()
			.map(|(entity, transform)| {
				(
					entity,
					agent_transform
						.translation()
						.distance_squared(transform.translation()),
				)
			})
			.collect::<Vec<_>>();
		targets.sort_by(|a, b| {
			a.1.partial_cmp(&b.1).expect("distance squared failed")
		});

		if let Some((target_entity, _)) = targets.first() {
			commands
				.entity(agent_entity)
				.insert(SteerTarget::Entity(*target_entity));
		}
	}
}
