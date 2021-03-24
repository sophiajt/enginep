use std::collections::VecDeque;

use crate::lib::Value;
use crate::lib::ValueIterator;
use rayon::prelude::*;

pub struct PartitionedParallelIterator {
    filter_map_fn: Box<dyn Fn(Value) -> Option<Value> + Send + Sync + 'static>,
    number_per_worker: usize,
    number_of_workers: usize,
    input: ValueIterator,

    completed_batch: VecDeque<Value>,
}

impl PartitionedParallelIterator {
    pub fn new(
        filter_map_fn: Box<dyn Fn(Value) -> Option<Value> + Send + Sync + 'static>,
        number_per_worker: usize,
        number_of_workers: usize,
        input: ValueIterator,
    ) -> Self {
        Self {
            filter_map_fn,
            number_per_worker,
            number_of_workers,
            input,
            completed_batch: VecDeque::new(),
        }
    }
}

impl Iterator for PartitionedParallelIterator {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        // FIXME: do we want to pop?
        if self.completed_batch.is_empty() {
            // There are no completed items to send, so let's work on the next batch

            let mut dataset = vec![];
            'outer: for _ in 0..self.number_of_workers {
                let mut per_worker_data = vec![];
                for _ in 0..self.number_per_worker {
                    if let Some(x) = self.input.next() {
                        per_worker_data.push(x);
                    } else {
                        if !per_worker_data.is_empty() {
                            dataset.push(per_worker_data);
                            break 'outer;
                        }
                    }
                }
                dataset.push(per_worker_data);
            }

            let results: VecDeque<Value> = dataset
                .into_par_iter()
                .flat_map_iter(|per_worker_dataset| {
                    let output: Vec<_> = per_worker_dataset
                        .into_iter()
                        .filter_map(|x| (self.filter_map_fn)(x))
                        .collect();

                    output
                })
                .collect();

            self.completed_batch = results;
        }

        self.completed_batch.pop_front()
    }
}
