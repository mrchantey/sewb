use bevy::prelude::*;

/// Multipurpose wrapper for a float value
#[derive(Debug, Clone, PartialEq, PartialOrd, Component, Deref, DerefMut)]
pub struct FloatValue(pub f32);
