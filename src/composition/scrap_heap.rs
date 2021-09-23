use serde::Deserialize;

use crate::{data::EngineDataTemplate, run::run_loop::RunLoopTemplate};

#[derive(Deserialize)]
pub struct ScrapHeap {
	pub paths: Vec<ScrapTemplate>,
}

#[derive(Deserialize)]
pub struct ScrapTemplate {
	pub name: String,
	pub run_loops: Vec<RunLoopTemplate>,
	pub data_objects: Vec<EngineDataTemplate>,
}
