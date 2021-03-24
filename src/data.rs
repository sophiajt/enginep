use nu_protocol::Value;

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
