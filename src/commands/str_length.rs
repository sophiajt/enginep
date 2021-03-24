use crate::*;
use num_bigint::BigInt;

pub struct StrLengthCommand;

impl PipelineElement for StrLengthCommand {
    fn start(&self, args: CommandArgs) -> ValueIterator {
        Box::new(args.input.map(|x| match x {
            Value::String(s) => Value::BigInt(BigInt::from(s.len())),
            _ => Value::SmallInt(0),
        }))
    }
}
