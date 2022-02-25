mod io;

pub mod std_lib {
	use core::ast_expressions::ast_expressions::*;
	use core::ast_nodes::ast_nodes::*;
	use crate::io::io;

	pub fn std_lib_file(module_name:String) ->Option<SourceCodeFile> {

		match module_name {
			"io".to_string() => {
				return Some(io::exports());
			}
			_ => {
				None
			}
		}

		None
	}
}
