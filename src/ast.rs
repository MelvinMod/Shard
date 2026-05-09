use std::fmt;

#[derive(Debug, Clone)]
pub struct CompilationUnit {
    pub nodes: Vec<Node>,
}

#[derive(Debug, Clone)]
pub enum Node {
    Function(FunctionDef),
    Variable(VariableDef),
    Struct(StructDef),
    Enum(EnumDef),
    Expression(Expression),
    Import(ImportDef),
}

#[derive(Debug, Clone)]
pub struct FunctionDef {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub body: Block,
    pub is_pub: bool,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub type_: Type,
}

#[derive(Debug, Clone)]
pub struct VariableDef {
    pub name: String,
    pub type_: Option<Type>,
    pub value: Expression,
    pub is_mut: bool,
}

#[derive(Debug, Clone)]
pub struct StructDef {
    pub name: String,
    pub fields: Vec<Field>,
    pub is_pub: bool,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub type_: Type,
}

#[derive(Debug, Clone)]
pub struct EnumDef {
    pub name: String,
    pub variants: Vec<EnumVariant>,
    pub is_pub: bool,
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub data: Option<Type>,
}

#[derive(Debug, Clone)]
pub struct ImportDef {
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression),
    Variable(VariableDef),
    Return(Option<Expression>),
    If(IfExpr),
    Loop(LoopExpr),
    For(ForExpr),
    Match(MatchExpr),
    Break,
    Continue,
}

#[derive(Debug, Clone)]
pub struct IfExpr {
    pub condition: Box<Expression>,
    pub then_block: Block,
    pub else_block: Option<Block>,
}

#[derive(Debug, Clone)]
pub struct LoopExpr {
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct ForExpr {
    pub variable: String,
    pub iterable: Expression,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct MatchExpr {
    pub value: Box<Expression>,
    pub arms: Vec<MatchArm>,
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Literal(Literal),
    Identifier(String),
    Wildcard,
    EnumVariant(String, Option<String>),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    BinaryOp(Box<Expression>, BinOp, Box<Expression>),
    UnaryOp(UnaryOp, Box<Expression>),
    Call {
        func: Box<Expression>,
        args: Vec<Expression>,
    },
    MethodCall {
        object: Box<Expression>,
        method: String,
        args: Vec<Expression>,
    },
    FieldAccess {
        object: Box<Expression>,
        field: String,
    },
    Index {
        collection: Box<Expression>,
        index: Box<Expression>,
    },
    Tuple(Vec<Expression>),
    Array(Vec<Expression>),
    Block(Block),
    Closure {
        params: Vec<Parameter>,
        body: Box<Block>,
    },
    If(Box<IfExpr>),
    Match(MatchExpr),
    Assign {
        target: Box<Expression>,
        value: Box<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    Char(char),
    String(String),
    Null,
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg,
    Not,
    BitNot,
    Deref,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Int8,
    Int16,
    Int32,
    Int64,
    UInt,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float,
    Float32,
    Float64,
    Bool,
    Bool8,
    Char,
    String,
    Void,
    Nil,
    Pointer(Box<Type>),
    Array(Box<Type>, Option<usize>),
    Slice(Box<Type>),
    Reference(Box<Type>),
    Function(Vec<Type>, Option<Box<Type>>),
    UserDefined(String),
    Generic(String, Vec<Type>),
    Tuple(Vec<Type>),
    Union(Vec<Type>),
    Infer,
    // AI Types
    Tensor,
    Model,
    Layer,
    Dataset,
    NeuralNetwork,
    TrainingConfig,
    InferenceConfig,
    Optimizer,
    LossFunction,
    ActivationFunction,
    Embedding,
    Attention,
    Transformer,
    RNN,
    CNN,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Int => write!(f, "Int"),
            Type::Int8 => write!(f, "Int8"),
            Type::Int16 => write!(f, "Int16"),
            Type::Int32 => write!(f, "Int32"),
            Type::Int64 => write!(f, "Int64"),
            Type::UInt => write!(f, "UInt"),
            Type::UInt8 => write!(f, "UInt8"),
            Type::UInt16 => write!(f, "UInt16"),
            Type::UInt32 => write!(f, "UInt32"),
            Type::UInt64 => write!(f, "UInt64"),
            Type::Float => write!(f, "Float"),
            Type::Float32 => write!(f, "Float32"),
            Type::Float64 => write!(f, "Float64"),
            Type::Bool => write!(f, "Bool"),
            Type::Bool8 => write!(f, "Bool8"),
            Type::Char => write!(f, "Char"),
            Type::String => write!(f, "String"),
            Type::Void => write!(f, "Void"),
            Type::Nil => write!(f, "Nil"),
            Type::Pointer(t) => write!(f, "*{}", t),
            Type::Array(t, size) => {
                if let Some(s) = size {
                    write!(f, "[{}; {}]", t, s)
                } else {
                    write!(f, "[{}]", t)
                }
            }
            Type::Slice(t) => write!(f, "[]{}", t),
            Type::Reference(t) => write!(f, "&{}", t),
            Type::Function(params, ret) => {
                write!(f, "(")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", p)?;
                }
                write!(f, ")")?;
                if let Some(r) = ret {
                    write!(f, " -> {}", r)?;
                }
                Ok(())
            }
            Type::UserDefined(name) => write!(f, "{}", name),
            Type::Generic(name, params) => {
                write!(f, "{}[", name)?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", p)?;
                }
                write!(f, "]")
            }
            Type::Tuple(types) => {
                write!(f, "(")?;
                for (i, t) in types.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", t)?;
                }
                write!(f, ")")
            }
            Type::Union(types) => {
                write!(f, "(")?;
                for (i, t) in types.iter().enumerate() {
                    if i > 0 { write!(f, " | ")?; }
                    write!(f, "{}", t)?;
                }
                write!(f, ")")
            }
            Type::Infer => write!(f, "_"),
            Type::Tensor => write!(f, "Tensor"),
            Type::Model => write!(f, "Model"),
            Type::Layer => write!(f, "Layer"),
            Type::Dataset => write!(f, "Dataset"),
            Type::NeuralNetwork => write!(f, "NeuralNetwork"),
            Type::TrainingConfig => write!(f, "TrainingConfig"),
            Type::InferenceConfig => write!(f, "InferenceConfig"),
            Type::Optimizer => write!(f, "Optimizer"),
            Type::LossFunction => write!(f, "LossFunction"),
            Type::ActivationFunction => write!(f, "ActivationFunction"),
            Type::Embedding => write!(f, "Embedding"),
            Type::Attention => write!(f, "Attention"),
            Type::Transformer => write!(f, "Transformer"),
            Type::RNN => write!(f, "RNN"),
            Type::CNN => write!(f, "CNN"),
        }
    }
}
