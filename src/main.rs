mod io;
mod model;
mod sort;

use crate::model::Record;
use io::BuffWriter;
use rand::prelude::*;

fn write_input(records: Vec<Record>) {
    let mut writer =
        BuffWriter::new(std::fs::File::create("input").expect("Can't write to input file."));
    records.iter().for_each(|record| {
        writer.write(record.clone());
    });
    writer.close();
    let written = records.len();
    println!("Written {written} records to input file.");
}

fn run_cmd(option: Option<String>) {
    match option {
        Some(command) => match command.as_str() {
            "--verbose" => sort::sort("input", "t1", "t2", true),
            _ => println!("Unknown run subcommand: {}", command),
        },
        None => {
            sort::sort("input", "t1", "t2", false);
        }
    }
}

fn input_cmd(command: &str) {
    match command {
        "random" => {
            let count: usize = std::env::args()
                .nth(3)
                .expect("Missing count.")
                .parse()
                .expect("Invalid count.");

            let mut rng = rand::thread_rng();
            let mut records: Vec<Record> = Vec::with_capacity(count);

            for _ in 0..count {
                let len: usize = rng.gen_range(1..=model::MAX_NUMBERS);
                let numbers: Vec<u8> = (0..len).map(|_| rng.gen_range(0..=100)).collect();
                records.push(Record { numbers });
            }

            write_input(records);
        }
        "manual" => {
            let input = std::env::args().nth(3).expect("Missing input.");
            let records = input
                .split("S")
                .map(|s| {
                    let numbers: Vec<u8> = s
                        .split(",")
                        .map(|num_str| num_str.parse::<u8>().expect("Invalid number"))
                        .collect();
                    
                    if (numbers.len() > model::MAX_NUMBERS) {
                        panic!("Record exceeds maximum allowed numbers: {}", model::MAX_NUMBERS);
                    }

                    Record { numbers }
                })
                .collect();
            write_input(records);
        }
        _ => {
            println!("Unknown input command: {command}");
        }
    }
}
fn run_command(command: &str) {
    match command {
        "sort" => {
            let sub_command = std::env::args().nth(2);
            run_cmd(sub_command);
        }
        "input" => {
            let sub_command = std::env::args().nth(2).expect("Missing input subcommand.");
            input_cmd(sub_command.as_str());
        }
        _ => {
            println!("Unknown command: {command}");
        }
    }
}
fn options() {
    let command = std::env::args().nth(1);

    match command {
        Some(command) => run_command(command.as_str()),
        None => println!(
            r"
Usage:
    program sort [--verbose] - Sort the input file. Use --verbose to print each phase.
    program input random <count> - Generate <count> random records and write to input file
    program input manual <records> - Write manual records to input file. Records should be in the format: num1,num2S...Snum1,num2"
        ),
    }
}

fn main() {
    options();
}
