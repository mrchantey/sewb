use beet::prelude::*;
use bevy::color::palettes::tailwind;
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
	system_registry: Res<SystemRegistry>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	for _ in 0..10 {
		commands
			.run_system(system_registry[&SystemRegistryKey::SpawnCollectable]);
	}


	let agent_entity = commands
		.spawn((
			Name::new("Agent"),
			Collecter,
			ForceBundle::default(),
			SteerBundle {
				max_force: MaxForce(0.1),
				arrive_radius: ArriveRadius(0.25),
				..default()
			}
			.scaled_dist(10.),
			VelocityScalar(Vec3::new(1., 0., 1.)),
			TargetNearest::<Collectable>::default(),
			PbrBundle {
				mesh: meshes.add(cnst::AGENT_BOUNDS.mesh()),
				material: materials.add(color_material(tailwind::BLUE_500)),
				transform: Transform::default()
					.with_rotation(Quat::from_rotation_x(PI)),
				..default()
			},
		))
		.with_children(|agent| {
			let agent_entity = agent.parent_entity();
			// agent.spawn((
			// 	Name::new("Wellness"),
			// 	FloatValue(0.),
			// 	LerpColor::default(),
			// 	ModifyOverTime::new(0.1),
			// 	TargetAgent(agent_entity),
			// ));

			// agent
			// 	.spawn((Name::new("Behavior"), RunOnSpawn, Repeat::default()))
			// 	.with_children(|parent| {
			agent.spawn((
				Name::new("Seek"),
				Running,
				TargetAgent(agent_entity),
				ContinueRun::default(),
				Seek::new(OnTargetNotFound::Clear),
			));
		})
		.id();
	// });

	let label_text_style = TextStyle {
		// font: asset_server.load("fonts/FiraMono-Medium.ttf"),
		font_size: 25.0,
		// color: ORANGE.into(),
		..default()
	};


	commands
		.spawn((
			Name::new("Label"),
			TargetAgent(agent_entity),
			WorldSpaceUi,
			NodeBundle {
				style: Style {
					position_type: PositionType::Absolute,
					..default()
				},
				..default()
			},
		))
		.with_children(|parent| {
			parent.spawn(
				TextBundle::from_section("Foo", label_text_style.clone())
					.with_style(Style {
						position_type: PositionType::Absolute,
						bottom: Val::ZERO,
						..default()
					})
					.with_no_wrap(),
			);
		});
}
