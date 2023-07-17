use std::time::Instant;
use crate::utils::print_matrix;
use itertools::Itertools;

pub fn christofides(adjacency_matrix: &Vec<Vec<u32>>) {
    // -> Vec<Vec<u32>>
    let mut start_time = Instant::now();
    let mut mst: Vec<Vec<u32>> = prim(adjacency_matrix);
    print_matrix(&mst);

    let vertices_with_odd_degree = vertices_with_odd_degree(&mst);
    println!("{:?}", vertices_with_odd_degree);

    let mut subgraph_with_odd_degree_vertices = subgraph_with_odd_degree_vertices(&vertices_with_odd_degree, &adjacency_matrix);
    print_matrix(&subgraph_with_odd_degree_vertices);

    let full_matching = vec![vec![8,7], vec![6,9]]; // NetworkX python script
    //let eulerian_multigraph = create_eulerian_multigraph(&mst, full_matching);
    //print_matrix(&eulerian_multigraph);

    let mut end_time = Instant::now();
    let mut elapsed_time = end_time - start_time;
    println!("Christofides : {:?}", elapsed_time);
}

pub fn create_eulerian_multigraph(mst : Vec<Vec<u32>>, full_matching: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut multigraph: Vec<Vec<u32>> = vec![vec![0; mst.len()]; mst.len()];
    for i in 0..mst.len() {
       for j in 0..mst.len() {
            if mst[i][j] > 0 {
                multigraph[i][j] += 1;
                multigraph[j][i] += 1;
            }
       }
    }
    for pair in full_matching {
        let i = pair[0];
        let j = pair[1];
        multigraph[i][j] += 1;
        multigraph[j][i] += 1;
    }

    return multigraph;
}

pub fn vertices_with_odd_degree(adjacency_matrix: &Vec<Vec<u32>>) -> Vec<u32> {
    let mut vertices_with_odd_degree: Vec<u32> = Vec::new();
    for vertex in 0..adjacency_matrix.len() {
        let row = &adjacency_matrix[vertex as usize];
        let degree = row.iter().filter(|&value| *value != 0).count();
        if degree % 2 != 0 {vertices_with_odd_degree.push(vertex as u32)};
    }

    return vertices_with_odd_degree;
}

pub fn prim(adjacency_matrix: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut mst: Vec<Vec<u32>> = vec![vec![0; adjacency_matrix.len()]; adjacency_matrix.len()];
    let mut visited: Vec<bool> = vec![false; adjacency_matrix.len()];
    visited[0] = true;

    while visited.iter().any(|&v| !v) {
        let mut a = 0;
        let mut b = 0;
        let mut min_value = u32::MAX;

        for i in 0..adjacency_matrix.len() {
            if visited[i] {
                let row = &adjacency_matrix[i];
                for j in 0..row.len() {
                    if !visited[j] && row[j] != 0 && row[j] < min_value {
                        min_value = row[j];
                        a = i;
                        b = j;
                    }
                }
            }
        }

        mst[a][b] = min_value;
        mst[b][a] = min_value;
        visited[b] = true;
    }

    return mst;
}

pub fn subgraph_with_odd_degree_vertices(vertices_with_odd_degree: &Vec<u32>, adjacency_matrix: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut subgraph: Vec<Vec<u32>> = vec![vec![0; adjacency_matrix.len()]; adjacency_matrix.len()];
    for pair in vertices_with_odd_degree.iter().combinations(2) {
        let i = *pair[0] as usize;
        let j = *pair[1] as usize;
        subgraph[i][j] = adjacency_matrix[i][j];
        subgraph[j][i] = adjacency_matrix[j][i];
    }
    return subgraph;
}

