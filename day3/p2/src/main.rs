use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let banks : Vec<&[u8]> = input
        .trim()
        .split_whitespace()
        .map(|a| a.as_bytes())
        .collect();
    
    let mut sum : i32 = 0;
    for bank in banks {
        let mut bigest = b'0';
        let mut add = 0;
        for el in bank.iter() {
            add = u8::max(add,(bigest - b'0')*10 + (el - b'0'));
            if bigest < *el {
                bigest = *el;
            }
        } 
        sum += add as i32;
    }
    println!("sum is {}",sum);
}
