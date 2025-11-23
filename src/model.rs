use std::cmp::Ordering;

pub const MAX_NUMBERS: usize = 15;

const fn build_prime_table() -> [bool; 256] {
    let mut table = [false; 256];
    let mut i = 0;

    while i < 256 {
        table[i] = is_prime(i);
        i += 1;
    }

    table
}

static IS_PRIME_TABLE: [bool; 256] = build_prime_table();

#[derive(Debug, Clone)]
pub struct Record {
    pub(crate) numbers: Vec<u8>,
}

pub fn count_primes(numbers: &Vec<u8>) -> usize {
    numbers
        .iter()
        .filter(|&&x| IS_PRIME_TABLE[x as usize])
        .count()
}

const fn is_prime(n: usize) -> bool {
    if n <= 1 {
        return false;
    }

    if n == 2 {
        return true;
    }

    if n % 2 == 0 {
        return false;
    }

    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
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
        count_primes(self.numbers.as_ref()).cmp(&count_primes(&other.numbers.as_ref()))
    }
}
