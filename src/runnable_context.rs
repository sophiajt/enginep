use crate::{Command, Scope, ValueIterator};
use nu_errors::ShellError;
use nu_source::Tag;
use parking_lot::Mutex;
use std::sync::{atomic::AtomicBool, Arc};

pub struct RunnableContext {
    pub input: ValueIterator,
    pub ctrl_c: Arc<AtomicBool>,
    pub current_errors: Arc<Mutex<Vec<ShellError>>>,
    pub name: Tag,
}

// impl RunnableContext {
//     pub fn get_command(&self, name: &str) -> Option<Command> {
//         self.scope.get_command(name)
//     }
// }
