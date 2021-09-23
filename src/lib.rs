pub mod composition;
pub mod data;
pub mod run;
pub mod sync;

use std::{collections::BTreeMap, error::Error, fs, path::PathBuf};

use libloading::Library;

use parking_lot::{Mutex, MutexGuard};

use crate::{
	composition::CompositionTemplate,
	run::run_loop::{RunLoop, RunLoopId},
};

pub const COMPOSITION_PATH: &'static str = "composition.ron";
pub const RELOAD_PATH: &'static str = "_reload.ron";
pub const SCRAP_HEAP_PATH: &'static str = "scrap_heap";

//TODO: extra platforms with cfg
pub const LIB_PREFIX: &'static str = "target/debug/lib";
pub const LIB_POSTFIX: &'static str = ".so";

static mut SCRAP_DATA: Option<Mutex<BTreeMap<String, (Library, Vec<Box<dyn RunLoop>>)>>> = None;

pub fn init_statics() {
	unsafe {
		SCRAP_DATA = Some(Mutex::new(BTreeMap::new()));
	}
}

fn get_composition(only_delta: bool) -> Result<CompositionTemplate, Box<dyn Error>> {
	let contents = fs::read_to_string(COMPOSITION_PATH)?;
	let mut composition: CompositionTemplate = ron::from_str(contents.as_str())?;

	if only_delta {
		let delta_string = fs::read_to_string(RELOAD_PATH)?;
		let delta_arr: Vec<String> = ron::from_str(&delta_string)?;

		for i in (0..composition.declarations.paths.len()).rev() {
			if !delta_arr.contains(&composition.declarations.paths[i].name) {
				composition.declarations.paths.remove(i);
			}
		}
	}

	let write = String::from("//This file is written to and managed automatically\n[]");
	fs::write(RELOAD_PATH, write).unwrap();

	Ok(composition)
}

pub fn lock_scrap_data<'a>() -> MutexGuard<'a, BTreeMap<String, (Library, Vec<Box<dyn RunLoop>>)>> {
	unsafe { SCRAP_DATA.as_ref().unwrap().lock() }
}

pub fn load_scraps(only_delta: bool) -> Result<(), Box<dyn Error>> {
	let composition = get_composition(only_delta)?;

	for path in &composition.declarations.paths {
		let mut lib_path = String::from(LIB_PREFIX);
		lib_path.push_str(path.name.as_str());
		lib_path.push_str(LIB_POSTFIX);
		lib_path = lib_path.replace("-", "_");

		let lib = unsafe { libloading::Library::new(lib_path)? };
		let mut g = lock_scrap_data();
		g.insert(path.name.clone(), (lib, vec![]));
		let storing = g.get_mut(&path.name).unwrap();

		for fn_path in &path.run_loops {
			let mut fn_name = String::from("new_runloop_");
			fn_name.push_str(fn_path.type_name.as_str());

			unsafe {
				let fn_ptr: libloading::Symbol<
					fn(&String, RunLoopId) -> Result<Box<dyn RunLoop>, ron::Error>,
				> = storing.0.get(fn_name.as_bytes())?;
				println!("{}", fn_name);

				let mut file_name = String::from(&fn_path.name);
				file_name.push_str(".ron");

				let mut path_buf = PathBuf::from(SCRAP_HEAP_PATH);
				path_buf.push(&path.name);
				path_buf.push("run_loops");
				path_buf.push(&file_name);

				println!("{}", path_buf.as_os_str().to_str().unwrap());
				let deserialize = fs::read_to_string(path_buf)?;
				storing.1.push(fn_ptr(&deserialize, RunLoopId::default())?);
			}
		}
	}
	Ok(())
}
