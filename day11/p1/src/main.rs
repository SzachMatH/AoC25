use std::fs;
use std::collections::{HashSet, HashMap};

const GRAPH_SIZE : usize = 800;

fn read_input() -> (usize, usize, [Vec<usize> ; GRAPH_SIZE]) {
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

    //let mut map : HashMap<&str, usize> = lines
    //    .iter()
    //    .map(|el| el.0)
    //    .enumerate()
    //    .map(|el| (el.1, el.0))//ugly ):
    //    .collect();
    //
    //let mut all_vert = HashSet::<&str>::new();
    //for line in lines {
    //    all_vert.insert(line.0);
    //    
    //}
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


    let start = map["you"];
    let finish = map["out"];
    (start,finish, graph.try_into().unwrap())
}

fn dfs(el : usize, append : &mut Vec<usize>, in_deg : &mut Vec<i32>, graph : &[Vec<usize>; GRAPH_SIZE]) {
    append.push(el);
    for edge in &graph[el] {
        in_deg[*edge] -=1;
        if in_deg[*edge] == 0 {
            dfs(*edge, append, in_deg, graph);
        }
    }
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
    let (start, finish, graph) = read_input();
    let top_sort = topological_sort(&graph);

    println!("my topsort is:");
    for a in &top_sort {
        println!("{}",a);
    }

    let mut dp = [0i64; GRAPH_SIZE];
    dp[start] = 1;

    for el in top_sort.iter() {
        if dp[*el] == 0 {
            continue;
        }
        println!("I am at {} my dp is {}",*el,dp[*el]);
        for edge in &graph[*el] {
            dp[*edge] += dp[*el];
        }
    }
    for i in 0..11 {
        print!("{} = ",i);
        for el in &graph[i] {
            print!("{} ",el);
        }
        println!();
    }

    for i in 0..11 {
        println!("dp[{}] is {}",i,dp[i]);

    }
    println!("dp of finish is {}", dp[finish]);
    
}
