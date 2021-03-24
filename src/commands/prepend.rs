use crate::*;

pub struct PrependCommand;

impl PipelineElement for PrependCommand {
    fn start(&self, args: CommandArgs) -> ValueIterator {
        let step = std::iter::once(args.args[0].clone());
        Box::new(step.chain(args.input))
    }
}
