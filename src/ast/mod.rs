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
        value: Box<Expression>,
    },
    // Future: If, For, While, FunctionDef, ClassDef, etc.
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    StringLiteral(String),
    IntegerLiteral(i64),
    Identifier(String),
    BinaryOperation {
        left: Box<Expression>,
        op: BinOp,
        right: Box<Expression>,
    },
    // Future: FloatLiteral(f64), FunctionCall(String, Vec<Expression>), etc.
    Call { /* ... */ },
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    FloorDiv,
    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    And,
    Or,
    BitAnd,
    BitOr,
    BitXor,
    LShift,
    RShift,
}
