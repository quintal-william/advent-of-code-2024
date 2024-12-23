use std::{
    collections::{HashMap, HashSet},
    vec,
};

use itertools::Itertools;

use crate::day::{Day, Solutions};

pub struct Day23;

type Nodes = HashMap<String, Vec<String>>;

fn to_key(group: Vec<String>) -> String {
    return group.iter().sorted().join(",");
}

fn from_key(key: String) -> Vec<String> {
    return key.split(',').map(|g| String::from(g)).collect();
}

fn get_combinations(arr: Vec<String>, k: u8) -> Vec<Vec<String>> {
    if k == 1 {
        return arr.iter().map(|v| vec![v.clone()]).collect();
    }

    let mut combinations = vec![];
    for i in 0..arr.len() {
        for c in get_combinations(arr[(i + 1)..].to_vec(), k - 1) {
            let mut combination = vec![arr[i].clone()];
            combination.extend(c);
            combinations.push(combination);
        }
    }
    return combinations;
}

// TODO this can be done using get_combinations only right?
fn get_groups(nodes: &Nodes, n: u8) -> HashSet<String> {
    let mut groups = HashSet::new();
    for (node, edges) in nodes.iter() {
        for combination in get_combinations(edges.clone(), n - 1) {
            let mut group = vec![node.clone()];
            group.extend(combination);
            groups.insert(to_key(group));
        }
    }
    return groups;
}

fn is_connected(nodes: &Nodes, group: Vec<String>) -> bool {
    return group.iter().all(|u| {
        group
            .iter()
            .all(|v| u == v || nodes.get(u).unwrap().contains(v))
    });
}

impl Day for Day23 {
    type Context = Nodes;
    type Part1 = i32;
    type Part2 = String;

    fn title() -> String {
        String::from("LAN Party")
    }

    fn solutions() -> Solutions<Self::Part1, Self::Part2> {
        Solutions {
            part1_example: Some(7),
            part1: Some(1149),
            part2_example: Some(String::from("co,de,ka,ta")),
            part2: Some(String::from("as,co,do,kh,km,mc,np,nt,un,uq,wc,wz,yo")),
        }
    }

    fn create_context(input: &str) -> Self::Context {
        let edges: Vec<Vec<String>> = input
            .split('\n')
            .map(|line| line.split('-').map(|v| String::from(v)).collect())
            .collect();

        let mut nodes = HashMap::new();
        for edge in edges {
            let mut u = nodes.get(&edge[0]).unwrap_or(&vec![]).clone();
            u.push(edge[1].clone());
            nodes.insert(edge[0].clone(), u);

            let mut v = nodes.get(&edge[1]).unwrap_or(&vec![]).clone();
            v.push(edge[0].clone());
            nodes.insert(edge[1].clone(), v);
        }
        return nodes;
    }

    fn solve_part1(nodes: &Self::Context) -> Self::Part1 {
        let mut connected_groups = 0;
        for group in get_groups(nodes, 3) {
            let g = from_key(group);
            if g.iter().any(|v| v.starts_with('t')) && is_connected(nodes, g) {
                connected_groups += 1;
            }
        }
        return connected_groups;
    }

    fn solve_part2(nodes: &Self::Context) -> Self::Part2 {
        let mut group_size = 4;
        let mut connected_groups: Option<HashSet<String>> = None;
        while connected_groups.clone().map(|c| c.len()).unwrap_or(0) != 1 {
            connected_groups = Some(HashSet::from_iter(
                get_groups(nodes, group_size)
                    .into_iter()
                    .filter(|g| is_connected(nodes, from_key(g.clone()))),
            ));
            group_size += 1;
        }
        return connected_groups
            .clone()
            .unwrap()
            .iter()
            .next()
            .unwrap()
            .clone();
    }
}
