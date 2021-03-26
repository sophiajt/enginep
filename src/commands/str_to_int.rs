use crate::{CommandArgs, Example, Scope, ValueIterator, WholeStreamCommand};
use nu_errors::ShellError;
use nu_protocol::ShellTypeName;
use nu_protocol::{
    ColumnPath, Primitive, ReturnSuccess, Signature, SyntaxShape, UntaggedValue, Value,
};
use nu_source::{Tag, Tagged};
use nu_value_ext::ValueExt;

use num_bigint::BigInt;
use num_traits::Num;

pub struct SubCommand;

impl WholeStreamCommand for SubCommand {
    fn name(&self) -> &str {
        "str to-int"
    }

    fn signature(&self) -> Signature {
        Signature::build("str to-int")
            .named("radix", SyntaxShape::Number, "radix of integer", Some('r'))
            .rest(
                SyntaxShape::ColumnPath,
                "optionally convert text into integer by column paths",
            )
    }

    fn usage(&self) -> &str {
        "converts text into integer"
    }

    fn run(&self, args: CommandArgs, _: &Scope) -> Result<ValueIterator, ShellError> {
        operate(args)
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Convert to an integer",
                example: "echo '255' | str to-int",
                result: Some(vec![UntaggedValue::int(255).into()]),
            },
            Example {
                description: "Convert str column to an integer",
                example: "echo [['count']; ['255']] | str to-int count | get count",
                result: Some(vec![UntaggedValue::int(255).into()]),
            },
            Example {
                description: "Convert to integer from binary",
                example: "echo '1101' | str to-int -r 2",
                result: Some(vec![UntaggedValue::int(13).into()]),
            },
            Example {
                description: "Convert to integer from hex",
                example: "echo 'FF' | str to-int -r 16",
                result: Some(vec![UntaggedValue::int(255).into()]),
            },
        ]
    }
}

fn operate(args: CommandArgs) -> Result<ValueIterator, ShellError> {
    let radix = 10;

    Ok(Box::new(
        args.input.map(move |v| action(&v, v.tag(), radix).unwrap()),
    ))
}

fn action(input: &Value, tag: impl Into<Tag>, radix: u32) -> Result<Value, ShellError> {
    match &input.value {
        UntaggedValue::Primitive(Primitive::String(s)) => {
            let trimmed = s.trim();

            let out = match trimmed {
                b if b.starts_with("0b") => {
                    let num = match BigInt::from_str_radix(b.trim_start_matches("0b"), 2) {
                        Ok(n) => n,
                        Err(reason) => {
                            return Err(ShellError::labeled_error(
                                "could not parse as integer",
                                reason.to_string(),
                                tag.into().span,
                            ))
                        }
                    };
                    UntaggedValue::int(num)
                }
                h if h.starts_with("0x") => {
                    let num = match BigInt::from_str_radix(h.trim_start_matches("0x"), 16) {
                        Ok(n) => n,
                        Err(reason) => {
                            return Err(ShellError::labeled_error(
                                "could not parse as int",
                                reason.to_string(),
                                tag.into().span,
                            ))
                        }
                    };
                    UntaggedValue::int(num)
                }
                _ => {
                    let num = match BigInt::from_str_radix(trimmed, radix) {
                        Ok(n) => n,
                        Err(reason) => {
                            return Err(ShellError::labeled_error(
                                "could not parse as int",
                                reason.to_string(),
                                tag.into().span,
                            ))
                        }
                    };
                    UntaggedValue::int(num)
                }
            };

            Ok(out.into_value(tag))
        }
        other => {
            let got = format!("got {}", other.type_name());
            Err(ShellError::labeled_error(
                "value is not string",
                got,
                tag.into().span,
            ))
        }
    }
}
