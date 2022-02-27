
pub mod io {
	use core::ast_expressions::ast_expressions::*;
	use core::ast_nodes::ast_nodes::*;
	use crate::io::{_print_line, _print_line_dbg};

	pub fn exports() -> SourceCodeFile {
		_file()
	}

	fn _module()->AstModuleDecl {
		AstModuleDecl {
			module_name: "io".to_string(),
			module_purity: Purity::Impure,
			module_callable: None,
			module_fields: vec![_read_line(), _print_line(), _print()],
			module_using_statements: vec![],
			module_class_decls: vec![_dbg_class()],
			location: Location {
				line: 0,
				column: 0,
				file: "".to_string(),
			},
		}
	}

	fn _file()->SourceCodeFile {
		SourceCodeFile {
			file_name: "std_lib/io.flx".to_string(),
			using_statements: vec![],
			file_modules: vec![_module()],
		}
	}

	fn _dbg_class()->AstClassDecl {
		AstClassDecl {
			class_name: "debug".to_string(),
			class_purity: Purity::Impure,
			class_constructor: None,
			class_static: true,
			class_static_fields: vec![
				_print_line_dbg()
			],
			class_fields: vec![],
			extends_class_names: vec![],
			location: Location::default(),
		}
	}

	fn _read_line()->AstFieldDecl {
		AstFieldDecl {
			field_name: "read_line".to_string(),
			field_type:
			AstType::AstFunctionPointer(
				Box::from(
					AstFunctionPointerType {
						function_pointer_type_parameters: vec![],
						function_pointer_type_return_type: Option::from(
							AstType::AstVariable(
								Box::from(
									AstVariableType {
										type_name: vec!["string".to_string()],
										type_parameters: vec![],
										location: Location {
											line: 0,
											column: 0,
											file: "".to_string(),
										},
									}
								)
							)
						),
						location: Location {
							line: 0,
							column: 0,
							file: "".to_string(),
						},
					})),

			field_purity: Purity::Impure,
			location: Location {
				line: 0,
				column: 0,
				file: "".to_string(),
			},
			body: Option::Some(
				Box::from(
					AstBlock {
						block_statements: vec![
							AstExpression::AstVariableDeclaration(AstVariableDeclarationExpression {
								variable_name: "s".to_string(),
								variable_type: AstVariableType {
									type_name: vec!["string".to_string()],
									type_parameters: vec![],
									location: Location::default(),
								},
								variable_value: None,
							}),
							AstExpression::AstCompilerStub("PLATFORM_READLINE_STRING".to_string()),
							AstExpression::AstFunctionReturn(AstFunctionReturnExpression {
								return_value: Box::new(AstExpression::AstVariableUsage(AstVariableUsageExpression {
									variable_name: "s".to_string(),
									location: Location::default(),
									usage: None,
								})),
								location: Location::default(),
							}),
						],
						location: Location::default(),
					}
				)
			),
		}
	}

	fn _print_line()->AstFieldDecl {
		AstFieldDecl {
			field_name: "printLine".to_string(),
			field_type: AstType::AstFunctionPointer(
				Box::from(
					AstFunctionPointerType {
						function_pointer_type_parameters: vec![
							AstParameterDecl {
								parameter_name: None,
								parameter_type: AstType::AstVariable(
									Box::from(
										AstVariableType {
											type_name: vec!["string:".to_string()],
											type_parameters: vec![],
											location: Location::default(),
										}
									)),
								location: Location::default(),
							}
						],
						function_pointer_type_return_type: None,
						location: Location::default(),
					})),
			field_purity: Purity::Impure,
			location: Location::default(),
			body: Option::Some(
				Box::from(
					AstBlock {
						block_statements: vec![
							AstExpression::AstCompilerStub("PLATFORM_PRINTLINE_STRING".to_string()),
						],
						location: Location::default(),
					}
				)
			),
		}
	}

	fn _print()->AstFieldDecl {
		AstFieldDecl {
			field_name: "printLine".to_string(),
			field_type: AstType::AstFunctionPointer(
				Box::from(
					AstFunctionPointerType {
						function_pointer_type_parameters: vec![
							AstParameterDecl {
								parameter_name: None,
								parameter_type: AstType::AstVariable(
									Box::from(
										AstVariableType {
											type_name:vec!["string:".to_string()],
											type_parameters: vec![],
											location: Location::default(),
										}
									)),
								location: Location::default(),
							}
						],
						function_pointer_type_return_type: None,
						location: Location::default(),
					})),
			field_purity: Purity::Impure,
			location: Location::default(),
			body: Option::Some(
				Box::from(
					AstBlock {
						block_statements: vec![
							AstExpression::AstCompilerStub("PLATFORM_PRINT_STRING".to_string()),
						],
						location: Location::default(),
					}
				)
			),
		}
	}

	fn _print_line_dbg()->AstFieldDecl {
		AstFieldDecl {
			field_name: "print".to_string(),
			field_type: AstType::AstFunctionPointer(
				Box::from(
					AstFunctionPointerType {
						function_pointer_type_parameters: vec![
							AstParameterDecl {
								parameter_name: None,
								parameter_type: AstType::AstVariable(
									Box::from(
										AstVariableType {
											type_name: vec!["string:".to_string()],
											type_parameters: vec![],
											location: Location::default(),
										}
									)),
								location: Location::default(),
							}
						],
						function_pointer_type_return_type: None,
						location: Location::default(),
					}
				)),
			field_purity: Purity::Pure,
			location: Location::default(),
			body: Option::Some(
				Box::from(
					AstBlock {
						block_statements: vec![
							AstExpression::AstCompilerStub("PLATFORM_PRINTLINE_STRING_DEBUG".to_string()),
						],
						location: Location::default(),
					}
				)
			),
		}
	}

}