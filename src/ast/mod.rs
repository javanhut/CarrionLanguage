#[derive(Debug, PartialEq, Clone)]
pub struct Identifier(pub String);

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Expression(Expression),
    FunctionDefinition(FunctionDefinition),
    Return(ReturnStatement),
    If(IfStatement),
    While(WhileStatement),
    For(ForStatement),
    Assignment(Assignment),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    BooleanLiteral(bool),
    List(Vec<Expression>),
    Dict {
        pairs: Vec<(Expression, Expression)>,
    },
    Prefix(PrefixExpression),
    Infix(InfixExpression),
    Index(IndexExpression),
    Call(CallExpression),
}

pub type BlockStatement = Vec<Statement>;

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDefinition {
    pub name: Identifier,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStatement {
    pub value: Option<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfStatement {
    pub condition: Box<Expression>,
    pub consequence: BlockStatement,
    pub alternatives: Vec<(Expression, BlockStatement)>,
    pub default: Option<BlockStatement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhileStatement {
    pub condition: Box<Expression>,
    pub body: BlockStatement,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForStatement {
    pub target: Identifier,
    pub iter: Box<Expression>,
    pub body: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Assignment {
    pub target: Identifier,
    pub value: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    And,
    Or,
    Not,
    PlusPlus,
    MinusMinus,
    PlusPlusIncrement,
    MinusMinusDecrement,
}
#[derive(Debug, PartialEq, Clone)]
pub struct PrefixExpression {
    pub operator: Operator,
    pub right: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct InfixExpression {
    pub left: Box<Expression>,
    pub operator: Operator,
    pub right: Box<Expression>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndexExpression {
    pub object: Box<Expression>,
    pub index: Box<Expression>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct CallExpression {
    pub fuction: Box<Expression>,
    pub arguments: Vec<Expression>,
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    fn new() -> Self {
        Program { statements: vec![] }
    }
}
