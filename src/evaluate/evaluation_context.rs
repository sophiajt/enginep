use crate::evaluate::Scope;
use crate::CommandArgs;
use crate::{Command, UnevaluatedCallInfo, ValueIterator};

use nu_errors::ShellError;
use nu_protocol::hir;
use nu_source::Tag;
use parking_lot::Mutex;
use std::sync::Arc;
use std::{collections::HashMap, sync::atomic::AtomicBool};

#[derive(Clone)]
pub struct EvaluationContext {
    pub current_errors: Arc<Mutex<Vec<ShellError>>>,
    pub ctrl_c: Arc<AtomicBool>,
    pub user_recently_used_autoenv_untrust: Arc<AtomicBool>,

    /// Windows-specific: keep track of previous cwd on each drive
    pub windows_drives_previous_cwd: Arc<Mutex<std::collections::HashMap<String, String>>>,
}

impl EvaluationContext {
    pub fn basic() -> EvaluationContext {
        EvaluationContext {
            ctrl_c: Arc::new(AtomicBool::from(false)),
            current_errors: Arc::new(parking_lot::Mutex::new(vec![])),
            user_recently_used_autoenv_untrust: Arc::new(AtomicBool::from(false)),
            windows_drives_previous_cwd: Arc::new(parking_lot::Mutex::new(HashMap::new())),
        }
    }
    pub fn from_args(args: &CommandArgs) -> EvaluationContext {
        EvaluationContext {
            current_errors: args.current_errors.clone(),
            ctrl_c: args.ctrl_c.clone(),
            user_recently_used_autoenv_untrust: Arc::new(AtomicBool::new(false)),
            windows_drives_previous_cwd: Arc::new(Mutex::new(std::collections::HashMap::new())),
        }
    }

    pub fn error(&self, error: ShellError) {
        self.with_errors(|errors| errors.push(error))
    }

    pub fn clear_errors(&self) {
        self.current_errors.lock().clear()
    }

    pub fn get_errors(&self) -> Vec<ShellError> {
        self.current_errors.lock().clone()
    }

    pub fn configure<T>(
        &mut self,
        config: &dyn nu_data::config::Conf,
        block: impl FnOnce(&dyn nu_data::config::Conf, &mut Self) -> T,
    ) {
        block(config, &mut *self);
    }

    pub fn with_errors<T>(&self, block: impl FnOnce(&mut Vec<ShellError>) -> T) -> T {
        let mut errors = self.current_errors.lock();

        block(&mut *errors)
    }

    pub fn run_command(
        &self,
        command: Command,
        name_tag: Tag,
        args: hir::Call,
        input: ValueIterator,
        scope: &Scope,
    ) -> Result<ValueIterator, ShellError> {
        let command_args = self.command_args(args, input, name_tag);
        command.run(command_args, scope)
    }

    fn call_info(&self, args: hir::Call, name_tag: Tag) -> UnevaluatedCallInfo {
        UnevaluatedCallInfo { args, name_tag }
    }

    fn command_args(&self, args: hir::Call, input: ValueIterator, name_tag: Tag) -> CommandArgs {
        CommandArgs {
            ctrl_c: self.ctrl_c.clone(),
            current_errors: self.current_errors.clone(),
            call_info: self.call_info(args, name_tag),
            input,
        }
    }
}
