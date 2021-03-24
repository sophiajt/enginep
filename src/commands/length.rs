use crate::lib::*;
use num_bigint::BigInt;

// Length command
pub struct LengthCommand;

impl PipelineElement for LengthCommand {
    fn start(&self, args: CommandArgs) -> ValueIterator {
        Box::new(LengthIterator {
            input: args.input,
            done: false,
        })
    }
}

struct LengthIterator {
    input: ValueIterator,
    done: bool,
}

impl Iterator for LengthIterator {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let input = &mut self.input;
        let output = Some(Value::BigInt(BigInt::from(input.count())));
        self.done = true;

        output
    }
}
