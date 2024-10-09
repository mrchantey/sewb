use bevy::prelude::*;
use std::time::Duration;



pub fn delay_visibility_plugin(app: &mut App) {
	app.add_systems(Update, delay_visibility);
}



#[derive(Component,Reflect)]
#[reflect(Component)]
pub struct DelayVisibility {
	timer: Timer,
}

impl DelayVisibility {
	pub fn new(delay: Duration) -> Self {
		Self {
			timer: Timer::new(delay, TimerMode::Once),
		}
	}

	pub fn one_frame() -> Self { Self::new(Duration::from_secs(0)) }
}

pub fn delay_visibility(
	time: Res<Time>,
	mut commands: Commands,
	mut query: Query<(Entity, &mut Visibility, &mut DelayVisibility)>,
) {
	for (entity, mut visibility, mut delay) in query.iter_mut() {
		delay.timer.tick(time.delta());
		if delay.timer.finished() {
			*visibility = Visibility::Inherited;
			commands.entity(entity).remove::<DelayVisibility>();
		}
	}
}
