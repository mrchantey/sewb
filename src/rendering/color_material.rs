use beetmash::prelude::MaterialPlaceholder;
use bevy::prelude::*;


pub fn color_material(col: impl Into<Color>) -> MaterialPlaceholder {
	MaterialPlaceholder::StandardMaterial {
		base_color: col.into(),
		unlit: true,
	}
}
