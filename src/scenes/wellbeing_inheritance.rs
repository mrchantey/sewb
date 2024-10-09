use crate::prelude::*;
use beet::prelude::*;
use beetmash::prelude::BundlePlaceholder;
use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use std::f32::consts::PI;
use std::time::Duration;


// params to expose
/// wellbeing decreases at amount/rate
pub const WELLBEING_DECREMENT_RATE: f32 = 1.;
pub const WELLBEING_DECREMENT_AMOUNT: f32 = 0.1;
/// initial wellbeing value
/// - starting value of 5 will decrease
/// - starting value of 10 will increase
pub const INITIAL_WELLBEING: f32 = 10.;

pub const NUM_WELLBEING_ACTIONS: usize = 10;

const FULL_WELLBEING: f32 = 10.;



pub fn wellbeing_inheritance(
	mut commands: Commands,
	system_registry: Res<SystemRegistry>,
) {
	for _ in 0..NUM_WELLBEING_ACTIONS {
		commands.run_system(system_registry.get(spawn_collectable));
	}

	let agent_entity = commands
		.spawn((
			Name::new("Agent"),
			Collecter,
			Wellbeing,
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
			BundlePlaceholder::Pbr {
				mesh: cnst::AGENT_BOUNDS.into(),
				material: color_material(tailwind::BLUE_500),
			},
			Transform::default().with_rotation(Quat::from_rotation_x(PI)),
		))
		.with_children(|agent| {
			let agent_entity = agent.parent_entity();
			agent.spawn((
				Name::new("Wellbeing Value"),
				Wellbeing,
				FloatValue(INITIAL_WELLBEING),
				LerpColor::default().with_min_max(0., FULL_WELLBEING),
				SetOverTime::new(
					Op::Sub,
					WELLBEING_DECREMENT_AMOUNT,
					Duration::from_secs_f32(WELLBEING_DECREMENT_RATE),
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
		Name::new("Wellbeing Text"),
		SetText { section: 1 },
		Wellbeing,
		world_space_ui_text(
			agent_entity,
			vec!["Wellbeing: ".into(), "0".into()],
		),
	));
}
