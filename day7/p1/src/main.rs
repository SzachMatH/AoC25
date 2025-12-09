use std::fs;

fn read_input() -> Vec< Vec<u8> > {
    let input = fs::read_to_string("input").unwrap();
    let vec = input
        .lines()
        .map(|a| a.to_string().into_bytes())
        .collect();
    vec
}

fn main() {
    let mut input = read_input();
    let mut ans : Vec< Vec<i64> > = vec![vec![0; input[0].len()]; input.len()];

    let mut res = 0;
    for i in 0..input.len()-1 { //rows
        for j in 0..input[i].len() {//I looked at input & it seems last line is empty
            if input[i][j] == b'S' {
                input[i+1][j] = b'|';
                ans[i+1][j]=1;
            }
            if input[i][j] == b'|' && input[i+1][j] == b'^' {
                res += 1;
                input[i+1][j-1] = b'|'; // again no need to check bound conditions
                input[i+1][j+1] = b'|'; // since the input is the way it is
                ans[i+1][j+1] += ans[i][j];
                ans[i+1][j-1] += ans[i][j];
            }
            if input[i][j] == b'|' && input[i+1][j] != b'^' {
                input[i+1][j] = b'|';
                ans[i+1][j] += ans[i][j];
            }
        }
        for k in 0..input.len() {
            for l in 0..input[k].len() {
                print!("{}",ans[k][l]);
            }
            for l in 0..input[k].len() {
                print!("{}",input[k][l] as char);
            }
            println!();
        }
    }
    let mut sum = 0;
    for el in &ans[input[0].len()-1] {
        sum+=el;
    }
    println!("res is {} sum is {}",res,sum);

}
