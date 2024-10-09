use bevy::ecs::entity::MapEntities;
use bevy::ecs::reflect::ReflectMapEntities;
use bevy::prelude::*;


#[derive(Component, Reflect)]
#[reflect(Component, MapEntities)]
pub struct DespawnChain(pub Entity);

impl MapEntities for DespawnChain {
	fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
		self.0 = entity_mapper.map_entity(self.0);
	}
}


pub fn despawn_chain_plugin(app: &mut App) {
	app.world_mut()
		.register_component_hooks::<DespawnChain>()
		.on_add(|mut world, entity, _| {
			let target = world.entity(entity).get::<DespawnChain>().unwrap().0;
			world.commands().entity(target).observe(
				move |_: Trigger<OnRemove, Name>, mut commands: Commands| {
					commands.entity(entity).despawn();
				},
			);
		});
}
