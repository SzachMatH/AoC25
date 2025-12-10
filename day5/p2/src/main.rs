use std::fs;
use std::str::FromStr;
use std::collections::{HashMap,HashSet};

const INF : i64 = 1_000000_000000_00000;

fn read_input() -> (Vec<(i64,i64)>, Vec<i64>) {
    let input = fs::read_to_string("input").unwrap();
    //the method you're about to see is bad for large data

    let segments : Vec<(i64, i64)> = input.trim()
        .lines()
        .filter(|a| a.contains('-'))
        .map(|a| a.split_once('-').unwrap())
        .map(|a| (i64::from_str(a.0).unwrap(), i64::from_str(a.1).unwrap()))
        .collect();


    let numb : Vec<i64> = input.trim()
        .lines()
        .filter(|a| !a.contains('-') && !a.trim().is_empty())
        .map(|a| i64::from_str(a).unwrap())
        .collect();
    (segments,numb)
}

fn prepare_segments(segments : &Vec<(i64, i64)>) -> Vec<(i64, u8)> {
    //This is bad code and should ne improved!
    let n1_iter = segments.iter().map(|a| (a.0,b'('));//notice that '(' comes before ')' in ASCII
    let n2_iter = segments.iter().map(|a| (a.1,b')'));
    let mut values : Vec<(i64,u8)> = n1_iter.chain(n2_iter).collect();
    values.sort();
    values
}

fn main() {
    let (segments, numbers) = read_input();
    let vec = prepare_segments(&segments);

    let mut res : i64 = 0;
    let mut counter = 0;
    let mut start = 0;
    for boundary in vec {
        if boundary.1 == b'(' {
            if counter == 0 {
                start = boundary.0;
            }
            counter +=1;
        }
        if boundary.1 == b')' {
            if counter == 1 {
                res += boundary.0 - start + 1;
            }
            counter -=1;
        }
    }
    println!("{}",res);

} 
