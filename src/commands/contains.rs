use crate::*;

pub struct ContainsCommand;

impl PipelineElement for ContainsCommand {
    fn start(&self, args: CommandArgs) -> ValueIterator {
        let item = args.args[0].clone();

        Box::new(ContainsIterator {
            input: args.input,
            item,
            done: false,
        })
    }
}

pub struct ContainsIterator {
    input: ValueIterator,
    item: Value,
    done: bool,
}

impl Iterator for ContainsIterator {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        self.done = true;
        let item = self.item.clone();
        self.input.find(|x| x == &item)
    }
}
