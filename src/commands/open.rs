use crate::*;
use std::{fs::File, io::BufRead};

use std::io::BufReader;

pub struct OpenCommand;

impl PipelineElement for OpenCommand {
    fn start(&self, args: CommandArgs) -> ValueIterator {
        let fname = match &args.args[0] {
            Value::String(s) => s.clone(),
            _ => return Box::new(std::iter::empty()),
        };

        let f = File::open(&fname).unwrap();
        let buf_reader = BufReader::with_capacity(1024 * 32, f);

        Box::new(OpenIterator { buf_reader })
    }
}

struct OpenIterator {
    buf_reader: BufReader<File>,
}

impl Iterator for OpenIterator {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        let buffer = self.buf_reader.fill_buf().unwrap();

        if buffer.len() > 0 {
            let st = String::from_utf8_lossy(buffer).to_string();
            let length = buffer.len();
            self.buf_reader.consume(length);
            Some(Value::String(st))
        } else {
            None
        }
    }
}
