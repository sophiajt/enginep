use crate::lib::*;

// Prepend command
pub struct PrependCommand;

impl PipelineElement for PrependCommand {
    fn start(&self, args: CommandArgs) -> ValueIterator {
        let step = std::iter::once(args.args[0].clone());
        Box::new(PrependIterator {
            input: Box::new(step.chain(args.input)),
        })
    }
}

struct PrependIterator {
    input: ValueIterator,
}

impl Iterator for PrependIterator {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.input.next()
    }
}
