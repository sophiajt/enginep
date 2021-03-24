use crate::lib::*;

// Take command
pub struct TakeCommand;

impl PipelineElement for TakeCommand {
    fn start(&self, args: CommandArgs) -> ValueIterator {
        if let Value::SmallInt(n) = &args.args[0] {
            Box::new(TakeIterator {
                input: args.input,
                remaining: *n,
            })
        } else {
            Box::new(TakeIterator {
                input: args.input,
                remaining: 0,
            })
        }
    }
}

struct TakeIterator {
    input: ValueIterator,
    remaining: i64,
}

impl Iterator for TakeIterator {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining > 0 {
            let result = self.input.next();
            self.remaining -= 1;
            result
        } else {
            None
        }
    }
}
