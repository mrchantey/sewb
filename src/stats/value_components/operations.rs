use bevy::prelude::*;

pub trait Operation<T> {
	/// The operation to apply to the value.
	fn apply(&self, lhs: &mut T, rhs: f32);
}


#[derive(Debug, Default, Clone, Reflect)]
pub enum Op {
	#[default]
	Add,
	Sub,
	Mul,
	Div,
}
impl Operation<f32> for Op {
	fn apply(&self, lhs: &mut f32, rhs: f32) {
		match self {
			Op::Add => *lhs += rhs,
			Op::Sub => *lhs -= rhs,
			Op::Mul => *lhs *= rhs,
			Op::Div => *lhs /= rhs,
		}
	}
}
