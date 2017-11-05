extern crate bit_vec;
use bit_vec::BitVec;
use std::num::Wrapping;

fn jenkins_one_at_a_time_hash(bytes: &[u8]) -> Wrapping<u32> {
    let mut hash = Wrapping(0u32);
    for byte in bytes {
        hash += Wrapping(*byte as u32);
        hash += hash << 10;
        hash ^= hash >> 6;
    }
    hash += hash << 3;
    hash ^= hash >> 11;
    hash += hash << 15;
    hash
}


fn bernstein_hash(bytes: &[u8]) -> Wrapping<u32> {
    let mut hash = Wrapping(0u32);
    for byte in bytes {
        hash = hash * Wrapping(33) ^ Wrapping(*byte as u32);
    }
    hash
}

struct BloomFilter {
    size: usize,
    bits: BitVec
}

impl BloomFilter {
    pub fn new(size: usize) -> BloomFilter {
        BloomFilter { size: size, bits: BitVec::from_elem(size, false) }
    }

    fn set(&mut self, input: &str) {
        let hash_fn1 = jenkins_one_at_a_time_hash;
        let hash_fn2 = bernstein_hash;
        for x in 1..11 {
            let wx = Wrapping(x);
            let hash = hash_fn1(input.as_bytes()) + (wx* hash_fn2(input.as_bytes()));
            let index = hash.0 as usize % self.size;
            self.bits.set(index, true);
        }
    }

    fn contains(&self, input: &str) -> bool {
        let hash_fn1 = jenkins_one_at_a_time_hash;
        let hash_fn2 = bernstein_hash;
        for x in 1..11 {
            let wx = Wrapping(x);
            let hash = hash_fn1(input.as_bytes()) + (wx* hash_fn2(input.as_bytes()));
            let index = hash.0 as usize % self.size;
            if self.bits[index] == false {
                return false;
            }
        }
        return true;
    }
}

fn main() {
    println!("{} maps to {}", "hello world",
             jenkins_one_at_a_time_hash("hello world".as_bytes()).0);
    println!("{} maps to {}", "hello world",
             bernstein_hash("hello world".as_bytes()).0);
    let mut bf = BloomFilter::new(1_000_000);
    println!("hello in set: {}", bf.contains("hello"));
    println!("world in set: {}", bf.contains("world"));
    bf.set("hello");
    println!("hello in set: {}", bf.contains("hello"));
    println!("world in set: {}", bf.contains("world"));
    bf.set("world");
    println!("hello in set: {}", bf.contains("hello"));
    println!("world in set: {}", bf.contains("world"));
}
