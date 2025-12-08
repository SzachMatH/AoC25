use std::fs;

fn read_input() -> Vec<String> {
    let input = fs::read_to_string("input").unwrap();
    let mut ret : Vec<String> = input
        .split_whitespace()//notice that this works well for spaces and separators!
        .map(|s| s.to_string())
        .collect();
    ret.reverse();
    ret
}


fn main() {
    let input = read_input();
    let operations = vec!['*','+'];
    let num_lines = input.iter()
        .filter(|a| a.contains(operations[0]) || a.contains(operations[1]))
        .count();
    let mut sum : i64 = 0;
    for i in 0..num_lines {
        let mut iterator = input.iter().skip(i).step_by(num_lines);
        sum += match iterator.next() {
            Some(val) => {
                if val == &'+'.to_string() {
                    iterator
                        .map(|a| a.parse::<i64>().unwrap())
                        .sum::<i64>() 
                } else if val == &'*'.to_string() {
                    iterator
                        .map(|a| a.parse::<i64>().unwrap())
                        .fold(1i64, |acc, el| acc * el)
                } else {
                    panic!("???");
                }
            },
            _ => { panic!("Something is wrong!"); }
        }
    }
    println!("sum is {}",sum);
}
