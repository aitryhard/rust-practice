mod ast;
mod error;
mod evaluator;
mod parser;
mod token;

pub use error::CalcError;

/// Parse and evaluate a mathematical expression string.
///
/// Supports `+`, `-`, `*`, `/`, `%`, `^` (power), parentheses `()`, unary minus,
/// and floating-point numbers.
///
/// # Examples
///
/// ```
/// let result = calculator::calculate("2 + 3 * 4").unwrap();
/// assert_eq!(result, 14.0);
/// ```
pub fn calculate(expression: &str) -> Result<f64, CalcError> {
    let tokens = token::Lexer::new(expression).tokenize()?;
    let ast = parser::Parser::new(tokens).parse()?;
    ast.eval()
}
