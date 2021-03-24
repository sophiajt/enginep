use crate::*;

use crate::par_iter_adapter::PartitionedParallelIterator;

// Par-each command
pub struct ParEachCommand;

impl PipelineElement for ParEachCommand {
    fn start(&self, args: CommandArgs) -> ValueIterator {
        match (args.args.get(0), args.args.get(1)) {
            (Some(Value::SmallInt(per_worker)), Some(Value::SmallInt(num_workers))) => {
                Box::new(PartitionedParallelIterator::new(
                    Box::new(|x| match x {
                        Value::SmallInt(x) => Some(Value::SmallInt(x + 100)),
                        y => Some(y),
                    }),
                    *per_worker as usize,
                    *num_workers as usize,
                    args.input,
                ))
            }
            _ => Box::new(std::iter::empty()),
        }
    }
}
