use crate::prelude::*;
use bevy::utils::HashMap;

pub fn default_sewb_agent_stats(team: i32) -> AgentStatMap {
	let mut map = HashMap::default();
	map.insert("wellbeing".into(), AgentStatValue::Float(100.0));
	map.insert("team".into(), AgentStatValue::Int(team));
	AgentStatMap(map)
}
