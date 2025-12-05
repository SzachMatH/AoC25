use std::fs;

const LENGTH : usize = 138;
const WIDTH : usize = 138;


fn main() {
    let input = fs::read_to_string("input").unwrap();

    let arr : Vec<[u8; LENGTH]> = input.trim()
        .split_whitespace()
        .map(|a| a.as_bytes().try_into().unwrap())
        .collect();
    
    let positions : Vec<(i32, i32)> = vec![(-1,-1), (0, -1), (1,-1),
    (-1,0), (1,0), //WE DO NOT CHECK (0,0) position
    (0,1), (-1,1), (1,1)];

    let okay = |y : i32,x : i32, arr : &Vec<[u8; LENGTH]>| { 
        if y<0 || x<0 || y>=arr.len() as i32 || x>=LENGTH as i32 {
            return true;
        }
        if arr[y as usize][x as usize] == b'@' {
            return false;
        }
        true
    };


    let mut result = 0;
    for i in 0..arr.len() {
        for j in 0..WIDTH {
            if arr[i][j] == b'.' {
                continue;
            }
            let mut count = 0;
            for pos in &positions {//this is something worth remembering
                //namely, when I want to iterate over something and not move just use &
                //simple yet worth reminding
                if okay((i as i32)-pos.0, (j as i32)-pos.1, &arr) {
                    //println!("okay is {}{} for {}{}",(j as i32)-pos.1,(i as i32)-pos.0,j,i);
                    count += 1;
                }
            }
            if 8-count < 4 {
                //println!("x={}, y={}",j,i);
                result += 1;
            }
        }
    }
    println!("the result is {}", result);
}
