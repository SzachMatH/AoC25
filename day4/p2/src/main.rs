use std::fs;
use std::{thread, time};

const LENGTH : usize = 138;
const WIDTH : usize = 138;


fn main() {
    let input = fs::read_to_string("input").unwrap();
    println!("{}",input);

    let mut curr : Vec<[u8; LENGTH]> = input.trim()
        .lines()
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
    let mut prev = vec![[0; LENGTH]; WIDTH];
    let mut change = vec![[0; LENGTH]; WIDTH];
    while &prev != &curr {   
        for i in 0..curr.len() {
            for j in 0..WIDTH {
                if curr[i][j] == b'.' {
                    change[i][j] = b'.';
                    continue;
                }
                let mut count = 0;
                for pos in &positions {//this is something worth remembering
                    //namely, when I want to iterate over something and not move just use &
                    //simple yet worth reminding
                    if okay((i as i32)-pos.0, (j as i32)-pos.1, &curr) {
                        //println!("okay is {}{} for {}{}",(j as i32)-pos.1,(i as i32)-pos.0,j,i);
                        count += 1;
                    }
                }
                if 8-count < 4 {
                    //println!("x={}, y={}",j,i);
                    result += 1;
                    change[i][j] = b'.';
                } else {
                    change[i][j] = curr[i][j];
                }
            }
        }
        prev = curr.clone();
        curr = change.clone();
        change = vec![[0; LENGTH]; WIDTH];

        print!("\x1B[2J\x1B[1;1H");
        
        for i in 0..curr.len() {
            let a : &str = str::from_utf8(&curr[i]).unwrap();
            println!("{}",a);
        }
        thread::sleep(time::Duration::from_millis(100));
        println!("");
    } 
    println!("the result is {}", result);
}
