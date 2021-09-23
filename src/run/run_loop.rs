use std::fmt::Debug;

use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct RunLoopId {
	id: u64,
	scrap_id: u64,
}

impl RunLoopId {
	pub fn new(id: u64, scrap_id: u64) -> Self {
		Self { id, scrap_id }
	}
}

pub trait RunLoop: Debug {
	fn run_loop_id(&self) -> RunLoopId;
	fn run(&self);
}

#[derive(Deserialize)]
pub struct RunLoopTemplate {
	pub name: String,
	pub type_name: String,
}
