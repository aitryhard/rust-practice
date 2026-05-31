use crate::ast::{BinOpKind, Expr};
use crate::error::CalcError;

impl Expr {
    pub fn eval(&self) -> Result<f64, CalcError> {
        match self {
            Expr::Number(n) => Ok(*n),
            Expr::UnaryMinus(inner) => {
                let val = inner.eval()?;
                Ok(-val)
            }
            Expr::BinOp { left, op, right } => {
                let lhs = left.eval()?;
                let rhs = right.eval()?;
                match op {
                    BinOpKind::Add => Ok(lhs + rhs),
                    BinOpKind::Sub => Ok(lhs - rhs),
                    BinOpKind::Mul => Ok(lhs * rhs),
                    BinOpKind::Div => {
                        if rhs == 0.0 {
                            Err(CalcError::EvalError("division by zero".into()))
                        } else {
                            Ok(lhs / rhs)
                        }
                    }
                    BinOpKind::Modulo => {
                        if rhs == 0.0 {
                            Err(CalcError::EvalError("modulo by zero".into()))
                        } else {
                            Ok(lhs % rhs)
                        }
                    }
                    BinOpKind::Pow => Ok(lhs.powf(rhs)),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{calculate, error::CalcError};

    #[test]
    fn eval_addition() {
        assert_eq!(calculate("2 + 3").unwrap(), 5.0);
    }

    #[test]
    fn eval_subtraction() {
        assert_eq!(calculate("10 - 3").unwrap(), 7.0);
    }

    #[test]
    fn eval_multiplication() {
        assert_eq!(calculate("4 * 5").unwrap(), 20.0);
    }

    #[test]
    fn eval_division() {
        assert_eq!(calculate("10 / 4").unwrap(), 2.5);
    }

    #[test]
    fn eval_modulo() {
        assert_eq!(calculate("10 % 3").unwrap(), 1.0);
    }

    #[test]
    fn eval_power() {
        assert_eq!(calculate("2 ^ 10").unwrap(), 1024.0);
    }

    #[test]
    fn eval_precedence() {
        assert_eq!(calculate("2 + 3 * 4").unwrap(), 14.0);
        assert_eq!(calculate("(2 + 3) * 4").unwrap(), 20.0);
    }

    #[test]
    fn eval_unary_minus() {
        assert_eq!(calculate("-5 + 3").unwrap(), -2.0);
        assert_eq!(calculate("-(2 + 3)").unwrap(), -5.0);
    }

    #[test]
    fn eval_right_associative_power() {
        assert_eq!(calculate("2 ^ 3 ^ 2").unwrap(), 512.0);
    }

    #[test]
    fn eval_nested() {
        assert_eq!(
            calculate("((2 + 3) * (4 - 1)) / 5").unwrap(),
            3.0
        );
    }

    #[test]
    fn eval_float() {
        assert!((calculate("3.14 * 2.0").unwrap() - 6.28).abs() < 1e-10);
    }

    #[test]
    fn eval_division_by_zero() {
        let result = calculate("1 / 0");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), CalcError::EvalError("division by zero".into()));
    }

    #[test]
    fn eval_modulo_by_zero() {
        let result = calculate("1 % 0");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), CalcError::EvalError("modulo by zero".into()));
    }

    #[test]
    fn eval_complex() {
        let result = calculate("-3 * (4 + 6) ^ 2 + 5 * 2.0 / 4 - 1");
        assert!((result.unwrap() - (-298.5)).abs() < 1e-10);
    }
}
