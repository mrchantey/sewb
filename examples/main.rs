use beet::prelude::*;
use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use sewb::prelude::*;
use std::f32::consts::PI;
use std::time::Duration;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(SewbPlugin)
		.add_systems(Startup, setup)
		.observe(wellness_is_speed)
		.run();
}

// params to expose
/// wellness decreases at amount/rate
pub const DECREMENT_RATE: f32 = 1.;
pub const DECREMENT_AMOUNT: f32 = 0.1;
pub const INITIAL_WELLNESS: f32 = 5.;



/// Every wellness point = 1 m/s max speed
fn wellness_is_speed(
	trigger: Trigger<GetFloatValue<Wellness>>,
	mut query: Query<&mut MaxSpeed, With<Wellness>>,
) {
	for mut speed in query.iter_mut() {
		**speed = *trigger.event().value;
	}
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
			Wellness,
			ForceBundle::default(),
			SteerBundle {
				max_force: MaxForce(999.),
				// max_force: MaxForce(0.1),
				max_speed: MaxSpeed(1.),
				arrive_radius: ArriveRadius(0.),
				// arrive_radius: ArriveRadius(0.25),
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
			agent.spawn((
				Name::new("Wellness Value"),
				Wellness,
				FloatValue(INITIAL_WELLNESS),
				LerpColor::default().with_min_max(0., 10.),
				SetOverTime::new(
					Op::Sub,
					DECREMENT_AMOUNT,
					Duration::from_secs_f32(DECREMENT_RATE),
				),
				TargetAgent(agent_entity),
			));

			agent.spawn((
				Name::new("Seek"),
				Running,
				TargetAgent(agent_entity),
				ContinueRun::default(),
				Seek::new(OnTargetNotFound::Clear),
			));
		})
		.id();

	let label_text_style = TextStyle {
		font_size: 25.0,
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
			parent.spawn((
				Name::new("Wellness Text"),
				SetText { section: 1 },
				Wellness,
				TextBundle::from_sections(vec![
					TextSection {
						value: "wellness: ".into(),
						style: label_text_style.clone(),
					},
					TextSection {
						value: "0".into(),
						style: label_text_style.clone(),
					},
				])
				.with_style(Style {
					position_type: PositionType::Absolute,
					bottom: Val::ZERO,
					..default()
				})
				.with_no_wrap(),
			));
		});
}
