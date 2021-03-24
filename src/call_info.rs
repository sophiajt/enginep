use nu_errors::ShellError;
use nu_protocol::{hir, CallInfo};
use nu_source::Tag;

use crate::evaluate::{evaluate_args, EvaluationContext};

#[derive(Clone)]
pub struct UnevaluatedCallInfo {
    pub args: hir::Call,
    pub name_tag: Tag,
}

impl UnevaluatedCallInfo {
    pub fn evaluate(self, ctx: &EvaluationContext) -> Result<CallInfo, ShellError> {
        let args = evaluate_args(&self.args, ctx)?;

        Ok(CallInfo {
            args,
            name_tag: self.name_tag,
        })
    }

    pub fn switch_present(&self, switch: &str) -> bool {
        self.args.switch_preset(switch)
    }
}
