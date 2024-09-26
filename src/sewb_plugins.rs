use crate::prelude::*;
use beet::prelude::*;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use forky::prelude::close_on_esc;

pub struct SewbPlugin;

impl Plugin for SewbPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, basics)
			.add_systems(Update, close_on_esc)
			.add_systems(Update, spawn_on_startup)
			.add_systems(Update, render_closest_connections)
			// .add_systems(Update, render_all_connections)
			.add_plugins(BeetDefaultPlugins)
			.add_plugins(stats_plugin)
			.add_plugins(
				WorldInspectorPlugin::default()
					.run_if(input_toggle_active(true, KeyCode::KeyI)),
			);

		let mut config_store =
			app.world_mut().resource_mut::<GizmoConfigStore>();
		let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
		config.line_width = 10.0;
	}
}

fn basics(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands.spawn((Name::new("Camera"), Camera3dBundle {
		transform: Transform::from_xyz(0., 5., 0.01)
			.looking_at(Vec3::ZERO, Vec3::Y),
		..default()
	}));
	commands.spawn((Name::new("Ground"), PbrBundle {
		mesh: meshes.add(Plane3d::default().mesh().size(5.0, 5.0)),
		material: materials.add(StandardMaterial {
			base_color: Color::srgb(0., 0., 0.),
			unlit: true,
			..default()
		}),
		..default()
	}));
	commands.spawn((Name::new("Light"), PointLightBundle {
		point_light: PointLight {
			intensity: 2_000_000.0,
			shadows_enabled: true,
			..default()
		},
		transform: Transform::from_xyz(4.0, 8.0, 4.0),
		..default()
	}));
}
