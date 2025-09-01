// https://adventofcode.com/2019/day/6

use hashbrown::HashSet;
use petgraph::{algo, graph::UnGraph};

use crate::utils::get_lines;

struct Orbit {
    body: String,
    satellite: String,
}

struct Input {
    objects: HashSet<String>,
    orbits: Vec<Orbit>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut objects: HashSet<String> = HashSet::new();
    let mut orbits: Vec<Orbit> = Vec::new();

    for line in lines {
        let splits: Vec<&str> = line.split(')').collect();
        let body: String = splits.first().map_or("", |v| v).to_string();
        let satellite = splits.last().map_or("", |v| v).to_string();

        objects.insert(body.clone());
        objects.insert(satellite.clone());

        orbits.push(Orbit { body, satellite });
    }

    Input { objects, orbits }
}

fn build_graph(
    input: &Input,
) -> (
    UnGraph<String, u32>,
    Option<petgraph::graph::NodeIndex>,
    Option<petgraph::graph::NodeIndex>,
    Option<petgraph::graph::NodeIndex>,
) {
    let mut graph = UnGraph::new_undirected();

    for object in &input.objects {
        graph.add_node(object.to_string());
    }

    let mut maybe_com_index = None;
    let mut maybe_san_index = None;
    let mut maybe_you_index = None;

    for orbit in &input.orbits {
        let maybe_body_index = graph.node_indices().find(|i| graph[*i] == orbit.body);
        let maybe_satellite_index = graph.node_indices().find(|i| graph[*i] == orbit.satellite);
        if let Some(body) = maybe_body_index
            && let Some(satellite) = maybe_satellite_index
        {
            if orbit.body == "COM" {
                maybe_com_index = Some(body);
            }
            if orbit.satellite == "SAN" {
                maybe_san_index = Some(body);
            }
            if orbit.satellite == "YOU" {
                maybe_you_index = Some(body);
            }

            graph.add_edge(body, satellite, 1); // Assign a weight of 1 for each edge
        }
    }

    (graph, maybe_com_index, maybe_san_index, maybe_you_index)
}

pub fn get_total_orbits(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let (graph, maybe_com_index, _, _) = build_graph(&input);

    let mut total_orbits = 0;

    for object in input.objects {
        let maybe_object_index = graph.node_indices().find(|i| graph[*i] == object);
        if let Some(object_index) = maybe_object_index
            && let Some(com_index) = maybe_com_index
            && let Some((distance, _path)) = algo::astar(
                &graph,
                com_index,
                |finish| finish == object_index,
                |e| *e.weight(), // Use the edge weight for the cost calculation
                |_| 0,
            )
        {
            total_orbits += distance;
        }
    }

    total_orbits
}

pub fn get_total_orbital_transfers(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let (graph, _, maybe_san_index, maybe_you_index) = build_graph(&input);

    let mut total_orbit_transfers = 0;
    if let Some(san_index) = maybe_san_index
        && let Some(you_index) = maybe_you_index
        && let Some((distance, _path)) = algo::astar(
            &graph,
            san_index,
            |finish| finish == you_index,
            |e| *e.weight(), // Use the edge weight for the cost calculation
            |_| 0,
        )
    {
        total_orbit_transfers += distance;
    }

    total_orbit_transfers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_orbits_test01() {
        assert_eq!(42, get_total_orbits("input/2019/day06_test01.txt"));
    }

    #[ignore]
    #[test]
    fn test_get_total_orbits() {
        assert_eq!(273985, get_total_orbits("input/2019/day06.txt"));
    }

    #[test]
    fn test_get_total_orbital_transfers_test02() {
        assert_eq!(
            4,
            get_total_orbital_transfers("input/2019/day06_test02.txt")
        );
    }

    #[ignore]
    #[test]
    fn test_get_total_orbital_transfers() {
        assert_eq!(460, get_total_orbital_transfers("input/2019/day06.txt"));
    }
}
