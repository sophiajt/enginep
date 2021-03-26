use nu_errors::ShellError;

use nu_data::value::format_leaf;
use nu_protocol::{ReturnSuccess, Signature, SyntaxShape, UntaggedValue, Value};
use nu_source::PrettyDebug;

use crate::{CommandArgs, EvaluationContext, Example, Scope, ValueIterator, WholeStreamCommand};

pub struct BuildStringArgs {
    rest: Vec<Value>,
}

pub struct BuildString;

impl WholeStreamCommand for BuildString {
    fn name(&self) -> &str {
        "build-string"
    }

    fn signature(&self) -> Signature {
        Signature::build("build-string")
            .rest(SyntaxShape::Any, "all values to form into the string")
    }

    fn usage(&self) -> &str {
        "Builds a string from the arguments."
    }

    fn run(&self, args: CommandArgs, scope: &Scope) -> Result<ValueIterator, ShellError> {
        let tag = args.call_info.name_tag.clone();

        let ctx = EvaluationContext::from_args(&args);

        let args = args.evaluate_once(scope)?;
        let start = args.nth(0).unwrap();
        let end = args.nth(1).unwrap();

        let mut output_string = String::new();
        output_string.push_str(&format_leaf(&start.value).plain_string(100_000));
        output_string.push_str(&format_leaf(&end.value).plain_string(100_000));

        Ok(Box::new(std::iter::once(
            UntaggedValue::string(output_string).into_value(tag),
        )))
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Builds a string from a string and a number, without spaces between them",
            example: "build-string 'foo' 3",
            result: None,
        }]
    }
}
