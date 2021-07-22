use crate::expr::Expr;
use crate::types::Type;

pub mod expr;
pub mod func;
pub mod scope;
pub mod types;
pub mod util;
pub mod var;

/// An enum of binary operation types.
pub enum BinOpKind {
    /// Represents the "+" operator.
    Add,
    /// Represents the "-" operator.
    Sub,
    /// Represents the "*" operator.
    Mul,
    /// Represents the "/" operator.
    Div,
    /// Represents the "%" operator.
    Mod,
    /// Represents the "==" operator.
    Eq,
    /// Represents the "<" operator.
    Lt,
    /// Represents the ">" operator.
    Gt,
    /// Represents the "<=" operator.
    Le,
    /// Represents the ">=" operator.
    Ge,
    /// Represents the ">>" operator.
    Shr,
    /// Represents the "<<" operator.
    Shl,
}

impl BinOpKind {
    /// Convert this operator to its string representation.
    pub fn to_string(&self) -> &'static str {
        use BinOpKind::*;
        match *self {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
            Mod => "%",
            Eq => "==",
            Lt => "<",
            Gt => ">",
            Le => "<=",
            Ge => ">=",
            Shr => ">>",
            Shl => "<<",
        }
    }

    /// Test if this operation is a comparison operator.
    pub fn is_comparison(&self) -> bool {
        use BinOpKind::*;
        match *self {
            Eq | Lt | Gt | Le | Ge => true,
            Add | Sub | Mul | Div | Mod | Shr | Shl => false,
        }
    }
}

/// An enum of unary operation types.
pub enum UnOpKind {
    /// Represents the "!" operator.
    Not,
    /// Represents the "-" operator.
    Neg,
}

impl UnOpKind {
    /// Convert this operator into its string representation.
    pub fn to_string(&self) -> &'static str {
        use UnOpKind::*;
        match *self {
            Not => "!",
            Neg => "-",
        }
    }
}

/// Represents a node in the AST tree.
pub struct Node {}
/// Represents an identifier.
#[derive(Debug, Clone)]
pub struct Ident {
    /// The ID of this identifier in the AST.
    pub id: i64,
    /// The raw string name of this identifier.
    pub val: String,
}

/// An enum of mutability state.
/// e.g. a variable is mutable but a constant is not.
#[derive(Debug, Clone)]
pub enum Mutability {
    /// Represents an immutable state.
    Not,
    /// Represents a mutable state.
    Mut,
}

/// Represents a variable.
pub struct Variable {
    /// The identifier of this variable.
    pub ident: Ident,
    /// The mutability status of this variable.
    pub mutable: Mutability,
    /// The type of this variable.
    pub ty: Type,
}

/// Represents a block.
pub struct Block {
    /// The parent block.
    pub parent: Option<Box<Block>>,
    /// Expressions contained within this block.
    pub exprs: Vec<Expr>,
}

/// Represents a function paretheses argument.
pub struct ParenArgument {
    /// The identifier of this paramenter.
    pub name: Ident,
    /// The type of this parameter.
    pub ty: Type,
}

/// Represents a function declaration.
pub struct FuncDeclare {
    /// The identifier of this function.
    ident: Ident,
    /// The method block of the function.
    method: Block,
    /// The parentheses arguments given to this function.
    args: Vec<ParenArgument>,
    /// The return type of the function.
    ret_ty: Type,
}
