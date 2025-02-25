// https://adventofcode.com/2024/day/23

use std::collections::HashMap;

use petgraph::{
    algo::kosaraju_scc,
    dot::Dot,
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

    println!("graph: {:?}", Dot::new(&graph));

    let conn_comps = kosaraju_scc(&graph);

    for conn_comp in conn_comps {
        println!("conn_comp: {:?}", conn_comp);
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_num_conn_start_t_test01() {
        assert_eq!(0, get_num_conn_start_t("input/2024/day23_test01.txt"));
    }

    #[test]
    fn test_get_num_conn_start_t() {
        assert_eq!(0, get_num_conn_start_t("input/2024/day23.txt"));
    }
}
