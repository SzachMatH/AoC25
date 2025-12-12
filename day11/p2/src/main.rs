use std::fs;
use std::collections::{HashSet, HashMap};

const GRAPH_SIZE : usize = 800;

fn read_input() -> (usize, usize, usize, usize, [Vec<usize> ; GRAPH_SIZE]) {
    let input = fs::read_to_string("input").unwrap();
    let lines : Vec<(&str,Vec<&str>)> = input
        .lines()
        .map(|line| line.split_once(':').unwrap())
        .map(|pair : (&str, &str)| {(
            pair.0,
            pair.1
                .split_whitespace()
                .collect()
        )})
        .collect();

    let all_vert : HashSet<&str> = lines.iter()
        .fold(HashSet::new(), |mut acc, line| {
            acc.insert(line.0);
            for el in &line.1 {
                acc.insert(el);
            } 
            acc
        });

    let map : HashMap<&str, usize> = all_vert
        .into_iter()
        .enumerate()
        .map(|(a,b)| (b,a))
        .collect();


    for (key, val) in map.iter() {
        println!("{} is {}", key, val);
    }

    let mut graph = vec![Vec::<usize>::new(); GRAPH_SIZE];
    for el in lines {
        graph[map[el.0]] = el.1.into_iter().map(|a| map[a]).collect();
    }


    let start = map["svr"];
    let finish = map["out"];
    let p1 = map["fft"];
    let p2 = map["dac"];
    (start,finish, p1, p2, graph.try_into().unwrap())
}

fn topological_sort(graph : &[Vec<usize>; GRAPH_SIZE]) -> Vec<usize> {
    let mut in_deg = vec![0; graph.len()];
    for el in 0..graph.len() {
        for edge in &graph[el] {
            in_deg[*edge] += 1;
        }
    }

    let mut stack : Vec<usize> = (0..graph.len())
        .filter(|a| in_deg[*a] == 0 )
        .collect();

    let mut ret = Vec::<usize>::with_capacity(graph.len());
    let mut visited = vec![false; graph.len()];
    while !stack.is_empty() {
        let curr = stack.pop().unwrap();
        visited[curr] = true;
        ret.push(curr);
        for edge in &graph[curr] {
            in_deg[*edge] -=1;
            if in_deg[*edge] == 0 && !visited[*edge] {
                stack.push(*edge);
            }
        }
    }
    ret
}

fn main() {
    let (start, finish, p1, p2, graph) = read_input();
    let top_sort = topological_sort(&graph);

    println!("my topsort is:");
    for a in &top_sort {
        println!("{}",a);
    }

    let mut dp = [[0i64; GRAPH_SIZE]; 4];
    dp[0][start] = 1;

    //01 - p2 is on
    //10 - p1 is on
    //11 - p1 and p2 is on
    //(We do bitmasking)
    for el in top_sort.iter() {
        for edge in &graph[*el] {
            if *edge == p1 {
                dp[2][*edge] += dp[0][*el];
                dp[3][*edge] += dp[1][*el];
            } else if *edge == p2 {
                dp[1][*edge] += dp[0][*el];
                dp[3][*edge] += dp[2][*el];
            } else {
                dp[0][*edge] += dp[0][*el];
                dp[1][*edge] += dp[1][*el];
                dp[2][*edge] += dp[2][*el];
                dp[3][*edge] += dp[3][*el];
            }
        }
    }
    for i in 0..11 {
        print!("{} = ",i);
        for el in &graph[i] {
            print!("{} ",el);
        }
        println!();
    }

    //for i in 0..11 {
    //    println!("dp[{}] is {}",i,dp[i]);
    //
    //}
    println!("dp of finish is {}", dp[3][finish]);
    
}
