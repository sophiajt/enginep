mod commands;
pub use commands::*;

mod par_iter_adapter;

use num_bigint::BigInt;

pub struct CommandArgs {
    pub input: ValueIterator,
    pub args: Vec<Value>,
    pub state: State,
}

pub struct State;

#[derive(Debug, Clone)]
pub enum Value {
    BigInt(BigInt),
    SmallInt(i64),
    Bool(bool),
}

impl Value {
    pub fn add(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::BigInt(a), Value::BigInt(b)) => Value::BigInt(a + b),
            (Value::BigInt(a), Value::SmallInt(b)) => Value::BigInt(a + BigInt::from(*b)),
            (Value::SmallInt(a), Value::BigInt(b)) => Value::BigInt(BigInt::from(*a) + b),
            (Value::SmallInt(a), Value::SmallInt(b)) => match a.checked_add(*b) {
                Some(c) => Value::SmallInt(c),
                None => Value::BigInt(BigInt::from(*a) + BigInt::from(*b)),
            },
            _ => Value::Bool(false),
        }
    }

    pub fn lt(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::BigInt(a), Value::BigInt(b)) => a < b,
            (Value::BigInt(a), Value::SmallInt(b)) => *a < BigInt::from(*b),
            (Value::SmallInt(a), Value::BigInt(b)) => BigInt::from(*a) < *b,
            (Value::SmallInt(a), Value::SmallInt(b)) => a < b,
            _ => false,
        }
    }
}

trait NewTrait: Iterator<Item = Value> + Clone {}

pub type ValueIterator = Box<dyn Iterator<Item = Value> + Send + Sync>;

pub trait PipelineElement {
    fn start(&self, args: CommandArgs) -> ValueIterator;
}

pub fn command(x: impl PipelineElement + 'static + Send) -> Box<dyn PipelineElement> {
    Box::new(x)
}
