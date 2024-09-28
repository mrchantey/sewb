use crate::prelude::*;
use beet::prelude::*;
use bevy::color::palettes::tailwind;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use forky::prelude::close_on_esc;
use bevy::prelude::TransformSystem::TransformPropagate;

pub struct SewbPlugin;

impl Plugin for SewbPlugin {
	fn build(&self, app: &mut App) {
		app 
		/*-*/
		.add_plugins((
			stats_plugin,
			system_registry_plugin, 
			collectable_plugin,
			delay_visibility_plugin,
		))
		// .add_systems(Update, render_all_connections)
		.add_plugins(BeetDefaultPlugins)
		.add_plugins(DebugGroupSteerPlugin::<GroupSteerAgent>::default())
		.add_plugins(
			WorldInspectorPlugin::default()
			.run_if(input_toggle_active(false, KeyCode::KeyI)),
		)
		.add_systems(Startup, basics)
		.add_systems(Update, (
			get_float_value::<Wellness>,
			close_on_esc,
			screenshot_on_spacebar,
			render_closest_connections,
			target_nearest::<Collectable>.in_set(PreTickSet),
		))
		.add_systems(PostUpdate, 
			world_space_ui.after(TransformPropagate)
		)
		.observe(set_float_value::<Wellness>)
		.observe(set_text_float::<Wellness>)
			/*-*/;

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
	commands.spawn((
		Name::new("Camera"),
		Camera3dBundle {
			transform: Transform::from_xyz(0., 50., 0.01)
				.looking_at(Vec3::ZERO, Vec3::Y),
			..default()
		},
	));
	commands.spawn((
		Name::new("Ground"),
		PbrBundle {
			mesh: meshes.add(Plane3d::default().mesh().size(100.0, 100.0)),
			material: materials.add(color_material(tailwind::SLATE_600)),
			..default()
		},
	));
	commands.spawn((
		Name::new("Light"),
		PointLightBundle {
			point_light: PointLight {
				intensity: 2_000_000.0,
				shadows_enabled: true,
				..default()
			},
			transform: Transform::from_xyz(4.0, 8.0, 4.0),
			..default()
		},
	));
}
