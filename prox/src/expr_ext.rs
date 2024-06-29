use easy_ext::ext;

#[ext(ExprExt)]
pub impl syn::Expr {
    /// Recursively strips the [`syn::Expr::Group`] wrappers
    fn strip_group(&self) -> &Self {
        match self {
            Self::Group(group) => group.expr.strip_group(),
            _ => self,
        }
    }
}
