pub mod source_map;
pub mod wasm;

pub use source_map::{SourceLocation, SourceMap};
pub use wasm::{parse_functions, get_module_info, ModuleInfo};