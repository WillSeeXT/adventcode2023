use std::fs::File;
use std::io::{self, BufRead, Seek};

struct SeedMap {
    src: u64,
    dst: u64,
    range: u64,
}

struct SeedsMaps {
    seed_soil_maps: Vec<SeedMap>,
    soil_fertilizer_maps: Vec<SeedMap>,
    fertilizer_water_maps: Vec<SeedMap>,
    water_light_maps: Vec<SeedMap>,
    light_temperature_maps: Vec<SeedMap>,
    temperature_humidity_maps: Vec<SeedMap>,
    humidity_location_maps: Vec<SeedMap>,
}

struct Seeds {
    seed_start: u64,
    range: u64,
}

fn main() {
    let puzzle_path = "../puzzle.txt";
    // let puzzle_path = "../puzzleTest.txt";
    let file = File::open(&puzzle_path).unwrap();

    let seeds = get_seeds(&file);
    let seed_soil_maps = get_maps(&file, "seed-to-soil map:");
    let soil_fertilizer_maps = get_maps(&file, "soil-to-fertilizer map:");
    let fertilizer_water_maps = get_maps(&file, "fertilizer-to-water map:");
    let water_light_maps = get_maps(&file, "water-to-light map:");
    let light_temperature_maps = get_maps(&file, "light-to-temperature map:");
    let temperature_humidity_maps = get_maps(&file, "temperature-to-humidity map:");
    let humidity_location_maps = get_maps(&file, "humidity-to-location map:");

    let seeds_maps = SeedsMaps {
        seed_soil_maps,
        soil_fertilizer_maps,
        fertilizer_water_maps,
        water_light_maps,
        light_temperature_maps,
        temperature_humidity_maps,
        humidity_location_maps,
    };

    let mut closest_location: u64 = find_location(seeds[0].seed_start, &seeds_maps);
    let mut closest_seed: u64 = seeds[0].seed_start;

    for seed in seeds {
        for i in 0..seed.range {
            let seed_location = find_location(seed.seed_start + i, &seeds_maps);

            if seed_location < closest_location {
                closest_location = seed_location;
                closest_seed = seed.seed_start + i;
            }
        }
    }

    println!("Closest seed: {} Location: {}", closest_seed, closest_location);
}

fn get_seeds(mut file: &File) -> Vec<Seeds> {
    let reader = io::BufReader::new(file);
    let mut seeds: Vec<Seeds> = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.starts_with("seeds: ") {
                let numbers: Vec<u64> = line
                    .split_whitespace()
                    .skip(1)
                    .filter_map(|s| s.parse().ok())
                    .collect();

                for i in (0..numbers.len()).step_by(2) {
                    let seeds_ret = Seeds {
                        seed_start: numbers[i],
                        range: numbers[i + 1],
                    };
                    seeds.push(seeds_ret);
                }
                break;
            }
        }
    }
    file.rewind().unwrap();
    seeds
}

fn get_maps(mut file: &File, start_string: &str) -> Vec<SeedMap> {
    let reader = io::BufReader::new(file);
    let mut seed_maps: Vec<SeedMap> = Vec::new();

    let mut seed_map_started = false;

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.starts_with(start_string) {
                seed_map_started = true;
                continue;
            }

            if seed_map_started {
                let numbers: Vec<u64> = line
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();

                if numbers.len() == 3 {
                    let seed_map = SeedMap {
                        src: numbers[1],
                        dst: numbers[0],
                        range: numbers[2],
                    };

                    seed_maps.push(seed_map);
                } else {
                    break; // Stop when the line is not in the expected format
                }
            }
        }
    }

    file.rewind().unwrap();
    seed_maps
}

fn find_location(seed: u64, seed_maps: &SeedsMaps) -> u64 {
    let mut location = seed;

    // println!("SEED (src) {}", seed);
    for seed_map in &seed_maps.seed_soil_maps {
        if seed >= seed_map.src && seed < seed_map.src + seed_map.range {
            location = seed_map.dst + (location - seed_map.src);
            break;
        }
    }
    // println!("  Soil (dest): {}", location);

    for seed_map in &seed_maps.soil_fertilizer_maps {
        if location >= seed_map.src && location < seed_map.src + seed_map.range {
            location = seed_map.dst + (location - seed_map.src);
            break;
        }
    }
    // println!("  Fertilizer (dest): {}",location);

    for seed_map in &seed_maps.fertilizer_water_maps {
        if location >= seed_map.src && location < seed_map.src + seed_map.range {
            location = seed_map.dst + (location - seed_map.src);
            break;
        }
    }
    // println!("  Water (dest): {}", location);

    for seed_map in &seed_maps.water_light_maps {
        if location >= seed_map.src && location < seed_map.src + seed_map.range {
            location = seed_map.dst + (location - seed_map.src);
            break;
        }
    }
    // println!("  Light (dest): {}",location);

    for seed_map in &seed_maps.light_temperature_maps {
        if location >= seed_map.src && location < seed_map.src + seed_map.range {
            location = seed_map.dst + (location - seed_map.src);
            break;
        }
    }
    // println!("  Temperature (dest): {}", location);

    for seed_map in &seed_maps.temperature_humidity_maps {
        if location >= seed_map.src && location < seed_map.src + seed_map.range {
            location = seed_map.dst + (location - seed_map.src);
            break;
        }
    }
    // println!("  Humidity (dest): {}",location);

    for seed_map in &seed_maps.humidity_location_maps {
        if location >= seed_map.src && location < seed_map.src + seed_map.range {
            location = seed_map.dst + (location - seed_map.src);
            break;
        }
    }
    // println!("  Location (dest): {}",location);

    location
}