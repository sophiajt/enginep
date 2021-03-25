use crate::evaluate::evaluate_baseline_expr;
use crate::evaluate::evaluation_context::EvaluationContext;
use crate::ValueIterator;
use nu_errors::ShellError;
use nu_protocol::hir::SpannedExpression;

use super::Scope;

pub fn run_expression_block(
    expr: &SpannedExpression,
    ctx: &EvaluationContext,
    scope: &Scope,
) -> Result<ValueIterator, ShellError> {
    let output = evaluate_baseline_expr(expr, ctx, scope)?;

    Ok(Box::new(std::iter::once(output)))
}
