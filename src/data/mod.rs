use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct EngineDataTemplate {
	pub name: String,
	pub accessors: BTreeMap<
		(
			String, //scrap name
			String, //RunLoop name
		),
		bool, //referenced scrap has mutable = true/immutable = false access
	>,
}
