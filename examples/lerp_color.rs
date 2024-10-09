use beet::prelude::TargetAgent;
use bevy::prelude::*;
use mrchantey_beetmash_sewb::prelude::*;
use std::f32::consts::PI;
use std::time::Duration;

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
	commands
		.spawn((
			Name::new("Agent"),
			Mesh3d(meshes.add(Cylinder::new(0.1, 0.1).mesh())),
			MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
			Transform::default().with_rotation(Quat::from_rotation_x(PI)),
		))
		.with_children(|parent| {
			parent.spawn((
				Name::new("Wellness"),
				FloatValue(0.),
				LerpColor::default(),
				SetOverTime::new(Op::Sub, 0.1, Duration::from_secs(1)),
				TargetAgent(parent.parent_entity()),
			));
		});
}
