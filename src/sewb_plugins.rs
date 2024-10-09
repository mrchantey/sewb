use crate::prelude::*;
use beet::prelude::*;
use beetmash::prelude::*;
use bevy::prelude::*;
// use bevy::input::common_conditions::input_toggle_active;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use forky::prelude::close_on_esc;
use bevy::prelude::TransformSystem::TransformPropagate;

pub struct SewbPlugin;


pub fn register_types(app:&mut App){
	app/*-*/
	.register_type::<Wellness>()
	.register_type::<Collecter>()
	.register_type::<Collectable>()
	.register_type::<TargetNearest::<Collectable>>()
	.register_type::<FloatValue>()
	.register_type::<SetOverTime>()
	.register_type::<DespawnChain>()
	.register_type::<SetText>()
	.register_type::<DelayVisibility>()
	.register_type::<WorldSpaceUi>()
	.register_type::<LerpColor>()
	/*-*/;

}

impl Plugin for SewbPlugin {
	fn build(&self, app: &mut App) {
		app/*-*/
		.add_plugins((
			stats_plugin,
			system_registry_plugin, 
			collectable_plugin,
			delay_visibility_plugin,
			despawn_chain_plugin,
			BeetmashDefaultPlugins::default(),
			DefaultPlaceholderPlugin,
			DefaultReplicatePlugin,
			BeetDefaultPlugins,
			DebugGroupSteerPlugin::<GroupSteerAgent>::default(),
			register_types,
		))
		// .add_systems(Update, render_all_connections)
		// .add_plugins(
		// 	WorldInspectorPlugin::default()
		// 	.run_if(input_toggle_active(false, KeyCode::KeyI)),
		// )
		.add_systems(Update, (
			get_float_value::<Wellness>,
			close_on_esc,
			render_closest_connections,
			target_nearest::<Collectable>.in_set(PreTickSet),
		))
		.add_systems(PostUpdate, 
			world_space_ui.after(TransformPropagate)
		)
		.observe(set_float_value::<Wellness>)
		.observe(set_text_float::<Wellness>)
		.observe(wellness_is_speed)
			/*-*/;

		let mut config_store =
			app.world_mut().resource_mut::<GizmoConfigStore>();
		let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
		config.line_width = 10.0;
	}
}
