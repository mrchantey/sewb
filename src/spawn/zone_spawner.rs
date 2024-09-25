use crate::prelude::*;
use bevy::prelude::*;
use forky::prelude::Vec3Ext;
use std::f32::consts::PI;

#[derive(Component)]
pub struct SpawnOnStartup {
	pub num_entities: u32,
	pub bounds: Rectangle,
}

impl Default for SpawnOnStartup {
	fn default() -> Self {
		Self {
			num_entities: 10,
			bounds: Rectangle::default(),
		}
	}
}

pub const TEAM_1: i32 = 1;

pub fn spawn_on_startup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	query: Query<(&Transform, &SpawnOnStartup), Added<SpawnOnStartup>>,
) {
	for (transform, spawn) in query.iter() {
		for _ in 0..spawn.num_entities {
			let mut pos = Vec3::random_in_cube() * 2. - 1.;
			pos.x *= spawn.bounds.half_size.x;
			pos.y = 0.;
			pos.z *= spawn.bounds.half_size.y;
			pos += transform.translation;
			commands.spawn((
				Name::new("Agent"),
				default_sewb_agent_stats(TEAM_1),
				PbrBundle {
					mesh: meshes.add(Cylinder::new(0.1, 0.1).mesh()),
					material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
					transform: Transform::from_translation(pos)
						.with_rotation(Quat::from_rotation_x(PI)),
					..default()
				},
			));
		}
	}
}

pub enum Zone {
	Circle { radius: f32 },
	Rectangle { width: f32, height: f32 },
}
