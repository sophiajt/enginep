use crate::evaluate::evaluate_baseline_expr;
use crate::evaluate::evaluation_context::EvaluationContext;
use crate::ValueIterator;
use nu_errors::ShellError;
use nu_protocol::hir::SpannedExpression;

pub fn run_expression_block(
    expr: &SpannedExpression,
    ctx: &EvaluationContext,
) -> Result<ValueIterator, ShellError> {
    let output = evaluate_baseline_expr(expr, ctx)?;

    Ok(Box::new(std::iter::once(output)))
}
