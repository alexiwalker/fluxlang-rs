extern crate core;

pub mod ast_expressions;
pub mod ast_nodes;
mod tests;
pub mod type_sizing;

mod localtests {
	#[test]
	fn compiles() {
		assert!(true);
	}
}

pub mod core {
	use std::borrow::Borrow;
	use std::task::Context;
	use core::panicking::panic;
	use crate::ast_nodes::ast_nodes::{AstClassDecl, AstFieldDecl, AstType, Purity, SourceCodeFile};

	pub struct CompilationContext<'a> {
		trees: Vec<&'a SourceCodeFile>,
		type_data: Vec<&'a TypeData<'a>>,
	}

	pub struct TypeData<'a> {
		pub name: String,

		//some types cant have a size, eg interfaces
		pub stack_size: Option<usize>,
		pub purity: Purity,

		pub sort_of_type: TypeType,

		pub extends: Vec<&'a TypeData<'a>>,
	}

	pub fn _int64() -> TypeData {
		TypeData {
			name: "int64".to_string(),
			stack_size: Some(8),
			purity: Purity::Pure,
			sort_of_type: TypeType::Primitive,
			extends: vec![],
		}
	}

	pub fn _int32() -> TypeData {
		TypeData {
			name: "int32".to_string(),
			stack_size: Some(4),
			purity: Purity::Pure,
			sort_of_type: TypeType::Primitive,
			extends: vec![],
		}
	}

	pub fn _int16() -> TypeData {
		TypeData {
			name: "int16".to_string(),
			stack_size: Some(2),
			purity: Purity::Pure,
			sort_of_type: TypeType::Primitive,
			extends: vec![],
		}
	}

	pub fn _int8() -> TypeData {
		TypeData {
			name: "int8".to_string(),
			stack_size: Some(1),
			purity: Purity::Pure,
			sort_of_type: TypeType::Primitive,
			extends: vec![],
		}
	}

	pub fn _uint64() -> TypeData {
		TypeData {
			name: "uint64".to_string(),
			stack_size: Some(8),
			purity: Purity::Pure,
			sort_of_type: TypeType::Primitive,
			extends: vec![],
		}
	}

	pub fn _uint32() -> TypeData {
		TypeData {
			name: "uint32".to_string(),
			stack_size: Some(4),
			purity: Purity::Pure,
			sort_of_type: TypeType::Primitive,
			extends: vec![],
		}
	}

	pub fn _uint16() -> TypeData {
		TypeData {
			name: "uint16".to_string(),
			stack_size: Some(2),
			purity: Purity::Pure,
			sort_of_type: TypeType::Primitive,
			extends: vec![],
		}
	}

	pub fn _uint8() -> TypeData {
		TypeData {
			name: "uint8".to_string(),
			stack_size: Some(1),
			purity: Purity::Pure,
			sort_of_type: TypeType::Primitive,
			extends: vec![],
		}
	}

	pub fn _bool() -> TypeData {
		TypeData {
			name: "bool".to_string(),
			stack_size: Some(1),
			purity: Purity::Pure,
			sort_of_type: TypeType::Primitive,
			extends: vec![],
		}
	}

	pub fn _char() -> TypeData {
		TypeData {
			name: "char".to_string(),
			stack_size: Some(1),
			purity: Purity::Pure,
			sort_of_type: TypeType::Primitive,
			extends: vec![],
		}
	}

	pub fn _string() -> TypeData {
		TypeData {
			name: "string".to_string(),
			stack_size: None,
			purity: Purity::Pure,
			sort_of_type: TypeType::Primitive,
			extends: vec![],
		}
	}

	pub fn _pointer() -> TypeData {
		TypeData {
			name: "pointer".to_string(),
			//size of usize which can address any memory
			stack_size: Some(8),

			//directly using a pointer is not a pure operation because it allows direct modification
			//in a way that is not allowed with a pure context
			purity: Purity::Impure,
			sort_of_type: TypeType::Pointer,
			extends: vec![],
		}
	}

	pub fn _void() -> TypeData {
		TypeData {
			name: "void".to_string(),
			stack_size: None,
			purity: Purity::Pure,
			sort_of_type: TypeType::Primitive,
			extends: vec![],
		}
	}

	pub fn _array(of: TypeData, s: usize) -> TypeData {
		TypeData {
			name: "array".to_string(),
			stack_size: of.stack_size * s,
			purity: Purity::Pure,
			sort_of_type: TypeType::Array,
			extends: vec![],
		}
	}

	trait IsType {
		fn TypeData(&self, context: &CompilationContext) -> TypeData;
	}


	impl IsType for AstClassDecl {
		fn TypeData(&mut self, context: &CompilationContext) -> TypeData {
			let mut sizeable_fields: Vec<AstFieldDecl> = vec![];
			let mut current_ptr_sizes = 0usize;
			for field in self.class_fields.clone().into_vec() {
				match field.field_type {
					AstType::AstVariable(vdecl) => {
						sizeable_fields.push(field.clone())
					}
					AstType::AstFunctionPointer(_) => {
						current_ptr_sizes += 8; // ptr takes 8 bytes
					}
				}
			}


			//we need to know what file this type is from so we can evaluate its using statements
			//that way we can check if the included types are already evaluated
			//if they aren't, we can find them and evaluate them to get the size we need
			//recursively, until we have the size of the original type
			let trees = context.trees.borrow();
			let mut types_declaring_file: Option<SourceCodeFile> = None;
			for tree in trees.into_vec() {
				let t = (*tree).clone();
				let file_of_type = self.location.file.clone();
				if file_of_type == tree.file_name {
					types_declaring_file = Some(t);
				}
			}

			let file = match types_declaring_file {
				None => {
					panic!("Could not find file type was declared in: {0} declared in file {1} but {1} not available in context", self.class_name,self.location.file)
				}
				Some(v) => {
					v
				}
			};

			let using_statements = file.using_statements.borrow();

			//todo take the fields type and resolve it down to a fully qualified type here and then lookup that type to get its size
			// for using in using_statements.into_iter() {
			// 	using.
			// }



			TypeData {
				name: self.class_name.clone(),
				stack_size: None,
				purity: self.class_purity.clone(),
				sort_of_type: TypeType::Class,
				extends: vec![],
			}
		}
	}


	pub enum TypeType {
		Primitive,
		Class,
		Interface,
		Pointer,
		Array,
		Function,
	}

}
