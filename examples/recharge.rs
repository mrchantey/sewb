use bevy::prelude::*;
use sewb::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SewbPlugin)
        .run();
}
