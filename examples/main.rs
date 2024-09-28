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
pub const WELLNESS_DECREMENT_RATE: f32 = 1.;
pub const WELLNESS_DECREMENT_AMOUNT: f32 = 0.1;
/// initial wellness value
/// - starting value of 5 will decrease
/// - starting value of 10 will increase
pub const INITIAL_WELLNESS: f32 = 5.;

pub const NUM_WELLNESS_ACTIONS: usize = 10;

const FULL_WELLNESS: f32 = 10.;


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
	for _ in 0..NUM_WELLNESS_ACTIONS {
		commands.run_system(system_registry.get(spawn_collectable));
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
				LerpColor::default().with_min_max(0., FULL_WELLNESS),
				SetOverTime::new(
					Op::Sub,
					WELLNESS_DECREMENT_AMOUNT,
					Duration::from_secs_f32(WELLNESS_DECREMENT_RATE),
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

	commands.spawn((
		Name::new("Wellness Text"),
		SetText { section: 1 },
		Wellness,
		world_space_ui_text(
			agent_entity,
			vec!["Wellness: ".into(), "0".into()],
		),
	));
}
