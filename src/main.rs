mod lib;
use lib::*;

mod commands;
use commands::*;

mod par_iter_adapter;

use std::collections::HashMap;

struct CallInfo {
    name: String,
    args: Vec<Value>,
}

fn build_pipeline(
    pipeline: Vec<CallInfo>,
    lookup: HashMap<String, Box<dyn PipelineElement>>,
) -> ValueIterator {
    let mut prev: ValueIterator = Box::new(std::iter::empty());

    for elem in pipeline.into_iter() {
        if let Some(command) = lookup.get(&elem.name) {
            prev = command.start(CommandArgs {
                input: prev,
                args: elem.args,
                state: State,
            })
        }
    }

    prev
}

fn main() {
    let count = match std::env::args().skip(1).next() {
        Some(x) => x.parse::<i64>().unwrap(),
        None => 1000000,
    };

    let mut map = HashMap::new();
    map.insert("count".into(), command(CountCommand));
    map.insert("take".into(), command(TakeCommand));
    map.insert("sum".into(), command(SumCommand));
    map.insert("append".into(), command(AppendCommand));
    map.insert("prepend".into(), command(PrependCommand));
    map.insert("where".into(), command(WhereCommand));
    map.insert("length".into(), command(LengthCommand));
    map.insert("par-each".into(), command(ParEachCommand));

    let pipeline = vec![
        CallInfo {
            name: "count".into(),
            args: vec![],
        },
        CallInfo {
            name: "take".into(),
            args: vec![Value::SmallInt(count)],
        },
        CallInfo {
            name: "append".into(),
            args: vec![Value::SmallInt(555)],
        },
        CallInfo {
            name: "prepend".into(),
            args: vec![Value::SmallInt(777)],
        },
        // CallInfo {
        //     name: "par-each".into(),
        //     args: vec![Value::SmallInt(7)],
        // },
        CallInfo {
            name: "where".into(),
            args: vec![Value::SmallInt(7)],
        },
        CallInfo {
            name: "par-each".into(),
            args: vec![Value::SmallInt(10), Value::SmallInt(4)],
        },
        // CallInfo {
        //     name: "length".into(),
        //     args: vec![],
        // }
        // CallInfo {
        //     name: "sum".into(),
        //     args: vec![],
        // },
    ];

    let pipeline = build_pipeline(pipeline, map);

    {
        // use rayon::prelude::*;

        // let result: Vec<_> = pipeline
        //     .par_bridge()
        //     .map(|x| match x {
        //         Value::Number(x) => Value::Number(x + 100),
        //     })
        //     .collect();

        // println!("result: {:?}", result);

        let result: Vec<_> = pipeline.collect();
        println!("{:?}", result);
    }
}