use bevy::color::palettes::tailwind;
use bevy::prelude::*;

#[derive(Component)]
pub struct Agent;





const CONNECTION_DISTANCE: f32 = 1.0;
pub fn render_closest_connections(
	mut gizmos: Gizmos,
	query: Query<&Transform, With<Agent>>,
) {
	let dist_sq = CONNECTION_DISTANCE * CONNECTION_DISTANCE;

	for agent1 in query.iter() {
		for agent2 in query.iter() {
			if agent1 == agent2 {
				continue;
			} else if (agent1.translation - agent2.translation).length_squared()
				> dist_sq
			{
				continue;
			}

			gizmos.line(
				agent1.translation,
				agent2.translation,
				tailwind::RED_700,
			);
		}
	}
}
pub fn render_all_connections(
	mut gizmos: Gizmos,
	query: Query<&Transform, With<Agent>>,
) {
	for agent1 in query.iter() {
		for agent2 in query.iter() {
			if agent1 == agent2 {
				continue;
			}
			gizmos.line(
				agent1.translation,
				agent2.translation,
				tailwind::RED_700,
			);
		}
	}
}
