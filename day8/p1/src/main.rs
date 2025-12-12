use std::fs;

fn read_input() -> Vec<(i64,i64,i64)> {
    let input = fs::read_to_string("input_test").unwrap();
    let ret : Vec<(i64,i64,i64)> = input
        .lines()
        .map(|line| {
            let vec : Vec<i64> = line.split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect();
            (vec[0],vec[1],vec[2])
        })
        .collect();
    ret
}


fn main() {
    let points = read_input();
    //I know O(n^3) solution, but do I really want to code it?
}
