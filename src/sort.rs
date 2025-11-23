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

        if (print_each_phase) {
            println!("Phase {phases}: ");
            print_all(input_path);
        }

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
    let mut output_prev: Option<Record> = None;

    let mut t1_next = t1reader.read();
    let mut t2_next = t2reader.read();

    loop {
        if t1_next.is_none() && t2_next.is_none() {
            break;
        }

        let mut t1_run_active = t1_next.is_some();
        let mut t2_run_active = t2_next.is_some();

        while t1_run_active || t2_run_active {
            let take_t1 = if t1_run_active && t2_run_active {
                t1_next.as_ref().unwrap() <= t2_next.as_ref().unwrap()
            } else {
                t1_run_active
            };

            let current_record: Record;
            if take_t1 {
                let rec = t1_next.take().unwrap();
                current_record = rec.clone();

                t1_next = t1reader.read();

                if let Some(next) = &t1_next {
                    if next < &rec {
                        t1_run_active = false;
                    }
                } else {
                    t1_run_active = false;
                }
            } else {
                let rec = t2_next.take().unwrap();
                current_record = rec.clone();

                t2_next = t2reader.read();

                if let Some(next) = &t2_next {
                    if next < &rec {
                        t2_run_active = false;
                    }
                } else {
                    t2_run_active = false;
                }
            }

            output.write(current_record.clone());

            if let Some(prev) = &output_prev {
                if &current_record < prev {
                    sorted = false;
                }
            }
            output_prev = Some(current_record);
        }
    }

    sorted
}

fn distribute(input: &mut BuffReader, t1writer: &mut BuffWriter, t2writer: &mut BuffWriter) {
    let mut prev: Option<Record> = None;
    let mut to_t1 = true;
    let mut runs = 1;
    while let Some(current) = input.read() {
        if (prev.as_ref().is_some_and(|prev| prev > &current)) {
            to_t1 = !to_t1;
            runs += 1;
        }

        if to_t1 {
            t1writer.write(current.clone());
        } else {
            t2writer.write(current.clone());
        }

        prev = Some(current);
    }
    println!("Current runs: {runs}");
}

fn print_all(input_path: &str) {
    let mut reader = BuffReader::new(File::open(input_path).unwrap());

    while let Some(record) = reader.read() {
        println!("{:?} {:?}", record, count_primes(record.numbers.as_ref()));
    }

    reader.close();
}
