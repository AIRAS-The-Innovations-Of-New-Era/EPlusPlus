// AST module placeholder

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Statement(Statement),
    // Future: ExpressionNode(Expression), Definition(Definition), etc.
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Assignment {
        target: Box<Expression>, // Changed from name: String to target: Box<Expression>
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
    },    For {
        vars: Vec<String>,  // Changed from single var to multiple vars
        iterable: Box<Expression>,
        body: Vec<AstNode>,
    },FunctionDef {
        name: String,
        params: Vec<String>,
        body: Vec<AstNode>,
        decorators: Vec<Decorator>, // Added decorators support
    },
    Print(Box<Expression>),
    Return(Option<Box<Expression>>),
    ExpressionStatement(Box<Expression>), // Added for standalone expressions
    Break,                              // Added for break statements
    Continue,                           // Added for continue statements
    Pass,                               // Added for pass statements
    #[allow(dead_code)] // Allowed because it's a planned feature
    ClassDef {
        name: String,
        base: Option<String>, // Optional base class name for inheritance
        body: Vec<AstNode>, // Contains assignments (attributes) and function definitions (methods, including __init__)
    },
    TryExcept {
        try_body: Vec<AstNode>,
        excepts: Vec<ExceptHandler>,
        else_body: Option<Vec<AstNode>>,
        finally_body: Option<Vec<AstNode>>,
    },
    Raise(Option<Expression>),
    With {
        items: Vec<WithItem>,
        body: Vec<AstNode>,
    },
    Yield(Option<Box<Expression>>), // For generator functions: yield or yield value
}

#[derive(Debug, Clone, PartialEq)]
pub enum Decorator {
    Simple(String),                           // @decorator_name
    WithArgs(String, Vec<Argument>),          // @decorator_name(arg1, name=arg2, ...)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Argument {
    Positional(Expression),                   // func(expr)
    Keyword(String, Expression),              // func(name=expr)
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
    Negate, // Unary minus (-)
    // UnaryPlus is typically a no-op, so we don't need a variant for it
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
    Lambda {
        params: Vec<String>,
        body: Box<Expression>,
    },
    // Comprehensions
    ListComprehension {
        element: Box<Expression>,
        comprehension: Comprehension,
    },
    DictComprehension {
        key: Box<Expression>,
        value: Box<Expression>,
        comprehension: Comprehension,
    },
    SetComprehension {
        element: Box<Expression>,
        comprehension: Comprehension,
    },
    GeneratorExpression {
        element: Box<Expression>,
        comprehension: Comprehension,
    },
    Call {
        callee: Box<Expression>,
        args: Vec<Expression>,
    },
    AttributeAccess {
        object: Box<Expression>,
        attr: String,
    },
    Index {
        object: Box<Expression>,
        index: Box<Expression>,
    },
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

#[derive(Debug, Clone, PartialEq)]
pub struct ExceptHandler {
    pub exception_type: Option<Expression>,
    pub name: Option<String>,
    pub body: Vec<AstNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WithItem {
    pub context_expr: Expression,
    pub optional_vars: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Comprehension {
    pub target: Vec<String>,    // Variable names (e.g., ["k", "v"] in "for k, v in items()")
    pub iter: Box<Expression>,  // Iterable expression (e.g., "range(10)")
    pub ifs: Vec<Expression>,   // Optional if conditions (e.g., "if x % 2 == 0")
    pub is_async: bool,         // For future async comprehensions
}
