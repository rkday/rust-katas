extern crate bit_vec;
use bit_vec::BitVec;

fn jenkins_one_at_a_time_hash(bytes: &[u8]) -> u32 {
    let mut hash: u32 = 0;
    for byte in bytes {
        hash = hash.wrapping_add(*byte as u32);
        hash = hash.wrapping_add(hash << 10);
        hash ^= hash >> 6;
    }
    hash = hash.wrapping_add(hash << 3);
    hash ^= hash >> 11;
    hash = hash.wrapping_add(hash << 15);
    hash
}


fn bernstein_hash(bytes: &[u8]) -> u32 {
    let mut hash: u32 = 0;
    for byte in bytes {
        hash = hash.wrapping_mul(33) ^ (*byte as u32);
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
            let hash = hash_fn1(input.as_bytes()).wrapping_add(hash_fn2(input.as_bytes()).wrapping_mul(x)) as usize;
            self.bits.set(hash % self.size, true);
        }
    }

    fn contains(&self, input: &str) -> bool {
        let hash_fn1 = jenkins_one_at_a_time_hash;
        let hash_fn2 = bernstein_hash;
        for x in 1..11 {
            let hash = hash_fn1(input.as_bytes()).wrapping_add(hash_fn2(input.as_bytes()).wrapping_mul(x)) as usize;
            if self.bits[hash % self.size] == false {
                return false;
            }
        }
        return true;
    }
}

fn main() {
    println!("{} maps to {}", "hello world", jenkins_one_at_a_time_hash("hello world".as_bytes()));
    println!("{} maps to {}", "hello world", bernstein_hash("hello world".as_bytes()));
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
