use std::fs;

fn read_input() -> Vec<(i32, Vec<i32>, Vec<i32>)> {
    let input = fs::read_to_string("input").unwrap();
    let lines : Vec<Vec<&str>> = input.lines()
        .map(|line : &str| line.split_whitespace().collect())
        .collect();

    let mut ret : Vec<(i32, Vec<i32>, Vec<i32>)> = Vec::with_capacity(lines.len());
    
    for line in lines {
        let mut iter = line.iter();

        //making first el. of return tuple
        let mut switch = 0_i32;
        if let Some(switch_unbitmasked) = iter.next() {
            let mut pow = 1_i32;
            for ascii in switch_unbitmasked.chars() {
                if ascii == '[' || ascii == ']' {
                    continue;
                }
                if ascii == '#' {
                    switch += pow;
                }

                pow *= 2;
            }
            println!();
        }

        //second element of return tuple
        let masks : Vec<i32> = iter
            .filter(|el| !el.contains('{'))
            .map(|el| {
                let mut iter2 = el.chars();
                let _ = iter2.next();
                let _ = iter2.next_back();
                let positions : Vec<u32> = iter2
                    .filter(|ascii| *ascii != ',')
                    .map(|ascii| ascii as u32 - '0' as u32)
                    .collect();
                
                let mut mask = 0i32;
                for pos in positions.iter() {
                    mask += 2i32.pow(*pos as u32);
                }
                mask
            })
            .collect();

        let joltage_string : String = line
            .into_iter()
            .filter(|el| el.contains('{'))
            .map(|joltage_list| {
                let mut char_iter = joltage_list.chars();
                let _ = char_iter.next();
                let _ = char_iter.next_back();
                char_iter.collect::<String>()
            })
            .collect();

        let joltage_vec : Vec<i32> = joltage_string
            .split(',')
            .map(|a| a.parse::<i32>().unwrap())
            .collect();

        ret.push((
            switch,
            masks,
            joltage_vec
        ));
    }
    ret
}

fn main() {
    let input : Vec<(i32, Vec<i32>, Vec<i32>)> = read_input();

    let mut result = 0;
    for (mask, switches, _) in input {
        println!("chechking for mask = {}",mask);
        for s in &switches {
            print!("{} ",*s);
        }
        println!();

        let mut res = 1000;
        let bitmask = |mut bitmask : i32, array : &Vec<i32>| -> i32 {
            let mut num = 0;
            let mut ret = 0;
            while bitmask != 0 {
                if bitmask % 2 != 0 {
                    ret ^= array[num];
                }
                num += 1;
                bitmask /= 2;
            }
            ret
        };
        let work_done = |mut num| -> i32 {
            let mut ret = 0;
            while num != 0 {
                if num % 2 != 0 {
                    ret += 1;
                }
                num /= 2;
            }
            ret
        };
        for num in 0..2i32.pow(switches.len() as u32) {
            let xor = bitmask(num, &switches);
            print!("bitmask for {} is {}",num, xor);
            if xor == mask {
                res = i32::min(res,work_done(num));
                print!(" we did it! xor={} mask={}",xor,mask);
                print!("work done is {}",work_done(num));
            }
            println!();
        }
        println!();
        result += res;
    }
    println!("result is {}",result);
}
