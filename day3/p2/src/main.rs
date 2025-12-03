use std::fs;

const BANK_SIZE : i64 = 100;

fn recursion(from : usize, left : i64, arr : &[u8; BANK_SIZE as usize]) -> i64 {
    let mut maxx = b'0';
    let mut position = 0;
    for i in from..(BANK_SIZE as usize -left as usize) {
        if maxx < arr[i] {
            maxx = arr[i];
            position = i;
        }
    }
    if left == 0 {
        return (maxx - b'0') as i64;
    }
    10i64.pow(left as u32)*((maxx - b'0') as i64 ) + recursion(position+1, left - 1, arr)
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let banks : Vec<[u8; BANK_SIZE as usize]> = input
        .trim()
        .split_whitespace()
        .map(|a| {
            let bytes = a.as_bytes();
            bytes.try_into().unwrap()
        })
        .collect();
    
    let mut sum : i64 = 0;
    for bank in banks {
        sum += recursion(0,11, &bank);
    }
    println!("sum is {}",sum);
}
