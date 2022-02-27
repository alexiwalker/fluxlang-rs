// dead code allowed because exposed library functions may not all be required by user
#[allow(dead_code)]
pub mod ast_expressions {
	use crate::ast_nodes::ast_nodes::{AstBlock, AstVariableType, LiteralValue, Location};

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

		AstCompilerStub(String),
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
}
