#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    UnaryMinus(Box<Expr>),
    BinOp {
        left: Box<Expr>,
        op: BinOpKind,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
    Modulo,
    Pow,
}

impl Expr {
    pub fn binary(left: Expr, op: BinOpKind, right: Expr) -> Self {
        Expr::BinOp {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }

    pub fn unary_minus(expr: Expr) -> Self {
        Expr::UnaryMinus(Box::new(expr))
    }

    pub fn number(n: f64) -> Self {
        Expr::Number(n)
    }
}
