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
        for x in 0..NUM_HASHES {
            let idx = self.index(input, x);
            self.bits.set(idx, true);
        }
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
}

fn main() {
    println!("{} maps to {}", "hello world",
             jenkins_one_at_a_time_hash("hello world".as_bytes()).0);
    println!("{} maps to {}", "hello world",
             bernstein_hash("hello world".as_bytes()).0);
    let mut hs = HashSet::new();
    let mut bf = BloomFilter::new(BLOOM_FILTER_SIZE);
    let f = File::open("/usr/share/dict/words").unwrap();
    let file = BufReader::new(&f);
    let mut num_entries: u32 = 0;
    for line in file.lines() {
        let lu1 = line.unwrap();
        let lu = lu1.trim();
        num_entries += 1;
        bf.set(&lu);
        hs.insert(lu.to_owned());
    }
    println!("hello in set: {}", bf.contains("hello"));
    println!("world in set: {}", bf.contains("world"));
    println!("fudge in set: {}", bf.contains("fudge"));
    println!("zxcvbnm in set: {}", bf.contains("zxcvbnm"));

    let alphabet = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    let mut num_matches = 0.0;
    let mut num_false_positives = 0.0;
    for _ in 0..1_000_000 {
        let mut s = String::new();
        for _ in 0..5 { s.push(*(thread_rng().choose(&alphabet).unwrap())); }
        if bf.contains(&s) {
            num_matches += 1.0;
            if !hs.contains(&s) {
                num_false_positives += 1.0;
            } else {
                //println!("{}", s);
            }
        }
    }
    let exponent = (-1.0 * NUM_HASHES as f64 * num_entries as f64) / BLOOM_FILTER_SIZE as f64;
    let predicted_fp_rate = (1.0-((2.71828 as f64).powf(exponent))).powi(NUM_HASHES as i32);
    println!("{} matches from 1M random strings, {:.3}% false positives vs {:.3}% predicted",
             num_matches,
             (num_false_positives as f64) * 100.0/1_000_000.0,
             predicted_fp_rate*100.0);
}
