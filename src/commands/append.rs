use crate::lib::*;

// Append command
pub struct AppendCommand;

impl PipelineElement for AppendCommand {
    fn start(&self, args: CommandArgs) -> ValueIterator {
        let step = std::iter::once(args.args[0].clone());
        Box::new(AppendIterator {
            input: Box::new(args.input.chain(step)),
        })
    }
}

struct AppendIterator {
    input: ValueIterator,
}

impl Iterator for AppendIterator {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.input.next()
    }
}
