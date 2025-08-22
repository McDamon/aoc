// https://adventofcode.com/2023/day/5

use itertools::Itertools;

use crate::utils::get_lines;

pub enum ParseStage {
    Seeds,
    SeedsToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

#[derive(Debug, Default)]
pub struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil: Vec<(u64, u64, u64)>,
    soil_to_fertilizer: Vec<(u64, u64, u64)>,
    fertilizer_to_water: Vec<(u64, u64, u64)>,
    water_to_light: Vec<(u64, u64, u64)>,
    light_to_temperature: Vec<(u64, u64, u64)>,
    temperature_to_humidity: Vec<(u64, u64, u64)>,
    humidity_to_location: Vec<(u64, u64, u64)>,
}

#[derive(Debug)]
pub struct Input {
    pub almanac: Almanac,
}

pub fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    Input {
        almanac: parse_almanac(lines),
    }
}

fn parse_almanac(lines: Vec<String>) -> Almanac {
    let mut parse_stage: ParseStage = ParseStage::Seeds;
    let mut almanac = Almanac::default();
    let mut parse_nums;
    for almanac_line in lines {
        let split_line: Vec<&str> = almanac_line.split(':').collect();
        match split_line[0] {
            "seeds" => {
                parse_stage = ParseStage::Seeds;
                let seeds: Vec<u64> = split_line[1]
                    .trim()
                    .split(' ')
                    .map(|seed| seed.parse().unwrap())
                    .collect();
                almanac.seeds = seeds;
                continue;
            }
            "seed-to-soil map" => {
                parse_stage = ParseStage::SeedsToSoil;
                continue;
            }
            "soil-to-fertilizer map" => {
                parse_stage = ParseStage::SoilToFertilizer;
                continue;
            }
            "fertilizer-to-water map" => {
                parse_stage = ParseStage::FertilizerToWater;
                continue;
            }
            "water-to-light map" => {
                parse_stage = ParseStage::WaterToLight;
                continue;
            }
            "light-to-temperature map" => {
                parse_stage = ParseStage::LightToTemperature;
                continue;
            }
            "temperature-to-humidity map" => {
                parse_stage = ParseStage::TemperatureToHumidity;
                continue;
            }
            "humidity-to-location map" => {
                parse_stage = ParseStage::HumidityToLocation;
                continue;
            }
            "" => {
                continue;
            }
            _ => {
                parse_nums = true;
            }
        }

        if parse_nums {
            let nums: (u64, u64, u64) = split_line[0]
                .trim()
                .split(' ')
                .map(|seed| seed.parse().unwrap())
                .collect_tuple()
                .unwrap();

            match parse_stage {
                ParseStage::Seeds => (),
                ParseStage::SeedsToSoil => {
                    almanac.seed_to_soil.push(nums);
                }
                ParseStage::SoilToFertilizer => {
                    almanac.soil_to_fertilizer.push(nums);
                }
                ParseStage::FertilizerToWater => {
                    almanac.fertilizer_to_water.push(nums);
                }
                ParseStage::WaterToLight => {
                    almanac.water_to_light.push(nums);
                }
                ParseStage::LightToTemperature => {
                    almanac.light_to_temperature.push(nums);
                }
                ParseStage::TemperatureToHumidity => {
                    almanac.temperature_to_humidity.push(nums);
                }
                ParseStage::HumidityToLocation => {
                    almanac.humidity_to_location.push(nums);
                }
            }
        }
    }
    almanac
}

pub fn get_lowest_location(input_file: &str, seeds_as_ranges: bool) -> u64 {
    let input = parse_input(input_file);

    let mut seeds: Vec<u64> = vec![];

    if seeds_as_ranges {
        input.almanac.seeds.chunks(2).for_each(|range| {
            let range = range[0]..(range[0] + range[1]);
            seeds.append(&mut range.collect_vec());
        });
    } else {
        seeds = input.almanac.seeds;
    }

    let locations: Vec<_> = seeds
        .iter()
        .map(|seed| {
            let soil_lookup = match get_destinations(*seed, &input.almanac.seed_to_soil) {
                Some(lookup_val) => lookup_val,
                _ => *seed,
            };

            let fertilizer_lookup =
                match get_destinations(soil_lookup, &input.almanac.soil_to_fertilizer) {
                    Some(lookup_val) => lookup_val,
                    _ => soil_lookup,
                };

            let water_lookup =
                match get_destinations(fertilizer_lookup, &input.almanac.fertilizer_to_water) {
                    Some(lookup_val) => lookup_val,
                    _ => fertilizer_lookup,
                };

            let light_lookup = match get_destinations(water_lookup, &input.almanac.water_to_light) {
                Some(lookup_val) => lookup_val,
                _ => water_lookup,
            };

            let temperature_lookup =
                match get_destinations(light_lookup, &input.almanac.light_to_temperature) {
                    Some(lookup_val) => lookup_val,
                    _ => light_lookup,
                };

            let humidity_lookup = match get_destinations(
                temperature_lookup,
                &input.almanac.temperature_to_humidity,
            ) {
                Some(lookup_val) => lookup_val,
                _ => temperature_lookup,
            };

            match get_destinations(humidity_lookup, &input.almanac.humidity_to_location) {
                Some(lookup_val) => lookup_val,
                _ => humidity_lookup,
            }
        })
        .collect();

    *locations.iter().min().unwrap()
}

fn get_destinations(lookup_val: u64, ranges: &Vec<(u64, u64, u64)>) -> Option<u64> {
    for range in ranges {
        if let Some(dest_val) = get_destination(lookup_val, range) {
            return Some(dest_val);
        }
    }
    Some(lookup_val)
}

fn get_destination(lookup_val: u64, (dest, source, length): &(u64, u64, u64)) -> Option<u64> {
    if lookup_val >= *source && lookup_val < source + length {
        let index_into_range = lookup_val - source;
        let dest_val = dest + index_into_range;
        return Some(dest_val);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_destination_test01() {
        assert_eq!(Some(50), get_destination(98, &(50u64, 98u64, 2u64)));
    }

    #[test]
    fn test_get_destination_test02() {
        assert_eq!(Some(51), get_destination(99, &(50u64, 98u64, 2u64)));
    }

    #[test]
    fn test_get_destination_test03() {
        assert_eq!(Some(55), get_destination(53, &(52u64, 50u64, 48u64)));
    }

    #[test]
    fn test_get_destinations_test01() {
        let range = vec![(50, 98, 2), (52, 50, 48)];
        assert_eq!(Some(81), get_destinations(79, &range));
    }

    #[test]
    fn test_get_destinations_test02() {
        let range = vec![(50, 98, 2), (52, 50, 48)];
        assert_eq!(Some(14), get_destinations(14, &range));
    }

    #[test]
    fn test_get_destinations_test03() {
        let range = vec![(50, 98, 2), (52, 50, 48)];
        assert_eq!(Some(57), get_destinations(55, &range));
    }

    #[test]
    fn test_get_destinations_test04() {
        let range = vec![(50, 98, 2), (52, 50, 48)];
        assert_eq!(Some(13), get_destinations(13, &range));
    }

    #[test]
    fn test_get_lowest_location_test01() {
        assert_eq!(35, get_lowest_location("input/2023/day05_test01.txt", false));
    }

    #[test]
    fn test_get_lowest_location_test02() {
        assert_eq!(13, get_lowest_location("input/2023/day05_test02.txt", false));
    }

    #[test]
    fn test_get_lowest_location_score() {
        assert_eq!(388071289, get_lowest_location("input/2023/day05.txt", false));
    }

    #[test]
    fn test_get_lowest_location_seeds_as_ranges_test01() {
        assert_eq!(46, get_lowest_location("input/2023/day05_test01.txt", true));
    }

    #[test]
    fn test_get_lowest_location_seeds_as_ranges() {
        assert_eq!(84206669, get_lowest_location("input/2023/day05.txt", true));
    }
}
