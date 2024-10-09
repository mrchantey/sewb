use crate::prelude::color_material;
use beetmash::prelude::BundlePlaceholder;
use bevy::color::palettes::tailwind;
use bevy::prelude::*;

pub fn basics(mut commands: Commands) {
	commands.spawn((
		Name::new("Camera"),
		BundlePlaceholder::Camera3d,
		Transform::from_xyz(0., 50., 0.01).looking_at(Vec3::ZERO, Vec3::Y),
	));
	commands.spawn((
		Name::new("Ground"),
		BundlePlaceholder::Pbr {
			mesh: Plane3d::new(Vec3::Y, Vec2::splat(50.)).into(),
			material: color_material(tailwind::SLATE_600),
		},
	));
	commands.spawn((
		Name::new("Light"),
		BundlePlaceholder::PointLight,
		// PointLight {
		// 	intensity: 2_000_000.0,
		// 	shadows_enabled: true,
		// 	..default()
		// },
		Transform::from_xyz(4.0, 8.0, 4.0),
	));
}
