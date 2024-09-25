use bevy::color::palettes::tailwind;
use bevy::prelude::*;

#[derive(Component)]
pub struct Agent;




pub fn render_connection(
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
