use crate::prelude::*;
use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use bevy::utils::HashMap;
use std::any::Any;
use std::any::TypeId;




#[derive(Debug, Default, Clone, Deref, DerefMut, Resource)]
pub struct SystemRegistry(pub HashMap<TypeId, SystemId>);


impl SystemRegistry {
	pub fn get(&self, val: impl Any) -> SystemId {
		self.0
			.get(&val.type_id())
			.copied()
			.expect("System not found")
	}
	pub fn add(&mut self, system: impl Any, system_id: SystemId) {
		self.0.insert(system.type_id(), system_id);
	}
}


pub fn system_registry_plugin(app: &mut App) {
	let mut registry = SystemRegistry::default();
	registry.add(spawn_collectable, app.register_system(spawn_collectable));
	app.insert_resource(registry);
}
