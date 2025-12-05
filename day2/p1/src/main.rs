#![allow(unused)]
use std::fs;
use itertools::Itertools;


fn main() {
    let input_string = fs::read_to_string("input").unwrap();
    let input : Vec<(i64,i64)> = input_string
        .trim()
        .split(',')
        .map(|a| {
            let (a0,a1) = a.split_once('-').unwrap();
            (a0.parse::<i64>().unwrap(), a1.parse::<i64>().unwrap())
        })
        .collect();
    
    let mut sum : i64 = 0;
    for segment in input {
        for n in segment.0..=segment.1 {
            let n_str = n.to_string();
            if n_str.len() % 2 != 0 {
                continue;
            }
            let (a,b) = n_str.split_at(n_str.len()/2);
            if a == b {
                sum += n;
            }
        }
    }
    println!("Answer is {}",sum);

}
