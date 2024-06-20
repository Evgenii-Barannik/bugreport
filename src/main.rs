use rand::prelude::*;
use std::fs::File;

const BIG_NUM: u64 = 10_000_000;

fn main() {
    let mut rng = rand::thread_rng();
    let mut v: Vec<String> = vec![];

    for _ in 0..BIG_NUM {
        let random_number: u128 = rng.gen();
        v.push(format!("{:x}", random_number))
    }

    let result_file= File::create("result.txt").unwrap();
    serde_json::to_writer_pretty(result_file, &v).unwrap();
}
