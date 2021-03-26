use crate::{empty_value_iterator, CommandArgs, Example, Scope, ValueIterator, WholeStreamCommand};

use nu_errors::ShellError;
use nu_protocol::{hir::CapturedBlock, Signature, SyntaxShape, Value};
use nu_source::Tagged;

pub struct Def;

impl WholeStreamCommand for Def {
    fn name(&self) -> &str {
        "def"
    }

    fn signature(&self) -> Signature {
        Signature::build("def")
            .required("name", SyntaxShape::String, "the name of the command")
            .required(
                "params",
                SyntaxShape::Table,
                "the parameters of the command",
            )
            .required("block", SyntaxShape::Block, "the body of the command")
    }

    fn usage(&self) -> &str {
        "Create a command and set it to a definition."
    }

    fn run(&self, _args: CommandArgs, _: &Scope) -> Result<ValueIterator, ShellError> {
        // Currently, we don't do anything here because we should have already
        // installed the definition as we entered the scope
        // We just create a command so that we can get proper coloring
        Ok(empty_value_iterator())
    }

    fn examples(&self) -> Vec<Example> {
        vec![]
    }
}