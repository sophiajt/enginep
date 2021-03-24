use crate::lib::*;

// Take command
pub struct TakeCommand;

impl PipelineElement for TakeCommand {
    fn start(&self, args: CommandArgs) -> ValueIterator {
        if let Value::SmallInt(n) = &args.args[0] {
            Box::new(args.input.take(*n as usize))
        } else {
            Box::new(args.input.take(0))
        }
    }
}
