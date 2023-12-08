fn main() {
    let input = include_str!("input-1.txt");
    let output = solve(input);
    dbg!(output);
}

struct Range {
    source_start: usize,
    destination_start: usize,
    length: usize,
}

struct RangeMap {
    ranges: Vec<Range>,
}

impl RangeMap {
    fn get(&self, index: usize) -> usize {
        for r in self.ranges.iter() {
            if (r.source_start..r.source_start+r.length).contains(&index) {
                let distance = index - r.source_start;
                return r.destination_start + distance;
            }
        }
        index
    }
}

fn solve(input: &str) -> usize {
    let mut it = input.split("\n\n");
    let seeds: Vec<usize> = it
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();
    let seed_to_soil = parse_range_map(it.next().unwrap());
    let soil_to_fertilizer = parse_range_map(it.next().unwrap());
    let fertilizer_to_water = parse_range_map(it.next().unwrap());
    let water_to_light = parse_range_map(it.next().unwrap());
    let light_to_temperature = parse_range_map(it.next().unwrap());
    let temperature_to_humidity = parse_range_map(it.next().unwrap());
    let humidity_to_location = parse_range_map(it.next().unwrap());
    seeds.iter().map(|&seed| {
        let soil = seed_to_soil.get(seed);
        let fertilizer = soil_to_fertilizer.get(soil);
        let water = fertilizer_to_water.get(fertilizer);
        let light = water_to_light.get(water);
        let temperature = light_to_temperature.get(light);
        let humidity = temperature_to_humidity.get(temperature);
        humidity_to_location.get(humidity)
    }).min().unwrap()
}

fn parse_range_map(input: &str) -> RangeMap {
    let mut lines = input.lines();
    // skip map name
    lines.next();
    RangeMap {
        ranges: lines
            .map(|line| {
                let mut it = line.split(' ');
                Range {
                    destination_start: it.next().unwrap().parse().unwrap(),
                    source_start: it.next().unwrap().parse().unwrap(),
                    length: it.next().unwrap().parse().unwrap(),
                }
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let output = solve(input);
        assert_eq!(output, 35);
    }
}
