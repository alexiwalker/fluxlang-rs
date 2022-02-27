#[cfg(test)]
mod tests {
	use crate::ast_expressions::ast_expressions::{
		AstExpression, AstFieldAccessExpression, AstFunctionCallExpression,
		AstFunctionReturnExpression, AstLiteralExpression, AstValueUsageExpression,
		AstVariableDeclarationExpression, AstVariableUsageExpression,
	};
	use crate::ast_nodes::ast_nodes::{
		AstBlock, AstFunctionDecl, AstModuleDecl, AstParameterDecl, AstType, AstUsingStatement,
		AstVariableType, LiteralValue, Location, Purity, SourceCodeFile,
	};

	#[test]
	fn it_works() {
		let result = 2 + 2;
		assert_eq!(result, 4);
	}

	#[test]
	fn construct_ast() {
		let using_statment: AstUsingStatement = AstUsingStatement {
			source_path_segments: vec![],
			use_import_as: None,
			use_as_pure: Purity::Pure,
			use_from_local_path: None,
			location: Location::new(1, 1, "test.flx"),
		};

		let main_module = AstModuleDecl {
			module_name: "main".to_string(),
			module_purity: Purity::Impure,
			module_callable: Option::Some(AstFunctionDecl {
				function_name: "".to_string(),
				function_purity: Purity::Pure,
				function_parameters: vec![AstParameterDecl {
					parameter_name: Option::from("args".to_string()),
					parameter_type: AstType::AstVariable(Box::from(AstVariableType {
						type_name: vec!["Array".to_string()],
						type_parameters: vec![AstType::AstVariable(Box::from(AstVariableType {
							type_name: vec!["string".to_string()],
							type_parameters: vec![],
							location: Location::default(),
						}))],
						location: Location::default(),
					})),
					location: Location::default(),
				}],
				function_return_type: None,
				function_body: AstBlock {
					block_statements: vec![
						AstExpression::AstFieldAccess(AstFieldAccessExpression {
							nested_names: vec!["io".to_string(), "print".to_string()],
							usage: Some(vec![AstValueUsageExpression::FunctionCall(
								AstFunctionCallExpression {
									function_parameters: vec![AstExpression::AstLiteral(
										AstLiteralExpression {
											value: LiteralValue::String("Hello,".to_string()),
											location: Location::default(),
										},
									)],
								},
							)]),
						}),
						AstExpression::AstFieldAccess(AstFieldAccessExpression {
							nested_names: vec!["io".to_string(), "print".to_string()],
							usage: Some(vec![AstValueUsageExpression::FunctionCall(
								AstFunctionCallExpression {
									function_parameters: vec![AstExpression::AstLiteral(
										AstLiteralExpression {
											value: LiteralValue::String(" World!".to_string()),
											location: Location::default(),
										},
									)],
								},
							)]),
						}),
						AstExpression::AstFieldAccess(AstFieldAccessExpression {
							nested_names: vec!["io".to_string(), "print".to_string()],
							usage: Some(vec![AstValueUsageExpression::FunctionCall(
								AstFunctionCallExpression {
									function_parameters: vec![AstExpression::AstLiteral(
										AstLiteralExpression {
											value: LiteralValue::String("\n".to_string()),
											location: Location::default(),
										},
									)],
								},
							)]),
						}),
						AstExpression::AstVariableDeclaration(AstVariableDeclarationExpression {
							variable_name: "v".to_string(),
							variable_type: AstVariableType {
								type_name: vec!["string".to_string()],
								type_parameters: vec![],
								location: Location::default(),
							},
							variable_value: Option::Some(Box::from(AstExpression::AstLiteral(
								AstLiteralExpression {
									value: LiteralValue::String("some other string".to_string()),
									location: Location::default(),
								},
							))),
						}),
						AstExpression::AstFieldAccess(AstFieldAccessExpression {
							nested_names: vec!["io".to_string(), "printLine".to_string()],
							usage: Some(vec![AstValueUsageExpression::FunctionCall(
								AstFunctionCallExpression {
									function_parameters: vec![AstExpression::AstVariableUsage(
										AstVariableUsageExpression {
											variable_name: "exampleFunctionCall".to_string(),
											location: Location::default(),
											usage: Option::from(vec![
												AstValueUsageExpression::FunctionCall(
													AstFunctionCallExpression {
														function_parameters: vec![
															AstExpression::AstVariableUsage(
																AstVariableUsageExpression {
																	variable_name: "v".to_string(),
																	location: Location::default(),
																	usage: None,
																},
															),
														],
													},
												),
											]),
										},
									)],
								},
							)]),
						}),
					],
					location: Location::default(),
				},
				location: Location::default(),
			}),
			module_fields: vec![
				(AstFunctionDecl {
					function_name: "exampleFunctionCall".to_string(),
					function_purity: Purity::Pure,
					function_parameters: vec![AstParameterDecl {
						parameter_name: Option::from("variable".to_string()),
						parameter_type: AstType::AstVariable(Box::from(AstVariableType {
							type_name: vec!["string".to_string()],
							type_parameters: vec![],
							location: Location::default(),
						})),
						location: Location::default(),
					}],
					function_return_type: Option::from(AstType::AstVariable(Box::from(
						AstVariableType {
							type_name: vec!["string".to_string()],
							type_parameters: vec![],
							location: Location::default(),
						},
					))),
					function_body: AstBlock {
						block_statements: vec![AstExpression::AstFunctionReturn(
							AstFunctionReturnExpression {
								return_value: Box::from(AstExpression::AstFieldAccess(
									AstFieldAccessExpression {
										nested_names: vec![
											"string".to_string(),
											"reverse".to_string(),
										],
										usage: Some(vec![AstValueUsageExpression::FunctionCall(
											AstFunctionCallExpression {
												function_parameters: vec![
													AstExpression::AstVariableUsage(
														AstVariableUsageExpression {
															variable_name: "variable".to_string(),
															location: Location::default(),
															usage: None,
														},
													),
												],
											},
										)]),
									},
								)),
								location: Location::default(),
							},
						)],
						location: Location::default(),
					},
					location: Location::default(),
				})
				.to_field_decl(),
				(AstFunctionDecl {
					function_name: "exampleFunctionCallProxy".to_string(),
					function_purity: Purity::Pure,
					function_parameters: vec![AstParameterDecl {
						parameter_name: Option::from("variable".to_string()),
						parameter_type: AstType::AstVariable(Box::from(AstVariableType {
							type_name: vec!["string".to_string()],
							type_parameters: vec![],
							location: Location::default(),
						})),
						location: Location::default(),
					}],
					function_return_type: Option::from(AstType::AstVariable(Box::from(
						AstVariableType {
							type_name: vec!["string".to_string()],
							type_parameters: vec![],
							location: Location::default(),
						},
					))),
					function_body: AstBlock {
						block_statements: vec![AstExpression::AstFunctionReturn(
							AstFunctionReturnExpression {
								return_value: Box::from(AstExpression::AstFieldAccess(
									AstFieldAccessExpression {
										nested_names: vec![
											"string".to_string(),
											"reverse".to_string(),
										],
										usage: Some(vec![AstValueUsageExpression::FunctionCall(
											AstFunctionCallExpression {
												function_parameters: vec![
													AstExpression::AstVariableUsage(
														AstVariableUsageExpression {
															variable_name: "variable".to_string(),
															location: Location::default(),
															usage: None,
														},
													),
												],
											},
										)]),
									},
								)),
								location: Location::default(),
							},
						)],
						location: Location::default(),
					},
					location: Location::default(),
				})
				.to_field_decl(),
			],
			module_using_statements: vec![],
			module_class_decls: vec![],
			location: Location {
				line: 0,
				column: 0,
				file: "".to_string(),
			},
		};

		let file_struct: SourceCodeFile = SourceCodeFile {
			file_name: "test.flx".to_string(),
			using_statements: vec![using_statment],
			file_modules: vec![main_module],
		};

		println!("{:?}", file_struct);
	}
}
