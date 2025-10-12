mod io;
mod model;
mod sort;

use io::{BuffReader, Reader, Writer};
use rand::prelude::*;

fn main() {
    sort::sort("test", "t1", "t2");
}

fn read_all() {
    let mut reader = BuffReader::new(std::fs::File::open("test").unwrap());

    while let Some(record) = reader.read() {
        println!("{:?}", record);
    }
    reader.close();
}
