use bevy::prelude::*;
use sewb::prelude::*;
use std::f32::consts::PI;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(SewbPlugin)
		.add_systems(Startup, setup)
		.run();
}

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands.spawn((Name::new("Agent1"), Agent, PbrBundle {
		mesh: meshes.add(Cylinder::new(0.1, 0.1).mesh()),
		material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
		transform: Transform::from_xyz(-1., 0., 0.)
			.with_rotation(Quat::from_rotation_x(PI)),
		..default()
	}));
	commands.spawn((Name::new("Agent2"), Agent, PbrBundle {
		mesh: meshes.add(Cylinder::new(0.1, 0.1).mesh()),
		material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
		transform: Transform::from_xyz(1., 0., 0.)
			.with_rotation(Quat::from_rotation_x(PI)),
		..default()
	}));
}
