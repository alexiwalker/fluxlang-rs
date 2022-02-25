
mod tests;

pub mod core {
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

		// todo: make this fields that hold functionpointers or other values of any type, not just
		// functions
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
					location: self.location.clone()
				})),
				field_purity: self.function_purity.clone(),
				location: self.location.clone(),
				body: Option::Some(Box::from(self.function_body.clone()))
			}
		}
	}
	#[derive(Debug, Clone)]

	pub struct AstClassDecl {
		pub class_name: String,
		pub class_purity: Purity,
		pub class_constructor: Option<AstFunctionDecl>,
		pub class_static: bool,
		pub class_static_methods: Option<Vec<AstFunctionDecl>>,
		pub class_fields: Vec<AstFieldDecl>,
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
		pub body: Option<Box<AstBlock>>
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
		pub type_name: String,
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

	pub enum AstExpression {
		AstBinary(AstBinaryExpression),
		AstUnary(AstUnaryExpression),
		AstLiteral(AstLiteralExpression),
		AstVariableDeclaration(AstVariableDeclarationExpression),
		AstVariableUsage(AstVariableUsageExpression),
		// AstFunctionCall(AstFunctionCallExpression),
		AstSubscript(AstSubscriptExpression),
		AstFieldAccess(AstFieldAccessExpression),
		AstFunctionReturn(AstFunctionReturnExpression),

		AstIf(Box<AstIfExpression>),
	}
	#[derive(Debug, Clone)]

	pub struct AstIfExpression {
		pub if_condition: AstExpression,
		pub if_body: AstBlock,
		pub if_else_body: Option<AstBlock>,
		pub location: Location,
	}
	#[derive(Debug, Clone)]

	pub struct AstFunctionReturnExpression {
		pub return_value: Box<AstExpression>,
		pub location: Location,
	}
	#[derive(Debug, Clone)]

	pub struct AstVariableUsageExpression {
		pub variable_name: String,
		pub location: Location,
		pub usage: Option<Vec<AstValueUsageExpression>>,
	}

	//value expression usage reflects that you could do, say...
	// let x = module.class.arrayfield[0](1,2,3)[0]() ... chaining calls together
	// so the [n] and (...params) can be represented as a vec of usages
	#[derive(Debug, Clone)]

	pub struct AstFieldAccessExpression {
		pub nested_names: Vec<String>,
		pub usage: Option<Vec<AstValueUsageExpression>>,
	}
	#[derive(Debug, Clone)]

	pub enum AstValueUsageExpression {
		Subscript(AstSubscriptExpression),
		FunctionCall(AstFunctionCallExpression),
	}
	#[derive(Debug, Clone)]

	pub struct AstSubscriptExpression {
		pub subscript_index: Box<AstExpression>,
	}
	#[derive(Debug, Clone)]

	pub struct AstFunctionCallExpression {
		pub function_parameters: Vec<AstExpression>,
	}
	#[derive(Debug, Clone)]

	pub struct AstVariableDeclarationExpression {
		// pub parent: Box<AstExpression>,
		pub variable_name: String,
		pub variable_type: AstVariableType,
		pub variable_value: Option<Box<AstExpression>>,
	}
	#[derive(Debug, Clone)]

	pub struct AstBinaryExpression {
		pub parent: Box<AstExpression>,

		pub binary_operator: BinaryExpressionType,
		pub left_operand: Box<AstExpression>,
		pub right_operand: Box<AstExpression>,
		pub location: Location,
	}
	#[derive(Debug, Clone)]

	pub struct AstUnaryExpression {
		pub unary_operator: UnaryExpressionType,
		pub operand: Box<AstExpression>,
		pub location: Location,
	}
	#[derive(Debug, Clone)]

	pub enum UnaryExpressionType {
		Negate,
		Not,
		// Dereference,
		// AddressOf,
		// not sure if those are needed / will be implemented
	}
	#[derive(Debug, Clone)]

	pub struct AstLiteralExpression {
		pub value: LiteralValue,
		pub location: Location,
	}
	#[derive(Debug, Clone)]

	pub enum BinaryExpressionType {
		Add,
		Sub,
		Mul,
		Div,
		Mod,
		Equal,
		NotEqual,
		LessThan,
		GreaterThan,
		LessThanOrEqual,
		GreaterThanOrEqual,
		And,
		Or,
		Assign,
		AssignAdd,
		AssignSub,
		AssignMul,
		AssignDiv,
		AssignMod,
		AssignAnd,
		BitwiseAnd,
		BitwiseOr,
		BitwiseXor,
		BitwiseLeftShift,
		BitwiseRightShift,
	}

	#[derive(Debug, Clone)]

	pub enum AstValueType {
		Named(Vec<String>),
		Literal,
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
