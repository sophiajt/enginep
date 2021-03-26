use crate::{
    commands::autoview::options::{ConfigExtensions, NuConfig as AutoViewConfiguration},
    empty_value_iterator, CommandArgs, Example, Interruptible, InterruptibleIterator,
    RawCommandArgs, RunnableContext, Scope, ValueIterator,
};
use crate::{UnevaluatedCallInfo, WholeStreamCommand};
use nu_data::primitive::get_color_config;
use nu_data::value::format_leaf;
use nu_errors::ShellError;
use nu_protocol::hir::{self, Expression, ExternalRedirection, Literal, SpannedExpression};
use nu_protocol::{Primitive, Signature, UntaggedValue, Value};
use nu_source::{PrettyDebug, Tag};
use nu_table::TextStyle;
use parking_lot::Mutex;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

pub struct Command;

impl WholeStreamCommand for Command {
    fn name(&self) -> &str {
        "autoview"
    }

    fn signature(&self) -> Signature {
        Signature::build("autoview")
    }

    fn usage(&self) -> &str {
        "View the contents of the pipeline as a table or list."
    }

    fn run(&self, args: CommandArgs, scope: &Scope) -> Result<ValueIterator, ShellError> {
        autoview(
            RunnableContext {
                input: args.input,
                ctrl_c: args.ctrl_c,
                current_errors: args.current_errors,
                name: args.call_info.name_tag,
            },
            scope,
        )
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Automatically view the results",
                example: "ls | autoview",
                result: None,
            },
            Example {
                description: "Autoview is also implied. The above can be written as",
                example: "ls",
                result: None,
            },
        ]
    }
}

pub struct RunnableContextWithoutInput {
    pub current_errors: Arc<Mutex<Vec<ShellError>>>,
    pub ctrl_c: Arc<AtomicBool>,
    pub name: Tag,
}

impl RunnableContextWithoutInput {
    pub fn convert(context: RunnableContext) -> (ValueIterator, RunnableContextWithoutInput) {
        let new_context = RunnableContextWithoutInput {
            ctrl_c: context.ctrl_c,
            current_errors: context.current_errors,
            name: context.name,
        };
        (context.input, new_context)
    }
}

pub fn autoview(context: RunnableContext, scope: &Scope) -> Result<ValueIterator, ShellError> {
    let configuration = AutoViewConfiguration::new();

    let table = scope.get_command("table");

    let pivot_mode = configuration.pivot_mode();

    let (mut input_stream, context) = RunnableContextWithoutInput::convert(context);
    //FIXME!!!!!!!!!!!!
    //let term_width = context.host.lock().width();

    let term_width = 80;
    let color_hm = get_color_config();

    if let Some(x) = input_stream.next() {
        match input_stream.next() {
            Some(y) => {
                let ctrl_c = context.ctrl_c;
                let xy = vec![x, y];
                let xy_iter: InterruptibleIterator =
                    xy.into_iter().chain(input_stream).interruptible(ctrl_c);

                for x in xy_iter {
                    println!("{:?}", x);
                }

                //let result: Vec<_> = xy_iter.collect();
                //println!("{:?}", result);
                // if let Some(table) = table {
                //     let command_args =
                //         create_default_command_args(&context).with_input(Box::new(xy_iter));
                //     let result = table.run(command_args, scope)?;
                //     result.collect::<Vec<_>>();
                // }
            }
            _ => {
                match x {
                    Value {
                        value: UntaggedValue::Primitive(Primitive::String(ref s)),
                        tag: Tag { anchor, span },
                    } if anchor.is_some() => {
                        print!("{}", s);
                    }
                    Value {
                        value: UntaggedValue::Primitive(Primitive::String(s)),
                        ..
                    } => {
                        print!("{}", s);
                    }
                    Value {
                        value: UntaggedValue::Primitive(Primitive::FilePath(s)),
                        ..
                    } => {
                        print!("{}", s.display());
                    }
                    Value {
                        value: UntaggedValue::Primitive(Primitive::Int(n)),
                        ..
                    } => {
                        print!("{}", n);
                    }
                    Value {
                        value: UntaggedValue::Primitive(Primitive::Decimal(n)),
                        ..
                    } => {
                        // TODO: normalize decimal to remove trailing zeros.
                        // normalization will be available in next release of bigdecimal crate
                        let mut output = n.to_string();
                        if output.contains('.') {
                            output = output.trim_end_matches('0').to_owned();
                        }
                        if output.ends_with('.') {
                            output.push('0');
                        }
                        print!("{}", output);
                    }
                    Value {
                        value: UntaggedValue::Primitive(Primitive::Boolean(b)),
                        ..
                    } => {
                        print!("{}", b);
                    }
                    Value {
                        value: UntaggedValue::Primitive(Primitive::Duration(_)),
                        ..
                    } => {
                        let output = format_leaf(&x).plain_string(100_000);
                        print!("{}", output);
                    }
                    Value {
                        value: UntaggedValue::Primitive(Primitive::Filesize(_)),
                        ..
                    } => {
                        let output = format_leaf(&x).plain_string(100_000);
                        print!("{}", output);
                    }
                    Value {
                        value: UntaggedValue::Primitive(Primitive::Date(d)),
                        ..
                    } => {
                        print!("{}", d);
                    }
                    Value {
                        value: UntaggedValue::Primitive(Primitive::Range(_)),
                        ..
                    } => {
                        let output = format_leaf(&x).plain_string(100_000);
                        print!("{}", output);
                    }

                    Value {
                        value: UntaggedValue::Primitive(Primitive::Binary(ref b)),
                        ..
                    } => {
                        use pretty_hex::*;
                        print!("{:?}", b.hex_dump());
                    }

                    Value {
                        value: UntaggedValue::Error(e),
                        ..
                    } => {
                        return Err(e);
                    }

                    Value {
                        value: UntaggedValue::Row(row),
                        ..
                    } if pivot_mode.is_always()
                        || (pivot_mode.is_auto()
                            && (row
                                .entries
                                .iter()
                                .map(|(_, v)| v.convert_to_string())
                                .collect::<Vec<_>>()
                                .iter()
                                .fold(0usize, |acc, len| acc + len.len())
                                + row.entries.iter().count() * 2)
                                > term_width) =>
                    {
                        let mut entries = vec![];
                        for (key, value) in row.entries.iter() {
                            entries.push(vec![
                                nu_table::StyledString::new(
                                    key.to_string(),
                                    TextStyle::new()
                                        .alignment(nu_table::Alignment::Left)
                                        .fg(nu_ansi_term::Color::Green)
                                        .bold(Some(true)),
                                ),
                                nu_table::StyledString::new(
                                    format_leaf(value).plain_string(100_000),
                                    nu_table::TextStyle::basic_left(),
                                ),
                            ]);
                        }

                        let table =
                            nu_table::Table::new(vec![], entries, nu_table::Theme::compact());

                        println!("{}", nu_table::draw_table(&table, term_width, &color_hm));
                    }
                    Value {
                        value: UntaggedValue::Primitive(Primitive::Nothing),
                        ..
                    } => {
                        // Do nothing
                    }
                    Value {
                        value: ref item, ..
                    } => {
                        if let Some(table) = table {
                            let mut stream = Vec::new();
                            stream.push(x);
                            let command_args = create_default_command_args(&context)
                                .with_input(Box::new(stream.into_iter()));
                            let result = table.run(command_args, scope)?;
                            result.collect::<Vec<_>>();
                        } else {
                            print!("{:?}", item);
                        }
                    }
                }
            }
        }
    }

    Ok(empty_value_iterator())
}

fn create_default_command_args(context: &RunnableContextWithoutInput) -> RawCommandArgs {
    let span = context.name.span;
    RawCommandArgs {
        ctrl_c: context.ctrl_c.clone(),
        current_errors: context.current_errors.clone(),
        call_info: UnevaluatedCallInfo {
            args: hir::Call {
                head: Box::new(SpannedExpression::new(
                    Expression::Literal(Literal::String(String::new())),
                    span,
                )),
                positional: None,
                named: None,
                span,
                external_redirection: ExternalRedirection::Stdout,
            },
            name_tag: context.name.clone(),
        },
        scope: Scope::new(),
    }
}
