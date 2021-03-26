use std::sync::Arc;

use crate::{evaluate_baseline_expr, EvaluatedWholeStreamCommandArgs, Example, ValueIterator};
use crate::{run_block, CommandArgs, EvaluationContext};
use crate::{Scope, WholeStreamCommand};
use nu_errors::ShellError;
use nu_parser::ParserScope;
use nu_protocol::{
    hir::CapturedBlock, hir::ClassifiedCommand, Signature, SyntaxShape, UntaggedValue, Value,
};

pub struct If;

pub struct IfArgs {
    condition: CapturedBlock,
    then_case: CapturedBlock,
    else_case: CapturedBlock,
}

impl WholeStreamCommand for If {
    fn name(&self) -> &str {
        "if"
    }

    fn signature(&self) -> Signature {
        Signature::build("if")
            .required(
                "condition",
                SyntaxShape::MathExpression,
                "the condition that must match",
            )
            .required(
                "then_case",
                SyntaxShape::Block,
                "block to run if condition is true",
            )
            .required(
                "else_case",
                SyntaxShape::Block,
                "block to run if condition is false",
            )
    }

    fn usage(&self) -> &str {
        "Run blocks if a condition is true or false."
    }

    fn run(&self, args: CommandArgs, scope: &Scope) -> Result<ValueIterator, ShellError> {
        if_command(args, scope)
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Run a block if a condition is true",
                example: "let x = 10; if $x > 5 { echo 'greater than 5' } { echo 'less than or equal to 5' }",
                result: Some(vec![UntaggedValue::string("greater than 5").into()]),
            },
            Example {
                description: "Run a block if a condition is false",
                example: "let x = 1; if $x > 5 { echo 'greater than 5' } { echo 'less than or equal to 5' }",
                result: Some(vec![UntaggedValue::string("less than or equal to 5").into()]),
            },
        ]
    }
}
fn if_command(raw_args: CommandArgs, scope: &Scope) -> Result<ValueIterator, ShellError> {
    let tag = raw_args.call_info.name_tag.clone();
    let context = Arc::new(EvaluationContext::from_args(&raw_args));

    let tag = raw_args.call_info.name_tag.clone();
    let ctx = EvaluationContext::from_args(&raw_args);

    let EvaluatedWholeStreamCommandArgs { args, input } = raw_args.evaluate_once(scope)?;
    let condition = match args.nth(0).unwrap() {
        Value {
            value: UntaggedValue::Block(x),
            tag,
        } => x,
        Value { tag, .. } => {
            return Err(ShellError::labeled_error(
                "Expected a variable name",
                "expected a variable name",
                tag.span,
            ))
        }
    };
    let then_case = match args.nth(1).unwrap() {
        Value {
            value: UntaggedValue::Block(x),
            tag,
        } => x,
        Value { tag, .. } => {
            return Err(ShellError::labeled_error(
                "Expected a variable name",
                "expected a variable name",
                tag.span,
            ))
        }
    };
    let else_case = match args.nth(2).unwrap() {
        Value {
            value: UntaggedValue::Block(x),
            tag,
        } => x,
        Value { tag, .. } => {
            return Err(ShellError::labeled_error(
                "Expected a variable name",
                "expected a variable name",
                tag.span,
            ))
        }
    };

    let cond = {
        if condition.block.block.len() != 1 {
            return Err(ShellError::labeled_error(
                "Expected a condition",
                "expected a condition",
                tag,
            ));
        }
        match condition.block.block[0].pipelines.get(0) {
            Some(item) => match item.list.get(0) {
                Some(ClassifiedCommand::Expr(expr)) => expr.clone(),
                _ => {
                    return Err(ShellError::labeled_error(
                        "Expected a condition",
                        "expected a condition",
                        tag,
                    ));
                }
            },
            None => {
                return Err(ShellError::labeled_error(
                    "Expected a condition",
                    "expected a condition",
                    tag,
                ));
            }
        }
    };

    scope.enter_scope();
    scope.add_vars(&condition.captured.entries);

    //FIXME: should we use the scope that's brought in as well?
    let condition = evaluate_baseline_expr(&cond, &*context, scope);
    match condition {
        Ok(condition) => match condition.as_bool() {
            Ok(b) => {
                let result = if b {
                    run_block(&then_case.block, &*context, scope, input)
                } else {
                    run_block(&else_case.block, &*context, scope, input)
                };
                scope.exit_scope();

                result
            }
            Err(e) => Ok(Box::new(std::iter::once(
                UntaggedValue::Error(e).into_untagged_value(),
            ))),
        },
        Err(e) => Ok(Box::new(std::iter::once(
            UntaggedValue::Error(e).into_untagged_value(),
        ))),
    }
}
