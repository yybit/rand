extern crate rand;
extern crate uuid;

use std::env;
use rand::Rng;
use uuid::Uuid;

const NUMBERSET: &[u8] = b"0123456789";
const LOWERCASESET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPERCASESET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const SYMBOLSET: &[u8] = b")(*&^%$#@!~";
const ALLSET: &[&[u8]] = &[NUMBERSET, LOWERCASESET, UPPERCASESET, SYMBOLSET];
const DEFAULTSET: &[&[u8]] = &[NUMBERSET, LOWERCASESET, UPPERCASESET];

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("require length");
    }

    if args[1] == "uuid4" {
        println!("{}", Uuid::new_v4());
        return;
    }
    let length: usize = args[1].parse().unwrap();

    let v: Vec<u8> = if args.len() > 2 {
        let pattern: &[u8] = args[2].as_bytes();
        ALLSET.iter().cloned().filter(|&set| pattern.iter().any(|b| set.contains(b))).collect::<Vec<_>>().concat()
    } else {
        DEFAULTSET.concat()
    };

    let mut rng = rand::thread_rng();
    let rand_string: String = (0..length).map(|_| {
        let idx = rng.gen_range(0, v.len());
        v[idx] as char
    }).collect();

    println!("{}", rand_string);
}
