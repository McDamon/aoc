// https://adventofcode.com/2024/day/23

use std::collections::{HashMap, HashSet};

use petgraph::{
    algo::astar,
    graph::{NodeIndex, UnGraph},
};

use crate::utils::get_lines;

struct Input {
    conn_pairs: Vec<(String, String)>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut conn_pairs: Vec<(String, String)> = vec![];

    for line in lines {
        let parts: Vec<&str> = line.split("-").collect();
        conn_pairs.push((parts[0].to_string(), parts[1].to_string()));
    }

    Input { conn_pairs }
}

fn build_conn_graph(
    conn_pairs: &Vec<(String, String)>,
) -> (UnGraph<String, ()>, HashMap<String, NodeIndex>) {
    let mut graph = UnGraph::<String, ()>::new_undirected();
    let mut node_indices = HashMap::<String, NodeIndex>::new();

    // Add nodes and edges

    // First create all the nodes
    for (node1, node2) in conn_pairs {
        if !node_indices.contains_key(node1) {
            let index = graph.add_node(node1.clone());
            node_indices.insert(node1.clone(), index);
        }

        if !node_indices.contains_key(node2) {
            let index = graph.add_node(node2.clone());
            node_indices.insert(node2.clone(), index);
        }
    }

    // Then add all edges in a separate pass
    for (node1, node2) in conn_pairs {
        let node1_index = node_indices.get(node1).unwrap();
        let node2_index = node_indices.get(node2).unwrap();
        graph.add_edge(*node1_index, *node2_index, ());
    }

    (graph, node_indices)
}

pub fn get_num_conn_start_t(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let (graph, node_indices) = build_conn_graph(&input.conn_pairs);

    let mut conn_comps = HashSet::<(String, String, String)>::new();

    for (comp1, ni1) in &node_indices {
        for (comp2, ni2) in &node_indices {
            for (comp3, ni3) in &node_indices {
                if *ni1 != *ni2 && *ni1 != *ni3 && *ni2 != *ni3 {
                    if let Some(n1_to_n2) =
                        astar(&graph, *ni1, |finish| finish == *ni2, |_| 1, |_| 0)
                        && let Some(n1_to_n3) =
                            astar(&graph, *ni1, |finish| finish == *ni3, |_| 1, |_| 0)
                        && let Some(n2_to_n3) =
                            astar(&graph, *ni2, |finish| finish == *ni3, |_| 1, |_| 0)
                    {
                        if n1_to_n2.0 == 1 && n1_to_n3.0 == 1 && n2_to_n3.0 == 1 {
                            let mut conn_comp = vec![comp1, comp2, comp3];
                            conn_comp.sort();
                            conn_comps.insert((conn_comp[0].clone(), conn_comp[1].clone(), conn_comp[2].clone()));
                        }
                    }
                }
            }
        }
    }

    let mut num_conn_start_t = 0;

    for (conn_comp1, conn_comp2, conn_comp3) in conn_comps {
        if conn_comp1.starts_with("t") || conn_comp2.starts_with("t") || conn_comp3.starts_with("t") {
            num_conn_start_t += 1;
        }
    }

    num_conn_start_t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_num_conn_start_t_test01() {
        assert_eq!(7, get_num_conn_start_t("input/2024/day23_test01.txt"));
    }

    #[test]
    fn test_get_num_conn_start_t() {
        assert_eq!(0, get_num_conn_start_t("input/2024/day23.txt"));
    }
}
