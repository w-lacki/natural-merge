use crate::io::{Reader, Writer};
use crate::model::Record;
use std::fs::File;

pub(crate) fn sort(input_path: &str, t1_path: &str, t2_path: &str) {
    loop {
        let input = File::open(input_path).unwrap();
        let t1 = File::create(t1_path).unwrap();
        let t2 = File::create(t2_path).unwrap();

        let mut input = crate::io::BuffReader::new(input);
        let mut t1_writer = crate::io::BuffWriter::new(t1);
        let mut t2_writer = crate::io::BuffWriter::new(t2);

        distribute(&mut input, &mut t1_writer, &mut t2_writer);
        input.close();
        t1_writer.close();
        t2_writer.close();

        let mut t1 = crate::io::BuffReader::new(File::open(t1_path).unwrap());
        let mut t2 = crate::io::BuffReader::new(File::open(t2_path).unwrap());
        let mut output = crate::io::BuffWriter::new(File::create(input_path).unwrap());

        let sorted = merge(&mut output, &mut t1, &mut t2);
        output.close();
        t1.close();
        t2.close();

        if sorted {
            break;
        }
    }
}
fn merge(output: &mut impl Writer, t1reader: &mut impl Reader, t2reader: &mut impl Reader) -> bool {
    let mut sorted = true;
    let mut prev: Option<Record> = None;

    let write_and_check = &mut |current: Record| {
        if (prev.as_ref().is_some_and(|prev| &current < prev)) {
            sorted = false
        }

        output.write(current.clone());
        prev = Some(current);
    };

    let mut t1 = t1reader.read();
    let mut t2 = t2reader.read();

    while t1.is_some() && t2.is_some() {
        if t1.as_ref().unwrap() <= t2.as_ref().unwrap() {
            write_and_check(t1.unwrap());
            t1 = t1reader.read();
        } else {
            write_and_check(t2.unwrap());
            t2 = t2reader.read();
        }
    }

    while t1.is_some() {
        write_and_check(t1.unwrap());
        t1 = t1reader.read();
    }

    while t2.is_some() {
        write_and_check(t2.unwrap());
        t2 = t2reader.read();
    }

    sorted
}

fn distribute(input: &mut impl Reader, t1writer: &mut impl Writer, t2writer: &mut impl Writer) {
    let mut prev: Option<Record> = None;
    let mut to_t1 = true;

    while let Some(current) = input.read() {
        if (prev.as_ref().is_some_and(|prev| prev > &current)) {
            to_t1 = !to_t1;
        }

        if to_t1 {
            t1writer.write(current.clone());
        } else {
            t2writer.write(current.clone());
        }

        prev = Some(current);
    }
}
