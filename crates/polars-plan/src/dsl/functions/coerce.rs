use super::*;

/// Take several expressions and collect them into a [`StructChunked`].
/// # Panics
/// panics if `exprs` is empty.
pub fn as_struct(exprs: Vec<Expr>) -> Expr {
    Expr::n_ary(FunctionExpr::AsStruct, exprs)
}
