use crate::lib::*;
use crate::par_iter_adapter::PartitionedParallelIterator;

use num_bigint::BigInt;

// Count command

pub struct CountCommand;

impl PipelineElement for CountCommand {
    fn start(&self, _: CommandArgs) -> ValueIterator {
        Box::new(CountIterator {
            current: Value::SmallInt(0),
        })
    }
}

struct CountIterator {
    current: Value,
}

impl Iterator for CountIterator {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        let output = self.current.clone();
        self.current = output.add(&Value::SmallInt(1));
        Some(output)
    }
}

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
        // if let Some(input) = &mut self.input {
        //     let result = input.
        //     self.input = None;
        //     Some(result)
        // } else {
        //     None
        // }
        Some(result)
    }
}

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

// Par each command

pub struct ParEachCommand;

impl PipelineElement for ParEachCommand {
    fn start(&self, args: CommandArgs) -> ValueIterator {
        match (args.args.get(0), args.args.get(1)) {
            (Some(Value::SmallInt(per_worker)), Some(Value::SmallInt(num_workers))) => {
                Box::new(PartitionedParallelIterator::new(
                    Box::new(|x| Some(x)),
                    *per_worker as usize,
                    *num_workers as usize,
                    args.input,
                ))
            }
            _ => Box::new(std::iter::empty()),
        }
    }
}
