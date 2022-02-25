#[cfg(test)]
mod tests {
	use crate::core::{AstBlock, AstExpression, AstFieldAccessExpression, AstFunctionCallExpression, AstFunctionDecl, AstLiteralExpression, AstModuleDecl, AstParameterDecl, AstType, AstUsingStatement, AstValueUsageExpression, AstVariableDeclarationExpression, AstVariableUsageExpression, LiteralValue, Location, Purity, SourceCodeFile};
	use crate::core::AstValueUsageExpression::FunctionCall;

	#[test]
	fn it_works() {
		let result = 2 + 2;
		assert_eq!(result, 4);
	}


	fn construct_ast() {
		let usingStatment: AstUsingStatement = AstUsingStatement {
			source_path_segments: vec![],
			use_import_as: None,
			use_as_pure: Purity::Pure,
			use_from_local_path: None,
			location: Location::new(1, 1, "test.flx"),
		};

		let fileStruct: SourceCodeFile = SourceCodeFile {
			file_name: "test.flx".to_string(),
			using_statements: vec![usingStatment],
			file_modules: vec![
				AstModuleDecl {
					module_name: "".to_string(),
					module_purity: Purity::Impure,
					module_callable: vec![
						AstFunctionDecl {
							function_name: "".to_string(),
							function_purity: Purity::Pure,
							function_parameters: vec![AstParameterDecl {
								parameter_name: "".to_string(),
								parameter_type: AstType {
									type_name: "".to_string(),
									type_parameters: vec![],
									location: Location::default(),
								},
								location: Location::default(),
							}
							],
							function_return_type: None,
							function_body: AstBlock {
								block_statements: vec![
									AstExpression::AstFieldAccess(AstFieldAccessExpression {
										nested_names: vec!["io".to_string(), "print".to_string()],
										usage: Some(
											vec![
												AstValueUsageExpression::FunctionCall(AstFunctionCallExpression {
													function_parameters: vec![
														AstExpression::AstLiteral(
															AstLiteralExpression {
																value: LiteralValue::String("Hello,".to_string()),
																location: Location::default(),
															})
													]
												}),
											]),
									}),
									AstExpression::AstFieldAccess(AstFieldAccessExpression {
										nested_names: vec!["io".to_string(), "print".to_string()],
										usage: Some(
											vec![
												AstValueUsageExpression::FunctionCall(AstFunctionCallExpression {
													function_parameters: vec![
														AstExpression::AstLiteral(
															AstLiteralExpression {
																value: LiteralValue::String(" World!".to_string()),
																location: Location::default(),
															})
													]
												}),
											]),
									}),
									AstExpression::AstFieldAccess(AstFieldAccessExpression {
										nested_names: vec!["io".to_string(), "print".to_string()],
										usage: Some(
											vec![
												AstValueUsageExpression::FunctionCall(AstFunctionCallExpression {
													function_parameters: vec![
														AstExpression::AstLiteral(
															AstLiteralExpression {
																value: LiteralValue::String("\n".to_string()),
																location: Location::default(),
															})
													]
												}),
											]),
									}),
									AstExpression::AstVariableDeclaration(AstVariableDeclarationExpression {
										variable_name: "v".to_string(),
										variable_type: AstType {
											type_name: "string".to_string(),
											type_parameters: vec![],
											location: Location::default(),
										},
										variable_value: Option::Some(
											Box::from(AstExpression::AstLiteral(AstLiteralExpression {
												value: LiteralValue::String("some other string".to_string()),
												location: Location::default(),
											}))
										),
									}),
									AstExpression::AstFieldAccess(AstFieldAccessExpression {
										nested_names: vec!["io".to_string(), "printLine".to_string()],
										usage: Some(
											vec![
												AstValueUsageExpression::FunctionCall(
													AstFunctionCallExpression {
														function_parameters: vec![
															AstExpression::AstVariableUsage(AstVariableUsageExpression {
																variable_name: "exampleFunctionCall".to_string(),
																location: Location::default(),
																usage: Option::from(vec![
																	AstValueUsageExpression::FunctionCall(AstFunctionCallExpression {
																		function_parameters: vec![
																			AstExpression::AstVariableUsage(AstVariableUsageExpression {
																				variable_name: "v".to_string(),
																				location: Location::default(),
																				usage: None,
																			})
																		]
																	})
																]),
															})
														]
													}),
											]),
									}),
								],

								location: Location::default(),
							},
							location: Location::default(),
						}],
					module_using_statements: vec![],
					module_class_decls: vec![],
					location: Location {
						line: 0,
						column: 0,
						file: "".to_string(),
					},
				}
			],
		};
	}
}
