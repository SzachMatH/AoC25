#![allow(dead_code, reason = "I coded another solution, but it does not work and I want it to still be here")]
use std::fs;
use std::str::FromStr;
use std::collections::{HashMap,HashSet};

const MAX_TREE : usize = 16;
const INF : i64 = 1_000000_000000_00000;

fn read_input() -> (Vec<(i64,i64)>, Vec<i64>) {
    let input = fs::read_to_string("input").unwrap();
    //the method you're about to see is bad for large data

    let segments : Vec<(i64, i64)> = input.trim()
        .lines()
        .filter(|a| a.contains('-'))
        .map(|a| a.split_once('-').unwrap())
        .map(|a| (i64::from_str(a.0).unwrap(), i64::from_str(a.1).unwrap()))
        .collect();

    let numb : Vec<i64> = input.trim()
        .lines()
        .filter(|a| !a.contains('-') && !a.trim().is_empty())
        .map(|a| i64::from_str(a).unwrap())
        .collect();
    (segments,numb)
}

fn prepare_segments(segments : &Vec<(i64, i64)>) -> HashMap<i64,usize> {
    //This is bad code and should ne improved!
    let n1_iter = segments.iter().map(|a| a.0);
    let n2_iter = segments.iter().map(|a| a.1);
    let mut values : HashSet<i64> = n1_iter.chain(n2_iter).collect();
    values.insert(-1); // this number represents numbers smaller than smaller left side segment
    values.insert(INF); // ----//----- higher than any right side of any segment
    let mut vect : Vec<i64> = values.into_iter().collect();
    vect.sort();

    let map = vect
        .into_iter()
        .enumerate()
        .map(|a| (a.1,a.0))
        .collect();
    map
}

fn bin_search(num : i64, arr : &Vec<i64>) -> usize {
    let mut l = 0;
    let mut r = arr.len();
    let good = |x : usize| {
        arr[x] > num
    };
    while r-l > 1 {
        let mid = l+(r-l)/2;
        if good(mid) {
            r=mid;
        } else {
            l=mid;
        }
    }
    l
}

struct SegTree {
    arr : Box<Vec<bool>>,
}

impl SegTree {
    fn query(&self,mut num : usize) -> bool {
        num += MAX_TREE;
        while num > 0 {
            if self.arr[num] {
                return true;
            }
            num/=2;
        }
        //todo!("make sure I shouldn't add +1 in num");
        false
    }

    fn update(&mut self,mut l : usize,mut r : usize) {
        r += MAX_TREE;
        l += MAX_TREE;
        while l < r {
            if l%2 == 1 { // l is a right child
                self.arr[l] = true;
                l-=1;
            }
            if r%2 == 1 { // r is a right child
                self.arr[r-1] = true;
                r-=1;
            }
            r/=2;
            //haha
            l/=2;
        }//I'm a little rusty in implementing segtrees, so this might be wrong!
    }
}

fn prepare_segment_tree(segments : &Vec<(i64, i64)>, map : &HashMap<i64,usize>) -> SegTree {
    let mut segtree = SegTree {
        arr : Box::new(vec![false; MAX_TREE*2+10]),
    };
    for seg in segments {
        segtree.update(map[&seg.0], map[&seg.1]+1); //should +1 be here??????????
    }
    segtree
}

fn main() {
    let (segments, numbers) = read_input();
    let map = prepare_segments(&segments);

    //let mut keys : Vec<i64> = map.keys().copied().collect();
    //keys.sort();
    //let segtree = prepare_segment_tree(&segments, &map);

    //let mut result = 0;
    //for num in numbers {
    //    let i = bin_search(num, &keys);
    //    result += segtree.query(i) as i32;
    //}

    let mut res = 0;
    for num in &numbers {
        for seg in &segments {
            if *num >= seg.0 && *num <= seg.1 {
                res += 1;
                break;
            }
        }
    }
    println!("solution is {}",res);
} 
