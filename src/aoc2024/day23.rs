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

pub fn get_num_conn_start_t(input_file: &str) -> usize {
    let input = parse_input(input_file);

    let (graph, node_indices) = build_conn_graph(&input.conn_pairs);

    let mut conn_comp_pairs = HashSet::<(String, String)>::new();
    let mut conn_comp_tuples = HashSet::<(String, String, String)>::new();

    for (comp1, ni1) in &node_indices {
        for (comp2, ni2) in &node_indices {
            let mut conn_comp = [comp1, comp2];
            conn_comp.sort();

            let conn_comp_tup = (conn_comp[0].clone(), conn_comp[1].clone());

            if *ni1 != *ni2 && let Some(n1_to_n2) = astar(&graph, *ni1, |finish| finish == *ni2, |_| 1, |_| 0) && n1_to_n2.0 == 1 {
                conn_comp_pairs.insert(conn_comp_tup);
            }
        }
    }

    for (comp1, ni1) in &node_indices {
        for (comp2, ni2) in &node_indices {
            let mut conn_comp12 = [comp1, comp2];
            conn_comp12.sort();

            let conn_comp_pair12 = (conn_comp12[0].clone(), conn_comp12[1].clone());

            if conn_comp_pairs.contains(&conn_comp_pair12) {
                for (comp3, ni3) in &node_indices {
                    if *ni1 != *ni2 && *ni1 != *ni3 && *ni2 != *ni3 {
                        let mut conn_comp13 = [comp1, comp3];
                        conn_comp13.sort();

                        let mut conn_comp23 = [comp2, comp3];
                        conn_comp23.sort();

                        let conn_comp_pair13 = (conn_comp13[0].clone(), conn_comp13[1].clone());

                        let conn_comp_pair22 = (conn_comp23[0].clone(), conn_comp23[1].clone());

                        if conn_comp_pairs.contains(&conn_comp_pair13) && conn_comp_pairs.contains(&conn_comp_pair22) && (comp1.starts_with("t")
                                || comp2.starts_with("t") || comp3.starts_with("t")) {
                            let mut conn_comp123 = [comp1, comp2, comp3];
                            conn_comp123.sort();

                            let conn_comp_tuple123 = (
                                conn_comp123[0].clone(),
                                conn_comp123[1].clone(),
                                conn_comp123[2].clone(),
                            );

                            conn_comp_tuples.insert(conn_comp_tuple123);
                        }
                    }
                }
            }
        }
    }

    conn_comp_tuples.len()
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
        assert_eq!(1327, get_num_conn_start_t("input/2024/day23.txt"));
    }
}
