use crate::io::{BuffReader, BuffWriter};
use crate::model::{Record, count_primes};
use std::fs::File;

pub fn sort(input_path: &str, t1_path: &str, t2_path: &str, print_each_phase: bool) {
    let mut reads = 0;
    let mut writes = 0;
    let mut phases = 0;

    println!("Before sorting:");
    print_all(input_path);

    loop {
        phases += 1;

        if (print_each_phase) {
            println!("Phase {phases}: ");
            print_all(input_path);
        }

        let input = File::open(input_path).unwrap();
        let t1 = File::create(t1_path).unwrap();
        let t2 = File::create(t2_path).unwrap();

        let mut input = BuffReader::new(input);
        let mut t1_writer = BuffWriter::new(t1);
        let mut t2_writer = BuffWriter::new(t2);

        distribute(&mut input, &mut t1_writer, &mut t2_writer);

        input.close();
        t1_writer.close();
        t2_writer.close();

        reads += input.reads;
        writes += t1_writer.writes + t2_writer.writes;

        let mut t1 = BuffReader::new(File::open(t1_path).unwrap());
        let mut t2 = BuffReader::new(File::open(t2_path).unwrap());
        let mut output = BuffWriter::new(File::create(input_path).unwrap());

        let sorted = merge(&mut output, &mut t1, &mut t2);
        output.close();
        t1.close();
        t2.close();

        reads += t1.reads + t2.reads;
        writes += output.writes;

        if sorted {
            break;
        }
    }

    println!("After sorting:");
    print_all(input_path);

    println!("Reads: {reads} Writes: {writes} Total Phases: {phases}");
}
fn merge(output: &mut BuffWriter, t1reader: &mut BuffReader, t2reader: &mut BuffReader) -> bool {
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

fn distribute(input: &mut BuffReader, t1writer: &mut BuffWriter, t2writer: &mut BuffWriter) {
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

fn print_all(input_path: &str) {
    let mut reader = BuffReader::new(File::open(input_path).unwrap());

    while let Some(record) = reader.read() {
        println!("{:?} {:?}", record, count_primes(record.numbers.as_ref()));
    }

    reader.close();
}
