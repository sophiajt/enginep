use crate::lib::*;

// Sum command
pub struct SumCommand;

impl PipelineElement for SumCommand {
    fn start(&self, args: CommandArgs) -> ValueIterator {
        Box::new(SumIterator {
            input: args.input,
            done: false,
        })
    }
}

struct SumIterator {
    input: ValueIterator,
    done: bool,
}

impl Iterator for SumIterator {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let input = &mut self.input;
        let result = input.fold(Value::SmallInt(0), |a, b| a.add(&b));
        self.done = true;
        Some(result)
    }
}
