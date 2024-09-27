use bevy::prelude::*;


pub fn color_material(col: impl Into<Color>) -> StandardMaterial {
	StandardMaterial {
		base_color: col.into(),
		unlit: true,
		..Default::default()
	}
}
