// AST module placeholder

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Statement(Statement),
    // Future: ExpressionNode(Expression), Definition(Definition), etc.
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Print(Box<Expression>),
    Assignment {
        name: String,
        operator: AssignmentOperator, // Changed from direct value to include operator
        value: Box<Expression>,
    },
    If {
        condition: Box<Expression>,
        then_body: Vec<AstNode>,
        elifs: Vec<(Expression, Vec<AstNode>)>,
        else_body: Option<Vec<AstNode>>,
    },
    // Future: For, While, FunctionDef, ClassDef, etc.
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum AssignmentOperator {
    Assign,         // =
    AddAssign,      // +=
    SubAssign,      // -=
    MulAssign,      // *=
    DivAssign,      // /
    ModAssign,      // %
    PowAssign,      // **
    FloorDivAssign, // //
    // Bitwise to be added later
    BitAndAssign,   // &=
    BitOrAssign,    // |=
    BitXorAssign,   // ^=
    LShiftAssign,   // <<=
    RShiftAssign,   // >>=
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)] // Added Copy
pub enum UnaryOp {
    Not,    // Logical NOT
    BitNot, // Bitwise NOT (~)
    // Add UnaryPlus, UnaryMinus if needed explicitly, though often handled by parser context
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    StringLiteral(String),
    IntegerLiteral(i64),
    FloatLiteral(f64), // Added float literal support
    Identifier(String),
    BinaryOperation {
        left: Box<Expression>,
        op: BinOp,
        right: Box<Expression>,
    },
    UnaryOperation { // New variant for unary operations
        op: UnaryOp,
        operand: Box<Expression>,
    },
    // Future: FunctionCall(String, Vec<Expression>), etc.
    Call { /* ... */ },
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)] // Added Copy
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    FloorDiv,

    // Comparison
    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,

    // Logical
    And, // logical and
    Or,  // logical or

    // Bitwise
    BitAnd,   // &
    BitOr,    // |
    BitXor,   // ^
    LShift,   // <<
    RShift,   // >>

    // Identity
    Is,
    IsNot,

    // Membership
    In,
    NotIn,
}
