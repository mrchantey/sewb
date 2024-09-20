use crate::spawn::spawn_on_startup;
use crate::spawn::SpawnOnStartup;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use forky::prelude::close_on_esc;

pub struct SewbPlugin;

impl Plugin for SewbPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, (setup, basics))
			.add_systems(Update, close_on_esc)
			.add_systems(Update, spawn_on_startup)
			.add_plugins(
				WorldInspectorPlugin::default()
					.run_if(input_toggle_active(true, KeyCode::KeyI)),
			);
	}
}

fn setup(mut commands: Commands) {
	commands.spawn((Name::new("Camera"), Camera3dBundle {
		transform: Transform::from_xyz(0., 5., 1.)
			.looking_at(Vec3::ZERO, Vec3::Y),
		..default()
	}));

	commands.spawn((
		Name::new("Spawner"),
		SpatialBundle::default(),
		SpawnOnStartup::default(),
	));
}

/// set up a simple 3D scene
fn basics(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	// // plane
	commands.spawn((Name::new("Ground"), PbrBundle {
		mesh: meshes.add(Plane3d::default().mesh().size(5.0, 5.0)),
		material: materials.add(Color::srgb(0., 0., 0.)),
		..default()
	}));
	// // cube
	// commands.spawn((
	//     Name::new("My Cube"),
	//     PbrBundle {
	//         mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
	//         material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
	//         transform: Transform::from_xyz(0.0, 0.5, 0.0),
	//         ..default()
	//     },
	// ));
	// light
	commands.spawn(PointLightBundle {
		point_light: PointLight {
			intensity: 2_000_000.0,
			shadows_enabled: true,
			..default()
		},
		transform: Transform::from_xyz(4.0, 8.0, 4.0),
		..default()
	});
}
