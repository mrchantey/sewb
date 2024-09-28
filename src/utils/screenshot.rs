use bevy::prelude::*;
use bevy::render::view::screenshot::ScreenshotManager;
use bevy::window::PrimaryWindow;
use std::path::PathBuf;

/// https://bevyengine.org/examples/window/screenshot/
pub fn screenshot_on_spacebar(
	input: Res<ButtonInput<KeyCode>>,
	main_window: Query<Entity, With<PrimaryWindow>>,
	mut screenshot_manager: ResMut<ScreenshotManager>,
	mut counter: Local<u32>,
) {
	if input.just_pressed(KeyCode::Space) {
		// Create the screenshots directory if it doesn't exist
		let path = PathBuf::from("screenshots");
		if !path.exists() {
			std::fs::create_dir(&path).unwrap();
		}
		*counter += 1;
		screenshot_manager
			.save_screenshot_to_disk(
				main_window.single(),
				path.join(format!("screenshot-{}.png", *counter)),
			)
			.unwrap();
	}
}
