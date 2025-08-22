// https://adventofcode.com/2019/day/6

use hashbrown::HashSet;
use petgraph::{Graph, algo};

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

pub fn get_total_orbits(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let mut graph = Graph::<String, u32>::new();

    for object in &input.objects {
        graph.add_node(object.to_string());
    }

    let mut maybe_com_index = None;

    for orbit in input.orbits {
        let maybe_body_index = graph.node_indices().find(|i| graph[*i] == orbit.body);
        let maybe_satellite_index = graph.node_indices().find(|i| graph[*i] == orbit.satellite);
        if let Some(body) = maybe_body_index
            && let Some(satellite) = maybe_satellite_index
        {
            if orbit.body == "COM" {
                maybe_com_index = Some(body);
            }

            graph.add_edge(body, satellite, 1); // Assign a weight of 1 for each edge
        }
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_orbits_test01() {
        assert_eq!(42, get_total_orbits("input/2019/day06_test01.txt"));
    }

    #[test]
    fn test_get_total_orbits() {
        assert_eq!(273985, get_total_orbits("input/2019/day06.txt"));
    }
}
