use crate::{Block, Expr, Ident, Node, types::TypeExpr};

/// A function declaration.
#[derive(Debug, PartialEq)]
pub struct FuncDecl {
    /// The identifier representing the function.
    pub ident: Node<Ident>,
    /// The arguments this function requires.
    pub args: Vec<Node<ParenArgument>>,
    /// The body of the function.
    pub body: Node<Block>,
}

/// A function call.
#[derive(Debug, PartialEq)]
pub struct FuncCall {
    /// The identifier of the function
    pub ident: Node<Ident>,
    /// Arguments being passed to the function.
    pub args: Vec<Node<Expr>>
}

/// An argument to a function call.
#[derive(Debug, PartialEq, Clone)]
pub struct ParenArgument {
    /// The identifier representing the AST node.
    pub ident: Node<Ident>,
    /// The identifier representing the type of this argument.
    pub type_expr: Node<TypeExpr>,
}

#[derive(Debug, PartialEq)]
pub struct ExternFunc {
    /// The identifier representing the external function.
    pub ident: Node<Ident>,
    /// The type of this function.
    /// The arguments this function requires.
    pub args: Vec<Node<ParenArgument>>,
    /// The identifier representing the return type of the function, if there is one.
    pub ret_type_expr: Node<TypeExpr>,
}
