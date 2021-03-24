use crate::*;
// Where command
pub struct WhereCommand;

impl PipelineElement for WhereCommand {
    fn start(&self, args: CommandArgs) -> ValueIterator {
        Box::new(WhereIterator {
            input: args.input,
            pred: args.args[0].clone(),
        })
    }
}

struct WhereIterator {
    input: ValueIterator,
    pred: Value,
}

impl Iterator for WhereIterator {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next) = self.input.next() {
            if self.pred.lt(&next) {
                return Some(next);
            }
        }

        None
    }
}
