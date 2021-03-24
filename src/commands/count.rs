use crate::*;

// Counts from 0 to the highest it can
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
