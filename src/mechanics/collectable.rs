use crate::prelude::*;
use crate::utils::DespawnChain;
use beetmash::prelude::BundlePlaceholder;
use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use forky::prelude::Vec3Ext;
use std::f32::consts::PI;


pub fn collectable_plugin(app: &mut App) { app.add_systems(Update, collect); }

#[derive(Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Collecter;



#[derive(Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Collectable;


pub fn collect(
	mut commands: Commands,
	registry: Res<SystemRegistry>,
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
				commands.trigger(SetFloatValue::<Wellness>::new(Op::Add, 0.1));
				commands.run_system(registry.get(spawn_collectable));
				commands.entity(entity).despawn();
			}
		}
	}
}


pub fn spawn_collectable(mut commands: Commands) {
	let pos = Vec3::random_in_cube_signed().with_y(0.) * 20.;
	let agent = commands
		.spawn((
			Collectable,
			Name::new("Collectable"),
			BundlePlaceholder::Pbr {
				mesh: cnst::COLLECTABLE_BOUNDS.into(),
				material: color_material(tailwind::EMERALD_500),
			},
			Transform::from_translation(pos)
				.with_rotation(Quat::from_rotation_x(PI)),
			Visibility::Hidden,
			DelayVisibility::three_frames(),
		))
		.id();
	commands.spawn((
		Name::new("Collectable Label"),
		world_space_ui_text(agent, vec!["A walk in the park".to_string()]),
		DespawnChain(agent),
	));

	// commands.entity(agent).observe(
	// 	move |_: Trigger<OnRemove, Collectable>, mut commands: Commands| {
	// 		commands.entity(label).despawn();
	// 	},
	// );
}
