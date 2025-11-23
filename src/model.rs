use std::cmp::Ordering;

pub const MAX_NUMBERS: usize = 15;

/// A compile-time function to build the 256-element prime table.
const fn build_prime_table() -> [bool; 256] {
    let mut table = [false; 256];
    let mut i = 0;

    // `while` loops are allowed in const functions.
    while i < 256 {
        table[i] = is_prime(i);
        i += 1;
    }

    table
}

// -----------------------------------------------------------------
// Here is the static array, initialized at compile time.
// -----------------------------------------------------------------
static IS_PRIME_TABLE: [bool; 256] = build_prime_table();

#[derive(Debug, Clone)]
pub struct Record {
    pub(crate) numbers: Vec<u8>,
}

pub fn count_primes(numbers: &Vec<u8>) -> usize {
    numbers.iter().filter(|&&x| IS_PRIME_TABLE[x as usize]).count()
}

const fn is_prime(n: usize) -> bool {
    // 0 and 1 are not prime numbers.
    if n <= 1 {
        return false;
    }
    // 2 is the only even prime number.
    if n == 2 {
        return true;
    }
    // Other even numbers are not prime.
    if n % 2 == 0 {
        return false;
    }

    // Check for odd divisors from 3 up to the square root of n.
    // We use `i * i <= n` to avoid floating-point math.
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            // Found a divisor, so it's not prime.
            return false;
        }
        i += 2;
    }

    // No divisors found, it's prime.
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
