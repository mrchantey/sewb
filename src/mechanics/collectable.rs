use crate::prelude::*;
use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use forky::prelude::Vec3Ext;
use std::f32::consts::PI;


pub fn collectable_plugin(app: &mut App) { app.add_systems(Update, collect); }

#[derive(Debug, Clone, Component)]
pub struct Collecter;



#[derive(Debug, Clone, Component)]
pub struct Collectable;


pub fn collect(
	mut commands: Commands,
	registry: Res<SystemRegistry>,
	mut values: Query<(&Name, &mut FloatValue)>,
	collectors: Query<&Transform, With<Collecter>>,
	collectables: Query<(Entity, &Transform), With<Collectable>>,
) {
	let collectable_radius_sq = cnst::AGENT_BOUNDS.radius.powi(2) * 2.;
	for collector in collectors.iter() {
		for (entity, collectable) in collectables.iter() {
			if collector
				.translation
				.distance_squared(collectable.translation)
				<= collectable_radius_sq
			{
				for (name, mut value) in values.iter_mut() {
					if name.as_str() == "Wellness" {
						value.0 += 0.1;
					}
				}

				println!("Collected {:?}", entity);
				commands
					.run_system(registry[&SystemRegistryKey::SpawnCollectable]);
				commands.entity(entity).despawn();
			}
		}
	}
}


pub fn spawn_collectable(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let pos = Vec3::random_in_cube_signed().with_y(0.) * 20.;
	commands.spawn((
		Collectable,
		Name::new("Collectable"),
		PbrBundle {
			mesh: meshes.add(cnst::COLLECTABLE_BOUNDS.mesh()),
			material: materials.add(color_material(tailwind::AMBER_500)),
			transform: Transform::from_translation(pos)
				.with_rotation(Quat::from_rotation_x(PI)),
			..default()
		},
	));
}
