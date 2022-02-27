// dead code allowed because exposed library functions may not all be required by user
#[allow(dead_code)]
pub mod ast_nodes {
	use crate::ast_expressions::ast_expressions::AstExpression;

	#[derive(Debug, Clone)]

	pub struct SourceCodeFile {
		pub file_name: String,
		pub using_statements: Vec<AstUsingStatement>,
		pub file_modules: Vec<AstModuleDecl>,
	}
	#[derive(Debug, Clone)]

	pub struct AstUsingStatement {
		pub source_path_segments: Vec<String>,
		pub use_import_as: Option<String>,
		pub use_as_pure: Purity,

		//if local path is not used, will check std and libraries
		pub use_from_local_path: Option<String>,
		pub location: Location,
	}
	#[derive(Debug, Clone)]

	pub struct AstModuleDecl {
		pub module_name: String,
		pub module_purity: Purity,
		pub module_callable: Option<AstFunctionDecl>,
		pub module_fields: Vec<AstFieldDecl>,
		pub module_using_statements: Vec<AstUsingStatement>,
		pub module_class_decls: Vec<AstClassDecl>,
		pub location: Location,
	}
	impl AstFunctionDecl {
		pub fn to_field_decl(&self) -> AstFieldDecl {
			AstFieldDecl {
				field_name: self.function_name.clone(),
				field_type: AstType::AstFunctionPointer(Box::from(AstFunctionPointerType {
					function_pointer_type_parameters: self.function_parameters.clone(),
					function_pointer_type_return_type: self.function_return_type.clone(),
					location: self.location.clone(),
				})),
				field_purity: self.function_purity.clone(),
				location: self.location.clone(),
				body: Option::Some(Box::from(self.function_body.clone())),
			}
		}
	}

	#[derive(Debug, Clone)]
	pub struct AstClassDecl {
		pub class_name: String,
		pub class_purity: Purity,
		pub class_constructor: Option<AstFunctionDecl>,
		pub class_static: bool,
		pub class_static_fields: Vec<AstFieldDecl>,
		pub class_fields: Vec<AstFieldDecl>,
		pub extends_class_names: Vec<String>,
		pub location: Location,
	}
	#[derive(Debug, Clone)]

	pub struct AstFieldDecl {
		pub field_name: String,
		pub field_type: AstType,
		pub field_purity: Purity,
		pub location: Location,

		// this is only relevant if the field is a function declaration
		// OR if the field is declared as part of an interface (eg, has a definition but not an implementation)
		pub body: Option<Box<AstBlock>>,
	}
	#[derive(Debug, Clone)]

	pub struct AstFunctionDecl {
		pub function_name: String,
		pub function_purity: Purity,
		pub function_parameters: Vec<AstParameterDecl>,
		pub function_return_type: Option<AstType>,
		pub function_body: AstBlock,
		pub location: Location,
	}

	#[derive(Debug, Clone)]
	pub struct AstParameterDecl {
		pub parameter_name: Option<String>,
		pub parameter_type: AstType,
		pub location: Location,
	}
	#[derive(Debug, Clone)]

	pub enum AstType {
		AstVariable(Box<AstVariableType>),
		AstFunctionPointer(Box<AstFunctionPointerType>),
	}

	#[derive(Debug, Clone)]

	pub struct AstVariableType {
		//type name is a vec, usually only one element
		//but it is a vec so we can do eg Module.Module.Module.Typename
		pub type_name: Vec<String>,
		pub type_parameters: Vec<AstType>,
		pub location: Location,
	}

	#[derive(Debug, Clone)]
	pub struct AstFunctionPointerType {
		pub function_pointer_type_parameters: Vec<AstParameterDecl>,
		pub function_pointer_type_return_type: Option<AstType>,
		pub location: Location,
	}

	#[derive(Debug, Clone)]
	pub struct AstBlock {
		pub block_statements: Vec<AstExpression>,
		pub location: Location,
	}

	#[derive(Debug, Clone)]
	pub enum Purity {
		Pure,
		Impure,
	}

	#[derive(Debug, Clone)]
	pub enum Tokens {
		Return(Location),
		If(Location),
		Else(Location),
		While(Location),
		For(Location),
		Break(Location),
		Continue(Location),
		Struct(Location),
		Enum(Location),
		FunctionBindArrow(Location),
		FunctionSymbolF(Location),
		LeftBrace(Location),
		RightBrace(Location),
		LeftBracket(Location),
		RightBracket(Location),
		LeftParen(Location),
		RightParen(Location),
		Comma(Location),
		Semicolon(Location),
		Colon(Location),
		Dot(Location),
		QuestionMark(Location),
		Equal(Location),
		NotEqual(Location),
		LessThan(Location),
		LessThanOrEqual(Location),
		GreaterThan(Location),
		GreaterThanOrEqual(Location),
		NumericLiteral(NumericLiteralValue),
		StringLiteral(String, Location),
		BooleanLiteral(bool, Location),
		HashSymbol(Location),

		ModuleImportSymbol(Location),
		ModuleKeyword(Location),
		ClassKeyword(Location),
		PureKeyword(Location),
		ImpureKeyword(Location),
		LetKeyword(Location),
		ImplicitKeyword(Location),
		ConstructorKeyword(Location),
		StaticKeyword(Location),
		Identifier(String, Location),

		Div(Location),
		IntDiv(Location),
		Mod(Location),
		Add(Location),
		Sub(Location),
		Mul(Location),
		And(Location),
		Or(Location),
		Not(Location),
		Xor(Location),
		LShift(Location),
		RShift(Location),
		BitAnd(Location),
		BitOr(Location),
		BitXor(Location),
		BitNot(Location),
		BitLShift(Location),
		BitRShift(Location),

		CoreKeyword(Location),

		Assign(Location),

		CompilerPlatformStub(String, Location),
	}

	#[derive(Debug, Clone)]
	pub enum LiteralValue {
		String(String),
		Numeric(NumericLiteralValue),
		Boolean(bool),
	}

	#[derive(Debug, Clone)]
	pub enum NumericLiteralValue {
		Int8(i8),
		Int16(i16),
		Int32(i32),
		Int64(i64),
		UInt8(u8),
		UInt16(u16),
		UInt32(u32),
		UInt64(u64),
		Float32(f32),
		Float64(f64),
	}

	#[derive(Debug, Clone)]
	pub struct Location {
		pub line: usize,
		pub column: usize,
		pub file: String,
	}

	impl Location {
		pub fn new(line: usize, column: usize, file: &str) -> Location {
			Location {
				line,
				column,
				file: file.to_string(),
			}
		}

		pub fn default() -> Location {
			Location::new(1, 1, "test.flx")
		}
	}
}
