use bevy::prelude::*;
use mrchantey_beetmash_sewb::prelude::*;
use mrchantey_beetmash_sewb::scenes;

fn main() {
	App::new()
		.add_plugins(SewbPlugin)
		.add_systems(Startup, (scenes::basics, scenes::wellbeing_inheritance))
		.run();
}
