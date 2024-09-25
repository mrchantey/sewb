use bevy::prelude::*;
use sewb::prelude::*;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(SewbPlugin)
		.add_systems(Startup, setup)
		.run();
}


fn setup(mut commands: Commands) {
	commands.spawn((
		Name::new("Spawner"),
		SpatialBundle::default(),
		SpawnOnStartup::default(),
	));
}
