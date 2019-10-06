mod os;

pub use crate::os::*;
use std::os::raw::c_int;

// __declspec(dllexport) int Plugin_ApiVersion = 1;
// __declspec(dllexport) struct IGameComponent Plugin_Component = {
// 	ObjExporter_Init /* Init */
// };

#[no_mangle]
pub static Plugin_ApiVersion: c_int = 1;
