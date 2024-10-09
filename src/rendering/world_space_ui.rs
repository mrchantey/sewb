use super::DelayVisibility;
use beet::prelude::*;
use beetmash::prelude::BundlePlaceholder;
use bevy::prelude::*;
use bevy::text::LineBreak;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct WorldSpaceUi;



pub fn world_space_ui(
	transforms: Query<&GlobalTransform>,
	camera: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
	mut labels: Query<(&mut Style, &TargetAgent), With<WorldSpaceUi>>,
) {
	let (camera, camera_global_transform) = camera.single();

	for (mut style, agent) in labels.iter_mut() {
		let Ok(transform) = transforms.get(**agent) else {
			continue;
		};

		let pos = transform.translation() + Vec3::new(-2., 0., -2.);

		let viewport_position = camera
			.world_to_viewport(camera_global_transform, pos)
			.unwrap();

		style.top = Val::Px(viewport_position.y);
		style.left = Val::Px(viewport_position.x);
	}
}


/// Returns a `WorldSpaceUi` component with a `TextBundle`
/// that has sensible styles.
pub fn world_space_ui_text(
	target_agent: Entity,
	sections: impl IntoIterator<Item = String>,
) -> impl Bundle {
	let label_text_style = TextStyle {
		font_size: 25.0,
		..default()
	};

	(
		DelayVisibility::three_frames(),
		TargetAgent(target_agent),
		WorldSpaceUi,
		BundlePlaceholder::Text {
			sections: sections
				.into_iter()
				.map(|text| TextSection {
					value: text,
					style: label_text_style.clone(),
				})
				.collect(),
			style: Style {
				position_type: PositionType::Absolute,
				bottom: Val::ZERO,
				..default()
			},
			visibility: Visibility::Hidden,
			linebreak: Some(LineBreak::NoWrap),
		},
	)
}
