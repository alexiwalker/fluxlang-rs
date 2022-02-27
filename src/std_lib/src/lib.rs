mod io;

pub mod std_lib {

	#[test]
	fn compiles(){
		println!("compiles");
	}


	#[test]
	fn get_module(){
		let res= std_lib_file("io".to_string());

		assert!(res.is_some());

		let res2= std_lib_file("iio".to_string());

		assert!(res2.is_none());

	}

	use core::ast_nodes::ast_nodes::*;
	use crate::io::io;

	pub fn std_lib_file(module_name:String) ->Option<SourceCodeFile> {

		match module_name.as_str() {
			"io" => {
				return Some(io::exports());
			}
			_ => {
				return None
			}
		}
	}
}
