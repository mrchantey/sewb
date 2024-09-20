use anyhow::Result;
use bevy::prelude::*;
use bevy::utils::HashMap;
use forky::prelude::OptionTExt;

pub struct AgentStatDefinitions(pub HashMap<String, AgentStatKey>);


#[derive(Component, Deref, DerefMut)]
pub struct AgentStatMap(pub HashMap<AgentStatKey, AgentStatValue>);

impl AgentStatMap {}


/// Used to define a key for an agent stat, also added to mutators like zones, so they will know which stat to modify.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Component)]
pub enum AgentStatKey {
	Bool(String),
	Float(String),
	Int(String),
	String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum AgentStatValue {
	Bool(bool),
	Float(f32),
	Int(i32),
	String(String),
}

impl AgentStatValue {
	pub fn set(&mut self, value: AgentStatValue) { *self = value; }
	pub fn add(&mut self, value: AgentStatValue) -> Result<()> {
		*self = match (&self, value) {
			(AgentStatValue::Float(a), AgentStatValue::Float(b)) => {
				AgentStatValue::Float(*a + b)
			}
			(AgentStatValue::Int(a), AgentStatValue::Int(b)) => {
				AgentStatValue::Int(*a + b)
			}
			(this, other) => {
				anyhow::bail!("Cannot add {:?} to {:?}", other, this)
			}
		};
		Ok(())
	}
	pub fn scale(&mut self, value: AgentStatValue) -> Result<()> {
		*self = match (&self, value) {
			(AgentStatValue::Float(a), AgentStatValue::Float(b)) => {
				AgentStatValue::Float(*a * b)
			}
			(AgentStatValue::Int(a), AgentStatValue::Int(b)) => {
				AgentStatValue::Int(*a * b)
			}
			(this, other) => {
				anyhow::bail!("Cannot scale {:?} by {:?}", this, other)
			}
		};
		Ok(())
	}
}


pub fn default_sewb_agent_stats(team: i32) -> AgentStatMap {
	let mut map = HashMap::default();
	map.insert(
		AgentStatKey::Float("wellbeing".to_string()),
		AgentStatValue::Float(100.0),
	);
	map.insert(
		AgentStatKey::Int("team".to_string()),
		AgentStatValue::Int(team),
	);
	AgentStatMap(map)
}


pub struct AgentStatModifer {
	key: AgentStatKey,
	value: AgentStatValue,
	operation: AgentStatOperation,
}

pub enum AgentStatOperation {
	Set,
	Add,
	Scale,
}

impl AgentStatModifer {
	pub fn apply(&self, stats: &mut AgentStatMap) -> Result<()> {
		let stat = stats.get_mut(&self.key).ok()?;
		match self.operation {
			AgentStatOperation::Set => stat.set(self.value.clone()),
			AgentStatOperation::Add => stat.add(self.value.clone())?,
			AgentStatOperation::Scale => stat.scale(self.value.clone())?,
		}
		Ok(())
	}
}
