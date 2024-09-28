use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct SetText {
	pub section: usize,
}


pub fn set_text_float<M: Component>(
	trigger: Trigger<GetFloatValue<M>>,
	mut texts: Query<(&mut Text, &SetText), With<M>>,
) {
	for (mut text, set_text) in texts.iter_mut() {
		text.sections[set_text.section].value =
			format!("{:.2}", *trigger.event().value);
	}
}
