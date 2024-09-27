use crate::prelude::*;
use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use bevy::utils::HashMap;




#[derive(Debug, Default, Clone, Deref, DerefMut, Resource)]
pub struct SystemRegistry(pub HashMap<SystemRegistryKey, SystemId>);



pub fn system_registry_plugin(app: &mut App) {
	let mut registry = SystemRegistry::default();
	registry.insert(
		SystemRegistryKey::SpawnCollectable,
		app.register_system(spawn_collectable),
	);
	app.insert_resource(registry);
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum SystemRegistryKey {
	SpawnCollectable,
}
