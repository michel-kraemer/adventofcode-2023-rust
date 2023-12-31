use rayon::prelude::*;
use std::{
    fs::File,
    io::{stdout, BufRead, BufReader, Write},
};

enum Mode {
    Unknown,
    SeedToSoil,
    SoilToFertizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

struct Seed {
    seed: u32,
    rng: u32,
}

#[derive(Debug)]
struct Mapping {
    src: u32,
    dst: u32,
    rng: u32,
}

fn parse_map(line: &str, map: &mut Vec<Mapping>) {
    let parts: Vec<u32> = line.split(' ').map(|s| s.parse().unwrap()).collect();
    let dst = parts[0];
    let src = parts[1];
    let rng = parts[2];

    let i = map.partition_point(|e| e.src <= src);
    map.insert(i, Mapping { src, dst, rng });
}

fn map_value(map: &Vec<Mapping>, value: u32) -> u32 {
    let i = map.partition_point(|e| e.src <= value);
    if i == 0 {
        return value;
    }

    let mapping = &map[i - 1];
    let d = value - mapping.src;
    if d >= mapping.rng {
        return value;
    }

    mapping.dst + d
}

fn main() {
    for part1 in [true, false] {
        let f = File::open("input.txt").expect("Could not open file");
        let reader = BufReader::new(f);

        let mut mode = Mode::Unknown;
        let mut seeds = Vec::new();

        let mut seed_to_soil = Vec::new();
        let mut soil_to_fertilizer = Vec::new();
        let mut fertilizer_to_water = Vec::new();
        let mut water_to_light = Vec::new();
        let mut light_to_temperature = Vec::new();
        let mut temperature_to_humidity = Vec::new();
        let mut humidity_to_location = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap();
            if line.starts_with("seeds:") {
                let parts = line[7..].split(' ').collect::<Vec<_>>();
                if part1 {
                    parts.into_iter().for_each(|s| {
                        let v = s.parse::<u32>().unwrap();
                        seeds.push(Seed { seed: v, rng: 1 });
                    });
                } else {
                    parts.chunks(2).for_each(|s| {
                        seeds.push(Seed {
                            seed: s[0].parse().unwrap(),
                            rng: s[1].parse().unwrap(),
                        });
                    });
                }
            } else if line.starts_with("seed-to-soil map:") {
                mode = Mode::SeedToSoil;
            } else if line.starts_with("soil-to-fertilizer map:") {
                mode = Mode::SoilToFertizer;
            } else if line.starts_with("fertilizer-to-water map:") {
                mode = Mode::FertilizerToWater;
            } else if line.starts_with("water-to-light map:") {
                mode = Mode::WaterToLight;
            } else if line.starts_with("light-to-temperature map:") {
                mode = Mode::LightToTemperature;
            } else if line.starts_with("temperature-to-humidity map:") {
                mode = Mode::TemperatureToHumidity;
            } else if line.starts_with("humidity-to-location map:") {
                mode = Mode::HumidityToLocation;
            } else if !line.is_empty() {
                match mode {
                    Mode::Unknown => panic!("Encountered text in unknown mode"),
                    Mode::SeedToSoil => parse_map(&line, &mut seed_to_soil),
                    Mode::SoilToFertizer => parse_map(&line, &mut soil_to_fertilizer),
                    Mode::FertilizerToWater => parse_map(&line, &mut fertilizer_to_water),
                    Mode::WaterToLight => parse_map(&line, &mut water_to_light),
                    Mode::LightToTemperature => parse_map(&line, &mut light_to_temperature),
                    Mode::TemperatureToHumidity => parse_map(&line, &mut temperature_to_humidity),
                    Mode::HumidityToLocation => parse_map(&line, &mut humidity_to_location),
                }
            }
        }

        println!("Searching ...");
        let min_location = seeds
            .iter()
            .map(|s| {
                print!(".");
                stdout().flush().expect("unable to flush stdout");
                (s.seed..s.seed + s.rng)
                    .into_par_iter()
                    .map(|i| {
                        let soil = map_value(&seed_to_soil, i);
                        let fertilizer = map_value(&soil_to_fertilizer, soil);
                        let water = map_value(&fertilizer_to_water, fertilizer);
                        let light = map_value(&water_to_light, water);
                        let temperature = map_value(&light_to_temperature, light);
                        let humidity = map_value(&temperature_to_humidity, temperature);
                        map_value(&humidity_to_location, humidity)
                    })
                    .min()
                    .unwrap()
            })
            .min();
        println!();

        println!("{}", min_location.unwrap());
    }
}
