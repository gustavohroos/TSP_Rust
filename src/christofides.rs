use std::time::Instant;
use crate::utils::*;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

type Vertex = usize;

pub fn christofides(adjacency_matrix: &Vec<Vec<u32>>) -> (Vec<u32>, u32) {
    // -> Vec<Vec<u32>>
    let mut start_time = Instant::now();
    let mut mst: Vec<Vec<u32>> = prim(adjacency_matrix);
    // println!("MST");
    // print_matrix(&mst);

    let vertices_with_odd_degree = vertices_with_odd_degree(&mst);
    // println!("Vertices with odd degree");
    // println!("{:?}", vertices_with_odd_degree);

    let mut subgraph_with_odd_degree_vertices = subgraph_with_odd_degree_vertices(&vertices_with_odd_degree, &adjacency_matrix);
    // println!("Subgraph with odd degree vertices");
    // print_matrix(&subgraph_with_odd_degree_vertices);

    // let full_matching = vec![(8,7), (6,9)]; // tsp1_253 NetworkX python script
    // let full_matching = vec![(0,2)]; // tsp2_1248
    let full_matching = minimum_weight_perfect_matching(&subgraph_with_odd_degree_vertices);
    let unique_matching = remove_duplicate_edges(full_matching);
    
    println!("Full matching");
    println!("{:?}", unique_matching);
    let mut eulerian_multigraph = create_eulerian_multigraph(mst, unique_matching);
    // println!("Eulerian multigraph");
    // print_matrix(&eulerian_multigraph);

    let euler_tour = find_eulerian_cycle(&mut eulerian_multigraph);
	println!("Euler tour \n{:?}", euler_tour);

    let mut hamiltonian_path: Vec<u32> = Vec::new();
    for vertex in euler_tour {
        if !hamiltonian_path.contains(&vertex) {
            hamiltonian_path.push(vertex);
        }
    }

    // Reference to the original vector
    let reference_to_vec: &Vec<u32> = &hamiltonian_path;

    // New vector containing references to the elements in the original vector
    let transformed_vec: Vec<&u32> = reference_to_vec.iter().collect();

    let cost = calculate_cost(&transformed_vec, &adjacency_matrix);

    let mut end_time = Instant::now();
    let mut elapsed_time = end_time - start_time;
    println!("Christofides : {:?}", elapsed_time);
    println!("Cost: {}", cost);
    println!("Path: {:?}", hamiltonian_path);

    (hamiltonian_path, cost)
}

pub fn create_eulerian_multigraph(mst : Vec<Vec<u32>>, full_matching: Vec<(usize, usize)>) -> Vec<Vec<u32>> {
    let mut multigraph: Vec<Vec<u32>> = vec![vec![0; mst.len()]; mst.len()];
    for i in 0..mst.len() {
       for j in 0..mst.len() {
            if mst[i as usize][j as usize] > 0 {
                multigraph[i as usize][j as usize] += 1;
            }
       }
    }
    for (u, v) in full_matching {
        multigraph[u][v] += 1;
        multigraph[v][u] += 1;
    }

    multigraph
}

fn minimum_weight_perfect_matching(adjacency_matrix: &Vec<Vec<u32>>) -> Vec<(Vertex, Vertex)> {
    let num_vertices = adjacency_matrix.len();
    let mut matching: HashMap<Vertex, Vertex> = HashMap::new();
    let mut visited: HashSet<Vertex> = HashSet::new();

    for v in 0..num_vertices {
        if !matching.contains_key(&v) {
            if let Some(path) = find_augmenting_path(adjacency_matrix, &matching, v, &mut visited) {
                augment_matching(&mut matching, &path);
            }
        }
    }

    matching.into_iter().collect()
}

fn find_augmenting_path(
    adjacency_matrix: &Vec<Vec<u32>>,
    matching: &HashMap<Vertex, Vertex>,
    v: Vertex,
    visited: &mut HashSet<Vertex>,
) -> Option<Vec<Vertex>> {
    let num_vertices = adjacency_matrix.len();
    let mut queue = VecDeque::new();
    queue.push_back(v);
    visited.clear();
    visited.insert(v);
    let mut parent: HashMap<Vertex, Vertex> = HashMap::new();
    parent.insert(v, v);

    while let Some(current) = queue.pop_front() {
        for u in 0..num_vertices {
            if adjacency_matrix[current][u] > 0 && !visited.contains(&u) {
                visited.insert(u);
                queue.push_back(u);
                parent.insert(u, current);

                if !matching.contains_key(&u) {
                    return Some(reconstruct_path(&parent, v, u));
                }
            }
        }
    }

    None
}

fn reconstruct_path(parent: &HashMap<Vertex, Vertex>, start: Vertex, end: Vertex) -> Vec<Vertex> {
    let mut path = vec![end];
    let mut current = end;

    while current != start {
        current = parent[&current];
        path.push(current);
    }

    path.reverse();
    path
}

fn augment_matching(matching: &mut HashMap<Vertex, Vertex>, path: &Vec<Vertex>) {
    for i in (0..path.len() - 1).step_by(2) {
        matching.insert(path[i], path[i + 1]);
        matching.insert(path[i + 1], path[i]);
    }
}

fn remove_duplicate_edges(matching: Vec<(Vertex, Vertex)>) -> Vec<(Vertex, Vertex)> {
    let mut unique_edges: HashSet<(Vertex, Vertex)> = HashSet::new();
    let mut result: Vec<(Vertex, Vertex)> = Vec::new();

    for edge in matching {
        if !unique_edges.contains(&edge) && !unique_edges.contains(&(edge.1, edge.0)) {
            unique_edges.insert(edge);
            result.push(edge);
        }
    }

    result
}

pub fn vertices_with_odd_degree(adjacency_matrix: &Vec<Vec<u32>>) -> Vec<u32> {
    let mut vertices_with_odd_degree: Vec<u32> = Vec::new();
    for vertex in 0..adjacency_matrix.len() {
        let row = &adjacency_matrix[vertex as usize];
        let degree = row.iter().filter(|&value| *value != 0).count();
        if degree % 2 != 0 {vertices_with_odd_degree.push(vertex as u32)};
    }
    vertices_with_odd_degree
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
    mst
}

pub fn subgraph_with_odd_degree_vertices(vertices_with_odd_degree: &Vec<u32>, adjacency_matrix: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut subgraph: Vec<Vec<u32>> = vec![vec![0; adjacency_matrix.len()]; adjacency_matrix.len()];
    for pair in vertices_with_odd_degree.iter().combinations(2) {
        let i = *pair[0] as usize;
        let j = *pair[1] as usize;
        subgraph[i][j] = adjacency_matrix[i][j];
        subgraph[j][i] = adjacency_matrix[j][i];
    }
    subgraph
}

fn find_eulerian_cycle(adjacency_matrix: &mut Vec<Vec<u32>>) -> Vec<u32> {
    let mut stack: Vec<u32> = Vec::new();
    let mut cycle: Vec<u32> = Vec::new();
    let mut current_vertex = 0;
    stack.push(current_vertex);

    while !stack.is_empty() {
        if adjacency_matrix[current_vertex as usize].iter().any(|&v| v != 0) {
            let mut next_vertex = 0;
            for i in 0..adjacency_matrix.len() {
                if adjacency_matrix[current_vertex as usize][i] != 0 {
                    next_vertex = i as u32;
                    break;
                }
            }
            stack.push(next_vertex);
            adjacency_matrix[current_vertex as usize][next_vertex as usize] -= 1;
            adjacency_matrix[next_vertex as usize][current_vertex as usize] -= 1;
            current_vertex = next_vertex;
        } else {
            current_vertex = stack.pop().unwrap();
            cycle.push(current_vertex);
        }
    }
    cycle.reverse();
    cycle
}