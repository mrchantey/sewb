use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct SetText {
	pub entity: Entity,
	pub section: usize,
}


pub fn set_text_float(
	mut texts: Query<&mut Text>,
	query: Query<(&SetText, &FloatValue), Changed<FloatValue>>,
) {
	for (set_text, float_value) in query.iter() {
		let mut text = texts.get_mut(set_text.entity).unwrap();
		text.sections[set_text.section].value = format!("{:.2}", float_value.0);
	}
}
