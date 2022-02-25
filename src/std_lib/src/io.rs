use std::panic::Location;

pub mod io {
	use core::ast_expressions::ast_expressions::*;
	use core::ast_nodes::ast_nodes::*;
	use crate::io::{PRINT, PRINT_LINE, READ_LINE};

	pub fn exports() -> SourceCodeFile {
		FILE
	}

	const FILE: SourceCodeFile = SourceCodeFile {
		file_name: "std_lib/io.flx".to_string(),
		using_statements: vec![],
		file_modules: vec![MODULE],
	};

	const MODULE: AstModuleDecl = AstModuleDecl {
		module_name: "io".to_string(),
		module_purity: Purity::Impure,
		module_callable: None,
		module_fields: vec![READ_LINE, PRINT_LINE, PRINT],
		module_using_statements: vec![],
		module_class_decls: vec![DEBUG_CLASS_DECl],
		location: Location {
			line: 0,
			column: 0,
			file: "".to_string(),
		},
	};

	const DEBUG_CLASS_DECl: AstClassDecl = AstClassDecl {
		class_name: "debug".to_string(),
		class_purity: Purity::Impure,
		class_constructor: None,
		class_static: true,
		class_static_fields: vec![
			PRINT_LINE_DEBUG
		],
		class_fields: vec![],
		location: Location::default(),
	};

	const READ_LINE: AstFieldDecl = AstFieldDecl {
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
									type_name: "string".to_string(),
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
								type_name: "string".to_string(),
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
	};

	const PRINT_LINE: AstFieldDecl = AstFieldDecl {
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
										type_name: "string:".to_string(),
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
	};

	const PRINT: AstFieldDecl = AstFieldDecl {
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
										type_name: "string:".to_string(),
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
	};

	const PRINT_LINE_DEBUG: AstFieldDecl = AstFieldDecl {
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
										type_name: "string:".to_string(),
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
	};
}