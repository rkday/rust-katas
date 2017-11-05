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

    fn index(&self, input: &str, hash_num: u32) -> usize {
        let wx = Wrapping(hash_num);
        let hash = jenkins_one_at_a_time_hash(input.as_bytes()) +
                   (wx * bernstein_hash(input.as_bytes()));
        hash.0 as usize % self.size
    }

    fn set(&mut self, input: &str) {
        for x in 0..10 {
            let idx = self.index(input, x);
            self.bits.set(idx, true);
        }
    }

    fn contains(&self, input: &str) -> bool {
        for x in 0..10 {
            let idx = self.index(input, x);
            if self.bits[idx] == false {
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
