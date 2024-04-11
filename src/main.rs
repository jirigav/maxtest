use std::{collections::{HashMap, HashSet}, fs};

use clap::Parser;
use num_bigint::BigUint;
use pyo3::{types::PyModule, Python};

fn load_data(path: &str, block_size: usize) -> Vec<Vec<u8>> {
    let len_of_block_in_bytes = block_size / 8;
    let mut data: Vec<_> = fs::read(path)
        .unwrap()
        .chunks(len_of_block_in_bytes)
        .map(<[u8]>::to_vec)
        .collect();
    if data[data.len() - 1].len() != len_of_block_in_bytes {
        println!("Data are not aligned with block size, dropping last block!");
        data.pop();
    }
    data
}

pub(crate) fn p_value(positive: usize, sample_size: usize, probability: f64) -> f64 {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let scipy = PyModule::import(py, "scipy").unwrap();
        let result: f64 = scipy
            .getattr("stats")
            .unwrap()
            .getattr("binomtest")
            .unwrap()
            .call1((positive, sample_size, probability, "two-sided"))
            .unwrap()
            .getattr("pvalue")
            .unwrap()
            .extract()
            .unwrap();
        result
    })
}

#[derive(Parser, Debug)]
#[command(version)]
pub(crate) struct Args {
    
    /// Length of block of data.
    block_size: usize,

    /// Path to input file.
    input_file_path: String,
    
}

fn main() {
    let args = Args::parse();

    let data = load_data(&args.input_file_path, args.block_size);
    let (training_data, testing_data) = data.split_at(data.len()/2);
    
    let mut int_data = training_data.into_iter().map(|x| BigUint::from_bytes_le(&x)).collect::<Vec<BigUint>>();
    let n = int_data.len();
    int_data.sort();

    let mut counts = HashMap::new();

    let mut count = 1;
    let mut prev_block = int_data[0].clone();

    for block in int_data.iter().skip(1) {

        if *block == prev_block{
            count += 1;
        } else {
            counts.insert(prev_block, count);
            count = 1;
            prev_block = block.clone();
        }
    }

    counts.insert(prev_block, count);

    int_data.dedup();

    int_data.sort_unstable_by(|a, b| counts[b].cmp(&counts[a]));

    let mut occur = 0;

    let mut z_max = -1.0;
    let mut t_max = 0;

    for (i, block) in int_data.iter().enumerate(){
        occur += counts[block];


        let p = (occur as f64)/(n as f64);
        let q = ((i + 1) as f64)/(2_f64.powf(args.block_size as f64));

        let z = (((n as f64)*(p - q))/f64::sqrt((n as f64)*q*(1.0-q))).abs();

        if z > z_max {
            z_max = z;
            t_max = i + 1;
        }
    }

    println!("Training Z-score: {}", z_max);
    let one_blocks = int_data.into_iter().take(t_max).collect::<HashSet::<BigUint>>();

    let c = testing_data.iter().filter(|x| one_blocks.contains(&BigUint::from_bytes_le(x))).count();

    let q = ((t_max) as f64)/(2_f64.powf(args.block_size as f64));
    let n = testing_data.len() as f64;
    let testing_z = (((c as f64) - n*q)/f64::sqrt(n*q*(1.0-q))).abs();

    println!("Testing Z-score: {}", testing_z);
    
    println!("p-value: {:.0e}", p_value(c, testing_data.len(), q));


}
