use crate::{
    empty_value_iterator, CommandArgs, EvaluationContext, Example, Scope, ValueIterator,
    WholeStreamCommand,
};

use nu_errors::ShellError;
use nu_parser::ParserScope;
use nu_protocol::{hir::CapturedBlock, hir::ClassifiedCommand, Signature, SyntaxShape};
use nu_source::Tagged;

pub struct Let;

impl WholeStreamCommand for Let {
    fn name(&self) -> &str {
        "let"
    }

    fn signature(&self) -> Signature {
        Signature::build("let")
            .required("name", SyntaxShape::String, "the name of the variable")
            .required("equals", SyntaxShape::String, "the equals sign")
            .required(
                "expr",
                SyntaxShape::MathExpression,
                "the value for the variable",
            )
    }

    fn usage(&self) -> &str {
        "Create a variable and give it a value."
    }

    fn run(&self, args: CommandArgs, scope: &Scope) -> Result<ValueIterator, ShellError> {
        letcmd(args, scope)
    }

    fn examples(&self) -> Vec<Example> {
        vec![]
    }
}

pub fn letcmd(args: CommandArgs, scope: &Scope) -> Result<ValueIterator, ShellError> {
    let tag = args.call_info.name_tag.clone();
    let ctx = EvaluationContext::from_args(&args);

    // let (LetArgs { name, rhs, .. }, _) = args.process().await?;

    // let name: Tagged<String> = args.call_info.args.positional.unwrap().get(0).unwrap();

    let (expr, captured) = {
        if rhs.block.block.len() != 1 {
            return Err(ShellError::labeled_error(
                "Expected a value",
                "expected a value",
                tag,
            ));
        }
        match rhs.block.block[0].pipelines.get(0) {
            Some(item) => match item.list.get(0) {
                Some(ClassifiedCommand::Expr(expr)) => (expr.clone(), rhs.captured.clone()),
                _ => {
                    return Err(ShellError::labeled_error(
                        "Expected a value",
                        "expected a value",
                        tag,
                    ));
                }
            },
            None => {
                return Err(ShellError::labeled_error(
                    "Expected a value",
                    "expected a value",
                    tag,
                ));
            }
        }
    };

    scope.enter_scope();
    scope.add_vars(&captured.entries);

    let value = evaluate_baseline_expr(&expr, &ctx).await;

    scope.exit_scope();

    let value = value?;

    let name = if name.item.starts_with('$') {
        name.item.clone()
    } else {
        format!("${}", name.item)
    };

    // Note: this is a special case for setting the context from a command
    // In this case, if we don't set it now, we'll lose the scope that this
    // variable should be set into.
    scope.add_var(name, value);

    Ok(empty_value_iterator())
}
