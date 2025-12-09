use std::fs;
use std::cmp::max;

fn read_data() -> Vec<(i128,i128)> {
    let input = fs::read_to_string("input").unwrap();
    let ret = input
        .lines()
        .map(|s| s.split_once(',').unwrap())
        .map(|(a,b)| (a.parse::<i128>().unwrap(),b.parse::<i128>().unwrap()) )
        .collect();
    ret
}


fn main() {
    let reds = read_data();

    let mut res : i128 = 0;
    for i in &reds {
        for j in &reds {
            let height = i128::abs(i.0 - j.0)+1;
            let width = i128::abs(i.1 - j.1)+1;
            let rect =  height*width;
            res = max(rect as i128, res);
        }
    }
    println!("res = {}",res);
}
