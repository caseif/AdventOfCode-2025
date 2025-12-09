use std::collections::HashMap;
use aoc2024_common::file::read_input_lines;

const DAY: u32 = 8;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Vector3u {
    x: u64,
    y: u64,
    z: u64,
}

impl Vector3u {
    fn new(x: impl Into<u64>, y: impl Into<u64>, z: impl Into<u64>) -> Self {
        Self { x: x.into(), y: y.into(), z: z.into() }
    }

    fn dist_sq(&self, other: &Self) -> f32 {
        (
            (self.x.abs_diff(other.x)) * (self.x.abs_diff(other.x)) +
            (self.y.abs_diff(other.y)) * (self.y.abs_diff(other.y)) +
            (self.z.abs_diff(other.z)) * (self.z.abs_diff(other.z))
        ) as f32
    }
}

fn main() {
    let coords = parse_input();
    println!("Part 1: {}", solve_p1(&coords));
    println!("Part 2: {}", solve_p2(&coords));
}

fn parse_input() -> Vec<Vector3u> {
    read_input_lines(DAY).iter()
        .map(|line| line.split(",").map(|n| n.parse::<u32>().unwrap()).collect::<Vec<_>>())
        .map(|spl| Vector3u::new(spl[0], spl[1], spl[2]))
        .collect()
}

fn build_circuits(coords: &Vec<Vector3u>, max_conns: usize) -> (HashMap<u32, Vec<usize>>, (Vector3u, Vector3u)) {
    let mut distances = Vec::<(usize, usize, f32)>::new();
    for (i, &coord_1) in coords.iter().enumerate() {
        for (j, &coord_2) in coords.iter().enumerate().skip(i + 1) {
            distances.push((i, j, coord_1.dist_sq(&coord_2)));
        }
    }
    distances.sort_by_key(|(_, _, dist)| (dist * 1000000.0) as u64);

    let mut next_circuit_id = 0;
    let mut circuits = HashMap::<u32, Vec<usize>>::new();
    let mut circuit_mappings = HashMap::<usize, u32>::new();
    let mut last_conn = (0, 0);
    for (i, j, _) in distances.iter().take(max_conns) {
        last_conn = (*i, *j);
        let i_circ_id = circuit_mappings.get(i).cloned();
        let j_circ_id = circuit_mappings.get(j).cloned();
        if i_circ_id.is_none() && j_circ_id.is_none() {
            circuits.insert(next_circuit_id, vec![*i, *j]);
            circuit_mappings.insert(*i, next_circuit_id);
            circuit_mappings.insert(*j, next_circuit_id);
            next_circuit_id += 1;
        } else if i_circ_id.is_some() && j_circ_id.is_some() {
            if i_circ_id.unwrap() == j_circ_id.unwrap() {
                continue;
            }
            let i_circ = circuits.get(&i_circ_id.unwrap()).unwrap().clone();
            let j_circ = circuits.get_mut(&j_circ_id.unwrap()).unwrap();
            for id in i_circ {
                j_circ.push(id);
                circuit_mappings.insert(id, j_circ_id.unwrap());
            }
            circuits.remove(&i_circ_id.unwrap());
        } else if i_circ_id.is_some() {
            circuits.get_mut(&i_circ_id.unwrap()).unwrap().push(*j);
            circuit_mappings.insert(*j, i_circ_id.unwrap());
        } else {
            circuits.get_mut(&j_circ_id.unwrap()).unwrap().push(*i);
            circuit_mappings.insert(*i, j_circ_id.unwrap());
        }

        if circuits.len() == 1 && circuits.values().map(|v| v.len()).max().unwrap() == coords.len() {
            break;
        }
    }

    return (circuits, (coords[last_conn.0], coords[last_conn.1]));
}

fn solve_p1(coords: &Vec<Vector3u>) -> u64 {
    let (circuits, _) = build_circuits(coords, 1000);

    let mut sizes: Vec<_> = circuits.values().map(|v| v.len()).collect();
    sizes.sort();

    (sizes[sizes.len() - 1] * sizes[sizes.len() - 2] * sizes[sizes.len() - 3]) as u64
}

fn solve_p2(coords: &Vec<Vector3u>) -> u64 {
    let (_, last_conn) = build_circuits(coords, usize::MAX);
    last_conn.0.x * last_conn.1.x
}
