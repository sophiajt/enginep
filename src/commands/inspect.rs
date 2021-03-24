use crate::lib::*;

// Take command
pub struct InspectCommand;

impl PipelineElement for InspectCommand {
    fn start(&self, args: CommandArgs) -> ValueIterator {
        Box::new(args.input.inspect(|x| println!("{:?}", x)))
    }
}
