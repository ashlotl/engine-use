pub mod scrap_heap;

use std::collections::BTreeMap;

use serde::Deserialize;

use crate::composition::scrap_heap::ScrapHeap;

#[derive(Deserialize)]
pub struct CompositionTemplate {
	pub declarations: ScrapHeap,
	pub graph: BTreeMap<(String, String), Vec<(String, String)>>,
}
