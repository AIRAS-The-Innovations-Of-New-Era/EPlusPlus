// AST module placeholder

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Not,
    BitNot,
    Negate,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
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
    And,
    Or,

    // Bitwise
    BitAnd,
    BitOr,
    BitXor,
    LShift,
    RShift,
    Assign, // Added for keyword argument handling in dict constructor
    Is,
    IsNot,
    In,
    NotIn,
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
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    StringLiteral(String),
    IntegerLiteral(i64),
    FloatLiteral(f64),
    BooleanLiteral(bool),
    NoneLiteral,
    ListLiteral(Vec<Expression>),
    TupleLiteral(Vec<Expression>),
    DictLiteral(Vec<(Expression, Expression)>),
    SetLiteral(Vec<Expression>),
    FrozensetLiteral(Vec<Expression>),
    ComplexLiteral(Box<Expression>, Box<Expression>),
    Identifier(String),
    BinaryOperation { left: Box<Expression>, op: BinOp, right: Box<Expression> },
    UnaryOperation { op: UnaryOp, operand: Box<Expression> },
    Lambda { params: Vec<String>, body: Box<Expression> },
    ListComprehension { element: Box<Expression>, var: String, iter: Box<Expression>, condition: Option<Box<Expression>> },
    Call { callee: Box<Expression>, args: Vec<Expression> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Argument {
    Positional(Expression),
    Keyword(String, Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Decorator {
    Simple(String),
    WithArgs(String, Vec<Argument>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Print(Box<Expression>),
    Assignment { name: String, operator: AssignmentOperator, value: Box<Expression> },
    If { condition: Box<Expression>, then_body: Vec<AstNode>, elifs: Vec<(Expression, Vec<AstNode>)>, else_body: Option<Vec<AstNode>> },
    While { condition: Box<Expression>, body: Vec<AstNode> },
    For { vars: Vec<String>, iterable: Box<Expression>, body: Vec<AstNode> },
    FunctionDef { name: String, params: Vec<String>, body: Vec<AstNode>, decorators: Vec<Decorator> },
    Return(Option<Box<Expression>>),
    ExpressionStatement(Box<Expression>),
    Break,
    Continue,
    Pass,
    ClassDef { name: String, base_class: Option<Box<Expression>>, body: Vec<AstNode> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Statement(Box<Statement>),
}
