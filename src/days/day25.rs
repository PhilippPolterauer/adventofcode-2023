use crate::util::*;
use nalgebra::DVector;
use std::collections::HashSet;
fn parse_input(input: &str) -> Graph<String> {
    let data: Vec<(String, Vec<String>)> = input
        .lines()
        .filter_map(|line| {
            line.split_once(": ").map(|(name, childs)| {
                (
                    name.to_owned(),
                    childs.split(' ').map(|s| s.to_owned()).collect(),
                )
            })
        })
        .collect();
    let mut graph = Graph::new();
    for (name, childs) in data {
        graph.add_node(name.clone());
        for child in childs {
            graph.add_node(child.clone());
            graph.add_edge(&name, &child, 1);
            graph.add_edge(&child, &name, 1);
        }
    }
    graph
}

pub fn part1(input: &str) -> i64 {
    let graph = parse_input(input);
    let deg = graph.degree_matrix();
    let adj = graph.adjacency_matrix();
    let lap = (deg - adj).cast::<f64>();
    let eigensys = lap.symmetric_eigen();

    // A.solve_lower_triangular(b)
    let mut eigenvecs: Vec<(f64, DVector<f64>)> = eigensys
        .eigenvalues
        .into_iter()
        .zip(eigensys.eigenvectors.column_iter())
        .map(|(a, b)| (*a, b.into_owned()))
        .collect();
    eigenvecs.sort_by(|a, b| a.0.total_cmp(&b.0));
    let v2 = &eigenvecs[1].1;
    let grpa: HashSet<usize> = v2
        .iter()
        .enumerate()
        .filter_map(|(id, val)| (val > &0.0).then_some(id))
        .collect();
    let grpb: HashSet<usize> = v2
        .iter()
        .enumerate()
        .filter_map(|(id, val)| (val < &0.0).then_some(id))
        .collect();

    //cnt cuts
    let mut cuts = HashSet::new();
    for a in &grpa {
        for b in &grpb {
            if graph.edges[*a].get(b).is_some() {
                cuts.insert((a, b));
            }
        }
    }
    // make sure we do 3 cuts
    assert_eq!(cuts.len(), 3);
    (grpa.len() * grpb.len()) as i64
}

pub fn part2(_input: &str) -> i64 {
    println!("done!");
    0
}
