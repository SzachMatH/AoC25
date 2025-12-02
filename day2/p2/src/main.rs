use std::fs;

fn invalid(num : i64) -> bool {
    let mut ret : bool = false;
    let n_str = num.to_string();
    for i in (1..n_str.len()).filter(|a| n_str.len() % a == 0) {
        let aa = n_str[0..i].to_string();
        let mut result : String = String::new();
        for _ in 0..n_str.len()/i {
            result.push_str(&aa);
        }
        if result == n_str {
            ret = true;
        }
    }
    ret
}

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
                if invalid(n) {
                    sum += n; 
                    println!("!!!!!!!!!!!!!!! WE FOUND {}",n);
                }
            }
        }

    println!("Sum is {}",sum);
    //let summ = input.iter().fold(0i64, |acc, range| {
    //    acc +
    //    (range.0..=range.1)
    //        .filter(|x| !invalid(*x))
    //        .sum::<i64>()
    //});
    //println!("Answer is {}",sum);

}
