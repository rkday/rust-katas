extern crate bit_vec;
extern crate rand;
use bit_vec::BitVec;
use std::num::Wrapping;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashSet;

use rand::{thread_rng, Rng};

const NUM_HASHES: u32 = 10;
const BLOOM_FILTER_SIZE: usize = 2_350_000;

// The JOAAT hash and the Bernstein hash looked like the simplest hashes to implement from
// http://www.burtleburtle.net/bob/hash/doobs.html.

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
        hash = (hash * Wrapping(33)) ^ Wrapping(*byte as u32);
    }
    hash
}

struct BloomFilter {
    size: usize,
    bits: BitVec,
    num_entries: u32
}

impl BloomFilter {
    pub fn new(size: usize) -> BloomFilter {
        BloomFilter { size: size, bits: BitVec::from_elem(size, false), num_entries: 0 }
    }

    fn index(&self, input: &str, hash_num: u32) -> usize {
        // Instead of using k independent hash functions, we can just use two - 
        // f1(input) + i * f2(input) is effectively a new hash function for each value of i. See
        // https://www.eecs.harvard.edu/~michaelm/postscripts/tr-02-05.pdf. 
        let wx = Wrapping(hash_num);
        let hash = jenkins_one_at_a_time_hash(input.as_bytes()) +
                   (wx * bernstein_hash(input.as_bytes()));
        hash.0 as usize % self.size
    }

    fn set(&mut self, input: &str) {
        for x in 0..NUM_HASHES {
            let idx = self.index(input, x);
            self.bits.set(idx, true);
        }
        self.num_entries += 1;
    }

    fn contains(&self, input: &str) -> bool {
        for x in 0..NUM_HASHES {
            let idx = self.index(input, x);
            if self.bits[idx] == false {
                return false;
            }
        }
        return true;
    }

    fn false_positive_probability(&self) -> f64 {
        let exponent = (-1.0 * NUM_HASHES as f64 * self.num_entries as f64) / self.size as f64;
        (1.0-((2.71828 as f64).powf(exponent))).powi(NUM_HASHES as i32)
    }
}

fn random_5_letter_word() -> String {
    let alphabet = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    let mut s = String::new();
    for _ in 0..5 { s.push(*(thread_rng().choose(&alphabet).unwrap())); }
    s
}

fn main() {
    let mut hs = HashSet::new();
    let mut bf = BloomFilter::new(BLOOM_FILTER_SIZE);
    let f = File::open("/usr/share/dict/words").unwrap();
    let file = BufReader::new(&f);

    // Insert all words into the dictionary into the Bloom filter and, to check for false
    // positives, a set.
    for line in file.lines() {
        let lu = line.unwrap();
        bf.set(&lu);
        hs.insert(lu.to_owned());
    }

    // Do some basic testing to ensure that it's basically working.
    println!("hello in set: {}", bf.contains("hello"));
    println!("world in set: {}", bf.contains("world"));
    println!("fudge in set: {}", bf.contains("fudge"));
    println!("zxcvbnm in set: {}", bf.contains("zxcvbnm"));

    // Generate a million 5-character strings and count the false positives (items which are not in
    // the set but for which the Bloom filter reurns true).
    let mut num_false_positives = 0.0;
    for _ in 0..1_000_000 {
        let s = random_5_letter_word();
        if bf.contains(&s) && !hs.contains(&s) {
            num_false_positives += 1.0;
        }
    }

    println!("From 1M random strings, {:.3}% false positives vs {:.3}% predicted",
             (num_false_positives as f64) * 100.0/1_000_000.0,
             bf.false_positive_probability()*100.0);
}
