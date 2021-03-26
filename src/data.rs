use std::sync::{atomic::AtomicBool, Arc};

use nu_protocol::Value;

pub struct InterruptibleIterator {
    inner: ValueIterator,
    interrupt_signal: Arc<AtomicBool>,
}

impl InterruptibleIterator {
    pub fn new<S>(inner: S, interrupt_signal: Arc<AtomicBool>) -> InterruptibleIterator
    where
        S: Iterator<Item = Value> + Send + Sync + 'static,
    {
        InterruptibleIterator {
            inner: Box::new(inner),
            interrupt_signal,
        }
    }
}

impl Iterator for InterruptibleIterator {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

pub trait Interruptible {
    fn interruptible(self, ctrl_c: Arc<AtomicBool>) -> InterruptibleIterator;
}

impl<S> Interruptible for S
where
    S: Iterator<Item = Value> + Send + Sync + 'static,
{
    fn interruptible(self, ctrl_c: Arc<AtomicBool>) -> InterruptibleIterator {
        InterruptibleIterator::new(self, ctrl_c)
    }
}

// pub struct CommandArgs {
//     pub input: ValueIterator,
//     pub args: Vec<Value>,
//     pub state: State,
// }

// pub struct State;

// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
// pub enum Value {
//     BigInt(BigInt),
//     SmallInt(i64),
//     String(String),
//     Bool(bool),
// }

// impl Value {
//     pub fn add(&self, other: &Value) -> Value {
//         match (self, other) {
//             (Value::BigInt(a), Value::BigInt(b)) => Value::BigInt(a + b),
//             (Value::BigInt(a), Value::SmallInt(b)) => Value::BigInt(a + BigInt::from(*b)),
//             (Value::SmallInt(a), Value::BigInt(b)) => Value::BigInt(BigInt::from(*a) + b),
//             (Value::SmallInt(a), Value::SmallInt(b)) => match a.checked_add(*b) {
//                 Some(c) => Value::SmallInt(c),
//                 None => Value::BigInt(BigInt::from(*a) + BigInt::from(*b)),
//             },
//             _ => Value::Bool(false),
//         }
//     }

//     pub fn lt(&self, other: &Value) -> bool {
//         match (self, other) {
//             (Value::BigInt(a), Value::BigInt(b)) => a < b,
//             (Value::BigInt(a), Value::SmallInt(b)) => *a < BigInt::from(*b),
//             (Value::SmallInt(a), Value::BigInt(b)) => BigInt::from(*a) < *b,
//             (Value::SmallInt(a), Value::SmallInt(b)) => a < b,
//             _ => false,
//         }
//     }
// }

// trait NewTrait: Iterator<Item = Value> + Clone {}

pub type ValueIterator = Box<dyn Iterator<Item = Value> + Send + Sync>;

pub fn empty_value_iterator() -> ValueIterator {
    Box::new(std::iter::empty())
}

// pub trait PipelineElement {
//     fn start(&self, args: CommandArgs) -> ValueIterator;
// }
