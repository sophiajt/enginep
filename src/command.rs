use nu_errors::ShellError;
use nu_protocol::{hir::Block, CallInfo, Value};
use nu_protocol::{EvaluatedArgs, Signature};
use nu_source::Tag;
use std::sync::Arc;
use std::{ops::Deref, sync::atomic::AtomicBool};

use crate::{
    empty_value_iterator, evaluate::EvaluationContext, UnevaluatedCallInfo, ValueIterator,
};

use crate::evaluate::Scope;

pub fn command(command: impl WholeStreamCommand + 'static) -> Command {
    Arc::new(command)
}

pub struct Example {
    pub example: &'static str,
    pub description: &'static str,
    pub result: Option<Vec<Value>>,
}

pub struct RawCommandArgs {
    pub ctrl_c: Arc<AtomicBool>,
    pub current_errors: Arc<parking_lot::Mutex<Vec<ShellError>>>,
    pub scope: Scope,
    pub call_info: UnevaluatedCallInfo,
}

impl RawCommandArgs {
    pub fn with_input(self, input: ValueIterator) -> CommandArgs {
        CommandArgs {
            ctrl_c: self.ctrl_c,
            current_errors: self.current_errors,
            call_info: self.call_info,
            input,
        }
    }
}

pub struct CommandArgs {
    pub ctrl_c: Arc<AtomicBool>,
    pub current_errors: Arc<parking_lot::Mutex<Vec<ShellError>>>,
    pub call_info: UnevaluatedCallInfo,
    pub input: ValueIterator,
}

impl CommandArgs {
    pub fn evaluate_once(self) -> Result<EvaluatedWholeStreamCommandArgs, ShellError> {
        let ctx = EvaluationContext::from_args(&self);
        let ctrl_c = self.ctrl_c.clone();
        let input = self.input;
        let call_info = self.call_info.evaluate(&ctx)?;

        Ok(EvaluatedWholeStreamCommandArgs::new(
            ctrl_c, call_info, input,
        ))
    }
}

pub struct EvaluatedWholeStreamCommandArgs {
    pub args: EvaluatedCommandArgs,
    pub input: ValueIterator,
}

impl Deref for EvaluatedWholeStreamCommandArgs {
    type Target = EvaluatedCommandArgs;
    fn deref(&self) -> &Self::Target {
        &self.args
    }
}

impl EvaluatedWholeStreamCommandArgs {
    pub fn new(
        ctrl_c: Arc<AtomicBool>,
        call_info: CallInfo,
        input: ValueIterator,
    ) -> EvaluatedWholeStreamCommandArgs {
        EvaluatedWholeStreamCommandArgs {
            args: EvaluatedCommandArgs { ctrl_c, call_info },
            input: input.into(),
        }
    }

    pub fn name_tag(&self) -> Tag {
        self.args.call_info.name_tag.clone()
    }

    pub fn parts(self) -> (ValueIterator, EvaluatedArgs) {
        let EvaluatedWholeStreamCommandArgs { args, input } = self;

        (input, args.call_info.args)
    }

    pub fn split(self) -> (ValueIterator, EvaluatedCommandArgs) {
        let EvaluatedWholeStreamCommandArgs { args, input } = self;

        (input, args)
    }
}

pub struct EvaluatedCommandArgs {
    pub ctrl_c: Arc<AtomicBool>,
    pub call_info: CallInfo,
}

impl EvaluatedCommandArgs {
    pub fn nth(&self, pos: usize) -> Option<&Value> {
        self.call_info.args.nth(pos)
    }

    /// Get the nth positional argument, error if not possible
    pub fn expect_nth(&self, pos: usize) -> Result<&Value, ShellError> {
        self.call_info
            .args
            .nth(pos)
            .ok_or_else(|| ShellError::unimplemented("Better error: expect_nth"))
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.call_info.args.get(name)
    }

    pub fn has(&self, name: &str) -> bool {
        self.call_info.args.has(name)
    }
}

pub trait WholeStreamCommand: Send + Sync {
    fn name(&self) -> &str;

    fn signature(&self) -> Signature {
        Signature::new(self.name()).desc(self.usage()).filter()
    }

    fn usage(&self) -> &str;

    fn extra_usage(&self) -> &str {
        ""
    }

    fn run(&self, args: CommandArgs, scope: &Scope) -> Result<ValueIterator, ShellError>;

    fn is_binary(&self) -> bool {
        false
    }

    // Commands that are not meant to be run by users
    fn is_internal(&self) -> bool {
        false
    }

    fn examples(&self) -> Vec<Example> {
        Vec::new()
    }
}

impl WholeStreamCommand for Block {
    fn name(&self) -> &str {
        &self.params.name
    }

    fn signature(&self) -> Signature {
        self.params.clone()
    }

    fn usage(&self) -> &str {
        &self.params.usage
    }

    fn run(&self, args: CommandArgs, scope: &Scope) -> Result<ValueIterator, ShellError> {
        /*
        let call_info = args.call_info.clone();

        let mut block = self.clone();
        block.set_redirect(call_info.args.external_redirection);

        let ctx = EvaluationContext::from_args(&args);
        let evaluated = call_info.evaluate(&ctx).await?;

        let input = args.input;
        ctx.scope.enter_scope();
        if let Some(args) = evaluated.args.positional {
            let mut args_iter = args.into_iter().peekable();
            let mut params_iter = self.params.positional.iter();
            loop {
                match (args_iter.peek(), params_iter.next()) {
                    (Some(_), Some(param)) => {
                        let name = param.0.name();
                        // we just checked the peek above, so this should be infallible
                        if let Some(arg) = args_iter.next() {
                            if name.starts_with('$') {
                                ctx.scope.add_var(name.to_string(), arg);
                            } else {
                                ctx.scope.add_var(format!("${}", name), arg);
                            }
                        }
                    }
                    (Some(arg), None) => {
                        if block.params.rest_positional.is_none() {
                            ctx.scope.exit_scope();
                            return Err(ShellError::labeled_error(
                                "Unexpected argument to command",
                                "unexpected argument",
                                arg.tag.span,
                            ));
                        } else {
                            break;
                        }
                    }
                    _ => break,
                }
            }
            if block.params.rest_positional.is_some() {
                let elements: Vec<_> = args_iter.collect();
                let start = if let Some(first) = elements.first() {
                    first.tag.span.start()
                } else {
                    0
                };
                let end = if let Some(last) = elements.last() {
                    last.tag.span.end()
                } else {
                    0
                };

                ctx.scope.add_var(
                    "$rest",
                    UntaggedValue::Table(elements).into_value(Span::new(start, end)),
                );
            }
        }
        if let Some(args) = evaluated.args.named {
            for named in &block.params.named {
                let name = named.0;
                if let Some(value) = args.get(name) {
                    if name.starts_with('$') {
                        ctx.scope.add_var(name, value.clone());
                    } else {
                        ctx.scope.add_var(format!("${}", name), value.clone());
                    }
                } else if name.starts_with('$') {
                    ctx.scope
                        .add_var(name, UntaggedValue::nothing().into_untagged_value());
                } else {
                    ctx.scope.add_var(
                        format!("${}", name),
                        UntaggedValue::nothing().into_untagged_value(),
                    );
                }
            }
        } else {
            for named in &block.params.named {
                let name = named.0;
                if name.starts_with('$') {
                    ctx.scope
                        .add_var(name, UntaggedValue::nothing().into_untagged_value());
                } else {
                    ctx.scope.add_var(
                        format!("${}", name),
                        UntaggedValue::nothing().into_untagged_value(),
                    );
                }
            }
        }
        let result = run_block(&block, &ctx, input).await;
        ctx.scope.exit_scope();
        result.map(|x| x.to_output_stream())
        */

        Ok(empty_value_iterator())
    }

    fn is_binary(&self) -> bool {
        false
    }

    fn is_internal(&self) -> bool {
        false
    }

    fn examples(&self) -> Vec<Example> {
        vec![]
    }
}

pub type Command = Arc<dyn WholeStreamCommand>;
