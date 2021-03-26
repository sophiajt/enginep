use std::sync::Arc;

use crate::{
    empty_value_iterator, run_block, CommandArgs, EvaluationContext, Example, ValueIterator,
};
use crate::{Scope, WholeStreamCommand};

use nu_errors::ShellError;
use nu_parser::ParserScope;
use nu_protocol::{
    hir::CapturedBlock, Signature, SyntaxShape, TaggedDictBuilder, UntaggedValue, Value,
};

pub struct Each;

impl WholeStreamCommand for Each {
    fn name(&self) -> &str {
        "each"
    }

    fn signature(&self) -> Signature {
        Signature::build("each")
            .required("block", SyntaxShape::Block, "the block to run on each row")
            .switch(
                "numbered",
                "returned a numbered item ($it.index and $it.item)",
                Some('n'),
            )
    }

    fn usage(&self) -> &str {
        "Run a block on each row of the table."
    }

    fn run(&self, args: CommandArgs, scope: &Scope) -> Result<ValueIterator, ShellError> {
        each(args, scope)
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Echo the sum of each row",
                example: "echo [[1 2] [3 4]] | each { echo $it | math sum }",
                result: None,
            },
            Example {
                description: "Echo the square of each integer",
                example: "echo [1 2 3] | each { echo $(= $it * $it) }",
                result: Some(vec![
                    UntaggedValue::int(1).into(),
                    UntaggedValue::int(4).into(),
                    UntaggedValue::int(9).into(),
                ]),
            },
            Example {
                description: "Number each item and echo a message",
                example:
                    "echo ['bob' 'fred'] | each --numbered { echo `{{$it.index}} is {{$it.item}}` }",
                result: Some(vec![Value::from("0 is bob"), Value::from("1 is fred")]),
            },
        ]
    }
}

pub fn process_row(
    captured_block: Arc<Box<CapturedBlock>>,
    context: Arc<EvaluationContext>,
    scope: &Scope,
    input: Value,
) -> Result<ValueIterator, ShellError> {
    let input_clone = input.clone();
    // When we process a row, we need to know whether the block wants to have the contents of the row as
    // a parameter to the block (so it gets assigned to a variable that can be used inside the block) or
    // if it wants the contents as as an input stream

    let input_stream = if !captured_block.block.params.positional.is_empty() {
        empty_value_iterator()
    } else {
        Box::new(std::iter::once(input_clone))
    };

    scope.enter_scope();
    scope.add_vars(&captured_block.captured.entries);

    if !captured_block.block.params.positional.is_empty() {
        // FIXME: add check for more than parameter, once that's supported
        scope.add_var(captured_block.block.params.positional[0].0.name(), input);
    } else {
        scope.add_var("$it", input);
    }

    let result = run_block(&captured_block.block, &*context, scope, input_stream);

    scope.exit_scope();

    Ok(result?)
}

pub(crate) fn make_indexed_item(index: usize, item: Value) -> Value {
    let mut dict = TaggedDictBuilder::new(item.tag());
    dict.insert_untagged("index", UntaggedValue::int(index));
    dict.insert_value("item", item);

    dict.into_value()
}

fn each(raw_args: CommandArgs, scope: &Scope) -> Result<ValueIterator, ShellError> {
    let context = Arc::new(EvaluationContext::from_args(&raw_args));

    //let (each_args, input): (EachArgs, _) = raw_args.process().await?;
    let ctx = EvaluationContext::from_args(&raw_args);

    let args = raw_args.evaluate_once(scope)?;
    let block = match args.nth(0).unwrap() {
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

    let block = Arc::new(block.clone());
    let scope = Arc::new(scope.clone());

    Ok(Box::new(
        args.input
            .map(move |input| {
                let block = block.clone();
                let context = context.clone();
                let scope = scope.clone();

                match process_row(block, context, &scope, input) {
                    Ok(s) => s,
                    Err(e) => Box::new(std::iter::once(
                        UntaggedValue::Error(e).into_untagged_value(),
                    )) as ValueIterator,
                }
            })
            .flatten(),
    ))
}
