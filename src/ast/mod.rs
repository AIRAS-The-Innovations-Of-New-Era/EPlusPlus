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
    While {
        condition: Box<Expression>,
        body: Vec<AstNode>,
    },
    For {
        var: String,
        iterable: Box<Expression>,
        body: Vec<AstNode>,
    },
    FunctionDef {
        name: String,
        params: Vec<String>,
        body: Vec<AstNode>,
    },
    Return(Option<Box<Expression>>),
    ExpressionStatement(Box<Expression>), // Added for standalone expressions
    Break,                              // Added for break statements
    Continue,                           // Added for continue statements
    Pass,                               // Added for pass statements
    #[allow(dead_code)] // Allowed because it's a planned feature
    ClassDef {
        name: String,
        body: Vec<AstNode>, // Simplified: can contain assignments (members) or defs (methods)
    },
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
    FloatLiteral(f64),
    BooleanLiteral(bool),
    NoneLiteral, // Added for None
    ListLiteral(Vec<Expression>), // Added for list literals
    TupleLiteral(Vec<Expression>), // For tuple literals
    DictLiteral(Vec<(Expression, Expression)>), // For dict literals
    SetLiteral(Vec<Expression>), // For set literals
    FrozensetLiteral(Vec<Expression>), // For frozenset literals
    ComplexLiteral(Box<Expression>, Box<Expression>), // For complex(a, b)
    Identifier(String),
    BinaryOperation {
        left: Box<Expression>,
        op: BinOp,
        right: Box<Expression>,
    },    UnaryOperation { // New variant for unary operations
        op: UnaryOp,
        operand: Box<Expression>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
    ListComprehension {
        element: Box<Expression>,
        var: String,
        iter: Box<Expression>,
        condition: Option<Box<Expression>>,
    }, // For list comprehensions (future)
    // Future: etc.
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
