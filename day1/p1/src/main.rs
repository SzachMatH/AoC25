use std::fs;

const DIAL_SIZE : i32 = 100;

fn brut() -> (i32,Box<Vec<i32>>) {
    let mut ans_arr = Vec::<i32>::new();
    let input = fs::read_to_string("input").unwrap();
    let rotations = input.split_whitespace(); //this gives me an iterator
    
    let mut result : i32 = 0;
    let mut curr_pos : i32 = 50;
    for rot in rotations {
        let ascii = rot.as_bytes();
        let n : i32 = rot[1..].parse().unwrap();
        match ascii[0] {
            b'R' => {
                for _i in 0..n {
                    curr_pos = (1+curr_pos) % DIAL_SIZE;
                    if curr_pos == 0 {
                        result+=1;
                    }
                }
                ans_arr.push(result.clone());
            },
            b'L' => {
                for _i in 0..n {
                    curr_pos = (curr_pos -1 + DIAL_SIZE) % DIAL_SIZE;
                    if curr_pos == 0 {
                        result+=1;
                    }
                }
                ans_arr.push(result.clone());
            }
            _ => { panic!("Bad input format!"); }
        }
    }
    (result,Box::new(ans_arr))
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input")?;
    let rotations = input.split_whitespace(); //this gives me an iterator
    
    let mut result : i32 = 0;
    let mut curr_pos : i32 = 50;

    let (brut_num,res_brut_arr) = brut();
    let mut i=0;
    let mut prev;

    for rot in rotations {
        
        prev = result;

        let ascii = rot.as_bytes();
        let n : i32 = rot[1..].parse().unwrap();

        match ascii[0] {
            b'R' => {
                result += (curr_pos + n)/100;
                curr_pos = (curr_pos + n) % DIAL_SIZE;
            },
            b'L' => {
                if curr_pos == 0 {
                    result += n/100;
                } else {
                    result += (i32::max(n-curr_pos+1,0)+99)/100;
                }
                curr_pos = (curr_pos - n + DIAL_SIZE*1000) % DIAL_SIZE;
            }
            _ => { panic!("Bad input format!"); }
        }

        if i>0 && res_brut_arr[i] - res_brut_arr[i-1] != result-prev {
            println!("letter = {} Difference n={}, position now = {}",str::from_utf8(&[ascii[0]]).unwrap(),n,curr_pos);
            println!("diff brut {}, diff {}, res = {}, res_brut {} prev {}",res_brut_arr[i]-res_brut_arr[i-1], result-prev, result, res_brut_arr[i], prev);
        }
        i+=1;
    }

    println!("Correct safe number is {}, brut num is {}",result, brut_num);

    Ok(())
}
