use crate::model::{MAX_NUMBERS, Record};
use std::fs::File;
use std::io::{Read, Write};

const BUFFER_SIZE: usize = 8192;
pub struct BuffWriter {
    path: File,
    buffer: [u8; BUFFER_SIZE],
    pos: usize,
    pub(crate) writes: usize,
}

impl BuffWriter {
    pub fn new(path: File) -> Self {
        BuffWriter {
            path,
            buffer: [0; BUFFER_SIZE],
            pos: 0,
            writes: 0,
        }
    }

    pub fn write(&mut self, record: Record) {
        let len = record.numbers.len() as u8;
        self.write_byte(len);
        for &num in record.numbers.iter() {
            self.write_byte(num);
        }
    }

    pub fn close(&mut self) {
        self.flush();
        self.path.sync_all().unwrap();
    }

    fn write_byte(&mut self, byte: u8) {
        if (self.pos >= BUFFER_SIZE) {
            self.flush();
        }

        self.buffer[self.pos] = byte;
        self.pos += 1;
    }

    fn flush(&mut self) {
        self.path.write_all(self.buffer[0..self.pos].as_ref()).expect("IO ERROR");
        self.pos = 0;
        self.writes += 1;
    }
}

pub(crate) struct BuffReader {
    path: File,
    buffer: [u8; BUFFER_SIZE],
    pos: usize,
    len: usize,
    pub(crate) reads: usize,
}

impl BuffReader {
    pub fn read(&mut self) -> Option<Record> {
        let size = self.read_byte()?;
        let mut numbers = Vec::with_capacity(size as usize);

        for _ in 0..size {
            let byte = self.read_byte().expect("READ ERROR");
            numbers.push(byte);
        }

        Some(Record { numbers })
    }

    pub fn new(path: File) -> Self {
        BuffReader {
            path,
            buffer: [0; BUFFER_SIZE],
            pos: 0,
            len: 0,
            reads: 0,
        }
    }
    fn read_byte(&mut self) -> Option<u8> {
        if self.pos >= self.len {
            self.fill();
        }

        if self.pos < self.len {
            let byte = self.buffer[self.pos];
            self.pos += 1;
            Some(byte)
        } else {
            None
        }
    }

    fn fill(&mut self) {
        self.len = self.path.read(&mut self.buffer).expect("READ ERROR");
        self.pos = 0;
        self.reads += 1;
    }

    pub fn close(&mut self) {
        self.path.sync_all().unwrap();
    }
}
