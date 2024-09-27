use beet::prelude::*;
use bevy::prelude::*;



#[derive(Component)]
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

		let pos = transform.translation() + Vec3::Z * -0.8;

		let viewport_position = camera
			.world_to_viewport(camera_global_transform, pos)
			.unwrap();

		style.top = Val::Px(viewport_position.y);
		style.left = Val::Px(viewport_position.x);
	}
}
