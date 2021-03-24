use crate::*;

// Append command
pub struct AppendCommand;

impl PipelineElement for AppendCommand {
    fn start(&self, args: CommandArgs) -> ValueIterator {
        let step = std::iter::once(args.args[0].clone());
        Box::new(args.input.chain(step))
    }
}
