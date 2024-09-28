use crate::prelude::*;
use bevy::prelude::*;





pub fn stats_plugin(app: &mut App) {
	app.add_systems(Update, (set_over_time, lerp_color));
}
