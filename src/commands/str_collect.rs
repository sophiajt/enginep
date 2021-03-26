use crate::{CommandArgs, Example, Scope, ValueIterator, WholeStreamCommand};
use nu_errors::ShellError;
use nu_protocol::{ReturnSuccess, Signature, SyntaxShape, UntaggedValue, Value};
use nu_source::Tagged;

pub struct SubCommand;

pub struct SubCommandArgs {
    separator: Option<Tagged<String>>,
}

impl WholeStreamCommand for SubCommand {
    fn name(&self) -> &str {
        "str collect"
    }

    fn signature(&self) -> Signature {
        Signature::build("str collect").desc(self.usage()).optional(
            "separator",
            SyntaxShape::String,
            "the separator to put between the different values",
        )
    }

    fn usage(&self) -> &str {
        "collects a list of strings into a string"
    }

    fn run(&self, args: CommandArgs, _: &Scope) -> Result<ValueIterator, ShellError> {
        collect(args)
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Collect a list of string",
            example: "echo ['a' 'b' 'c'] | str collect",
            result: Some(vec![Value::from("abc")]),
        }]
    }
}

pub fn collect(args: CommandArgs) -> Result<ValueIterator, ShellError> {
    let tag = args.call_info.name_tag.clone();

    let strings: Vec<String> = args
        .input
        .map(|value| match value.as_string() {
            Ok(s) => s,
            Err(e) => {
                println!("Failed at: {:?} {:?}", e, value);
                panic!();
            }
        })
        .collect();

    let output = strings.join("");

    Ok(Box::new(std::iter::once(
        UntaggedValue::string(output).into_value(tag),
    )))
}
