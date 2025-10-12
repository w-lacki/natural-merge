use std::cmp::Ordering;

pub const MAX_NUMBERS: usize = 10;

#[derive(Debug, Clone)]
pub struct Record {
    pub(crate) numbers: Vec<u8>,
}

fn count_primes(numbers: &Vec<u8>) -> usize {
    numbers.iter().filter(|&&x| is_prime(x)).count()
}

fn is_prime(number: u8) -> bool {
    if number < 2 {
        return false;
    }
    for i in 2..=number / 2 {
        if number % i == 0 {
            return false;
        }
    }
    true
}

impl Eq for Record {}

impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        count_primes(self.numbers.as_ref()) == count_primes(other.numbers.as_ref())
    }
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Record {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare based on the `age` field.
        count_primes(self.numbers.as_ref()).cmp(&count_primes(&other.numbers.as_ref()))
    }
}