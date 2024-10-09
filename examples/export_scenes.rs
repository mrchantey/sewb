use anyhow::Result;
use beet::prelude::*;
use beetmash::prelude::*;
use mrchantey_beetmash_sewb::prelude::*;
use mrchantey_beetmash_sewb::scenes;

fn main() -> Result<()> {
	SceneGroupExporter::new((
		MostDefaultPlugins,
		BeetDefaultPlugins,
		DefaultPlaceholderPlugin,
		register_types,
		system_registry_plugin,
		temp_patches,
	))
	.add_ignored_resources(2)
	.add_scene("basics", scenes::basics)
	.add_scene("wellbeing_inheritance", scenes::wellbeing_inheritance)
	.export()?;

	Ok(())
}
