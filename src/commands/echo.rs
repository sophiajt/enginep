use crate::{CommandArgs, Example, Scope, ValueIterator, WholeStreamCommand};
use nu_errors::ShellError;
use nu_protocol::hir::Operator;
use nu_protocol::{Primitive, Range, RangeInclusion, Signature, SyntaxShape, UntaggedValue, Value};
use nu_source::{SpannedItem, Tag};

pub struct Echo;

#[derive(Debug)]
pub struct EchoArgs {
    pub rest: Vec<Value>,
}

impl WholeStreamCommand for Echo {
    fn name(&self) -> &str {
        "echo"
    }

    fn signature(&self) -> Signature {
        Signature::build("echo").rest(SyntaxShape::Any, "the values to echo")
    }

    fn usage(&self) -> &str {
        "Echo the arguments back to the user."
    }

    fn run(&self, args: CommandArgs, scope: &Scope) -> Result<ValueIterator, ShellError> {
        echo(args, scope)
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Put a hello message in the pipeline",
                example: "echo 'hello'",
                result: Some(vec![Value::from("hello")]),
            },
            Example {
                description: "Print the value of the special '$nu' variable",
                example: "echo $nu",
                result: None,
            },
        ]
    }
}

fn echo(args: CommandArgs, scope: &Scope) -> Result<ValueIterator, ShellError> {
    let args = args.evaluate_once(scope)?;
    let rest = args.call_info.args.positional.clone().unwrap();

    let stream: ValueIterator = Box::new(
        rest.into_iter()
            .map(|i| match i.as_string() {
                Ok(s) => Box::new(std::iter::once(
                    UntaggedValue::string(s).into_value(i.tag.clone()),
                )),
                _ => match i {
                    Value {
                        value: UntaggedValue::Table(table),
                        ..
                    } => Box::new(table.into_iter().map(|x| x.clone())) as ValueIterator,
                    Value {
                        value: UntaggedValue::Primitive(Primitive::Range(range)),
                        tag,
                    } => Box::new(RangeIterator::new(range, tag)) as ValueIterator,
                    x => Box::new(std::iter::once(x.clone())),
                },
            })
            .flatten(),
    );

    Ok(stream)
}

struct RangeIterator {
    curr: Primitive,
    end: Primitive,
    tag: Tag,
    is_end_inclusive: bool,
    moves_up: bool,
}

impl RangeIterator {
    pub fn new(range: Box<Range>, tag: Tag) -> RangeIterator {
        let start = match range.from.0.item {
            Primitive::Nothing => Primitive::Int(0.into()),
            x => x,
        };

        let end = match range.to.0.item {
            Primitive::Nothing => Primitive::Int(u64::MAX.into()),
            x => x,
        };

        RangeIterator {
            moves_up: start <= end,
            curr: start,
            end,
            tag: tag.clone(),
            is_end_inclusive: matches!(range.to.1, RangeInclusion::Inclusive),
        }
    }
}

impl Iterator for RangeIterator {
    type Item = Value;
    fn next(&mut self) -> Option<Self::Item> {
        let ordering = if self.end == Primitive::Nothing {
            Ordering::Less
        } else {
            let result =
                nu_data::base::coerce_compare_primitive(&self.curr, &self.end).map_err(|_| {
                    ShellError::labeled_error(
                        "Cannot create range",
                        "unsupported range",
                        self.tag.span,
                    )
                });

            if let Err(result) = result {
                return Some(UntaggedValue::Error(result).into_untagged_value());
            }

            let result = result
                .expect("Internal error: the error case was already protected, but that failed");

            result.compare()
        };

        use std::cmp::Ordering;

        if self.moves_up
            && (ordering == Ordering::Less || self.is_end_inclusive && ordering == Ordering::Equal)
        {
            let output = UntaggedValue::Primitive(self.curr.clone()).into_value(self.tag.clone());

            let next_value = nu_data::value::compute_values(
                Operator::Plus,
                &UntaggedValue::Primitive(self.curr.clone()),
                &UntaggedValue::int(1),
            );

            self.curr = match next_value {
                Ok(result) => match result {
                    UntaggedValue::Primitive(p) => p,
                    _ => {
                        return Some(
                            UntaggedValue::Error(ShellError::unimplemented(
                                "Internal error: expected a primitive result from increment",
                            ))
                            .into_untagged_value(),
                        );
                    }
                },
                Err((left_type, right_type)) => {
                    return Some(
                        UntaggedValue::Error(ShellError::coerce_error(
                            left_type.spanned(self.tag.span),
                            right_type.spanned(self.tag.span),
                        ))
                        .into_value(self.tag.clone()),
                    );
                }
            };
            Some(output)
        } else if !self.moves_up
            && (ordering == Ordering::Greater
                || self.is_end_inclusive && ordering == Ordering::Equal)
        {
            let output = UntaggedValue::Primitive(self.curr.clone()).into_value(self.tag.clone());

            let next_value = nu_data::value::compute_values(
                Operator::Plus,
                &UntaggedValue::Primitive(self.curr.clone()),
                &UntaggedValue::int(-1),
            );

            self.curr = match next_value {
                Ok(result) => match result {
                    UntaggedValue::Primitive(p) => p,
                    _ => {
                        return Some(
                            UntaggedValue::Error(ShellError::unimplemented(
                                "Internal error: expected a primitive result from increment",
                            ))
                            .into_untagged_value(),
                        );
                    }
                },
                Err((left_type, right_type)) => {
                    return Some(
                        UntaggedValue::Error(ShellError::coerce_error(
                            left_type.spanned(self.tag.span),
                            right_type.spanned(self.tag.span),
                        ))
                        .into_value(self.tag.clone()),
                    );
                }
            };
            Some(output)
        } else {
            None
        }
    }
}
