pub mod cnst {
	use bevy::prelude::*;


	pub const AGENT_BOUNDS: Cylinder = Cylinder {
		radius: 0.5,
		half_height: 0.002,
	};

	pub const COLLECTABLE_BOUNDS: Cylinder = Cylinder {
		radius: 0.5,
		half_height: 0.001,
	};
}
