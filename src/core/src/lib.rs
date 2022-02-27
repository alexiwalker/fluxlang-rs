
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
	use std::collections::HashMap;
	use crate::ast_nodes::ast_nodes::{AstClassDecl, AstFieldDecl, AstType, AstUsingStatement, Location, Purity, SourceCodeFile};

	pub struct CompilationContext<'a> {
		// trees: Vec<&'a SourceCodeFile>,
		trees: HashMap<String, &'a SourceCodeFile>,
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

	pub fn _int64<'a>() -> TypeData<'a> {
		TypeData {
			name: "int64".to_string(),
			stack_size: Some(8),
			purity: Purity::Pure,
			sort_of_type: TypeType::Primitive,
			extends: vec![],
		}
	}

	pub fn _int32<'a>() -> TypeData<'a> {
		TypeData {
			name: "int32".to_string(),
			stack_size: Some(4),
			purity: Purity::Pure,
			sort_of_type: TypeType::Primitive,
			extends: vec![],
		}
	}

	pub fn _int16<'a>() -> TypeData<'a> {
		TypeData {
			name: "int16".to_string(),
			stack_size: Some(2),
			purity: Purity::Pure,
			sort_of_type: TypeType::Primitive,
			extends: vec![],
		}
	}

	pub fn _int8<'a>() -> TypeData<'a>  {
		TypeData {
			name: "int8".to_string(),
			stack_size: Some(1),
			purity: Purity::Pure,
			sort_of_type: TypeType::Primitive,
			extends: vec![],
		}
	}

	pub fn _uint64<'a>() -> TypeData<'a> {
		TypeData {
			name: "uint64".to_string(),
			stack_size: Some(8),
			purity: Purity::Pure,
			sort_of_type: TypeType::Primitive,
			extends: vec![],
		}
	}

	pub fn _uint32<'a>() -> TypeData<'a>  {
		TypeData {
			name: "uint32".to_string(),
			stack_size: Some(4),
			purity: Purity::Pure,
			sort_of_type: TypeType::Primitive,
			extends: vec![],
		}
	}

	pub fn _uint16<'a>() -> TypeData<'a>  {
		TypeData {
			name: "uint16".to_string(),
			stack_size: Some(2),
			purity: Purity::Pure,
			sort_of_type: TypeType::Primitive,
			extends: vec![],
		}
	}

	pub fn _uint8<'a>() -> TypeData<'a>  {
		TypeData {
			name: "uint8".to_string(),
			stack_size: Some(1),
			purity: Purity::Pure,
			sort_of_type: TypeType::Primitive,
			extends: vec![],
		}
	}

	pub fn _float64<'a>() -> TypeData<'a>  {
		TypeData {
			name: "float64".to_string(),
			stack_size: Some(8),
			purity: Purity::Pure,
			sort_of_type: TypeType::Primitive,
			extends: vec![],
		}
	}

	pub fn _float32<'a>() -> TypeData<'a>  {
		TypeData {
			name: "float32".to_string(),
			stack_size: Some(4),
			purity: Purity::Pure,
			sort_of_type: TypeType::Primitive,
			extends: vec![],
		}
	}


	pub fn _bool<'a>() -> TypeData<'a>  {
		TypeData {
			name: "bool".to_string(),
			stack_size: Some(1),
			purity: Purity::Pure,
			sort_of_type: TypeType::Primitive,
			extends: vec![],
		}
	}

	pub fn _char<'a>() -> TypeData<'a>  {
		TypeData {
			name: "char".to_string(),
			stack_size: Some(1),
			purity: Purity::Pure,
			sort_of_type: TypeType::Primitive,
			extends: vec![],
		}
	}

	pub fn _string<'a>() -> TypeData<'a>  {
		TypeData {
			name: "string".to_string(),

			//ptr size - strings live on the heap
			stack_size: Some(8),
			purity: Purity::Pure,
			sort_of_type: TypeType::Primitive,
			extends: vec![],
		}
	}

	pub fn _pointer<'a>() -> TypeData<'a>  {
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

	pub fn _void<'a>() -> TypeData<'a>  {
		TypeData {
			name: "void".to_string(),
			stack_size: None,
			purity: Purity::Pure,
			sort_of_type: TypeType::Primitive,
			extends: vec![],
		}
	}

	pub fn _array<'a>(of: TypeData, s: usize) -> TypeData<'a> {
		TypeData {
			name: "array".to_string(),
			stack_size: Some(of.stack_size.unwrap_or(0) * s),
			purity: Purity::Pure,
			sort_of_type: TypeType::Array,
			extends: vec![],
		}
	}

	trait Type {
		fn type_data(&mut self, context: &CompilationContext) -> TypeData;
	}


	impl Type for AstClassDecl {
		fn type_data(&mut self, context: &CompilationContext) -> TypeData {
			let mut sizeable_fields: Vec<AstFieldDecl> = vec![];
			let mut current_ptr_sizes = 0usize;

			//doesnt include static fields, only instance members
			for field in self.class_fields.clone().into_iter() {
				match field.field_type {
					AstType::AstVariable(_) => {
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
			let mut types_declaring_file: Option<&SourceCodeFile> = None;
			for tree in trees.into_iter() {
				let t = tree.1.clone();
				let file_of_type = self.location.file.clone();
				if file_of_type == tree.1.file_name {
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

			let using_statements:&Vec<AstUsingStatement> = file.using_statements.borrow();

			//todo take the fields type and resolve it down to a fully qualified type here and then lookup that type to get its size
			let mut available_types:Vec<&AstType> = vec![];
			for using in using_statements.into_iter() {

				let local_path = using.use_from_local_path.clone();

				let imported_file:&SourceCodeFile = match local_path {
					None => {
						panic!("Could not find file type was declared in: {0} declared in file {1} but {1} not available in context", self.class_name,self.location.file)
					}
					Some(path) => {

						let tree=context.trees.get(&*path.clone());

						let _i = match tree {
							None => {

								panic!("File not found")
							}
							Some(_t) => {
								_t
							}
						};

						*_i

					}
				};

				// todo extract information about the imported file and use that to get the types
				// then use the available types to get the size of the current type

			}

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
