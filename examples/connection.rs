use beet::prelude::*;
use bevy::prelude::*;
use forky::prelude::Vec3Ext;
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
	for i in 0..10 {
		let name = format!("Agent{}", i);
		let pos = Vec3::random_in_cube_signed().with_y(0.);
		commands
			.spawn((
				Name::new(name),
				Agent,
				PbrBundle {
					mesh: meshes.add(Cylinder::new(0.1, 0.1).mesh()),
					material: materials.add(StandardMaterial {
						base_color: Color::srgb(0.3, 0.5, 0.3),
						unlit: true,
						..default()
					}),
					transform: Transform::from_translation(pos)
						.with_rotation(Quat::from_rotation_x(PI)),
					..default()
				},
				ForceBundle::default(),
				SteerBundle::default(),
				VelocityScalar(Vec3::new(0.1, 0., 0.1)),
				GroupSteerAgent,
			))
			.with_children(|agent| {
				agent.spawn((
					RunOnSpawn,
					ContinueRun::default(),
					TargetAgent(agent.parent_entity()),
					Separate::<GroupSteerAgent>::new(1.),
					Align::<GroupSteerAgent>::new(1.),
					Cohere::<GroupSteerAgent>::new(1.),
					Wander::new(0.01),
				));
			});
	}
}


// commands
// .spawn((
// 	BundlePlaceholder::Sprite("spaceship_pack/ship_2.png".into()),
// 	Transform::from_translation(position)
// 		.with_scale(Vec3::splat(0.5)),
// 	RotateToVelocity2d,
// 	ForceBundle::default(),
// 	SteerBundle::default().scaled_to(100.),
// 	VelocityScalar(Vec3::new(1., 1., 0.)),
// 	GroupSteerAgent,
// ))
