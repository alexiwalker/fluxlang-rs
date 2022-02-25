
mod tests;


pub mod core {

	pub struct SourceCodeFile {
		pub file_name: String,
		pub using_statements: Vec<AstUsingStatement>,
		pub file_modules: Vec<AstModuleDecl>,
    }

	pub struct AstUsingStatement {
		pub source_path_segments: Vec<String>,
		pub use_import_as: Option<String>,
		pub use_as_pure: Purity,

		//if local path is not used, will check std and libraries
		pub use_from_local_path: Option<String>,
		pub location: Location,
    }


	pub struct AstModuleDecl {
		pub module_name: String,
		pub module_purity: Purity,
		pub module_callable: Vec<AstFunctionDecl>,
		pub module_using_statements: Vec<AstUsingStatement>,
        pub module_class_decls: Vec<AstClassDecl>,
        pub location: Location,
	}

    pub struct AstClassDecl {
        pub class_name: String,
        pub class_purity: Purity,
        pub class_constructor: Option<AstFunctionDecl>,
        pub class_static: bool,
        pub class_static_methods: Option<Vec<AstFunctionDecl>>,
        pub class_fields: Vec<AstFieldDecl>,
        pub location: Location,
    }

	pub struct AstFieldDecl {
		pub field_name: String,
		pub field_type: AstType,
		pub field_purity: Purity,
		pub location: Location,
	}

	pub struct AstFunctionDecl {
		pub function_name: String,
		pub function_purity: Purity,
		pub function_parameters: Vec<AstParameterDecl>,
		pub function_return_type: Option<AstType>,
		pub function_body: AstBlock,
        pub location: Location,
	}

	pub struct AstParameterDecl {
		pub parameter_name: String,
		pub parameter_type: AstType,
        pub location: Location,
	}

	pub struct AstType {
		pub type_name: String,
		pub type_parameters: Vec<AstType>,
        pub location: Location,
	}

	pub struct AstBlock {
		pub block_statements: Vec<AstExpression>,
        pub location: Location,
	}


    pub enum AstExpression {
        AstBinary(AstBinaryExpression),
        AstUnary(AstUnaryExpression),
        AstLiteral(AstLiteralExpression),
        AstVariableDeclaration(AstVariableDeclarationExpression),
        AstVariableUsage(AstVariableUsageExpression),
        // AstFunctionCall(AstFunctionCallExpression),
        AstSubscript(AstSubscriptExpression),
		AstFieldAccess(AstFieldAccessExpression),
    }

	pub struct AstVariableUsageExpression {
		pub variable_name: String,
		pub location: Location,
		pub usage: Option<Vec<AstValueUsageExpression>>
	}





	//value expression usage reflects that you could do, say...
	// let x = module.class.arrayfield[0](1,2,3)[0]() ... chaining calls together
	// so the [n] and (...params) can be represented as a vec of usages
	pub struct AstFieldAccessExpression {
		pub nested_names: Vec<String>,
		pub usage: Option<Vec<AstValueUsageExpression>>
	}

	pub enum AstValueUsageExpression {
		Subscript(AstSubscriptExpression),
		FunctionCall(AstFunctionCallExpression),
	}

	pub struct AstSubscriptExpression {
		pub subscript_index: Box<AstExpression>,
	}

	pub struct AstFunctionCallExpression {
		pub function_parameters: Vec<AstExpression>,
	}







	pub struct AstVariableDeclarationExpression {
		// pub parent: Box<AstExpression>,

		pub variable_name: String,
		pub variable_type: AstType,
		pub variable_value: Option<Box<AstExpression>>,
	}

	pub struct AstBinaryExpression {
		pub parent: Box<AstExpression>,

		pub binary_operator: BinaryExpressionType,
		pub left_operand: Box<AstExpression>,
		pub right_operand: Box<AstExpression>,
		pub location: Location,
	}

	pub struct AstUnaryExpression {

		pub unary_operator: UnaryExpressionType,
		pub operand: Box<AstExpression>,
		pub location: Location,
	}

	pub enum UnaryExpressionType {
		Negate,
		Not,
		// Dereference,
		// AddressOf,
		// not sure if those are needed / will be implemented
	}

	pub struct AstLiteralExpression {
		pub value: LiteralValue,
		pub location: Location,
	}

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
	pub enum AstValueType {
		Named(Vec<String>),
		Literal,
	}



	pub enum Purity {
		Pure,
		Impure,
	}

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

	pub enum LiteralValue {
		String(String),
		Numeric(NumericLiteralValue),
		Boolean(bool),
	}

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
				file:file.to_string(),
			}
		}

		pub fn default()->Location {
			Location::new(1,1,"test.flx")
		}
	}
}