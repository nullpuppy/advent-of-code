use itertools::Itertools;

pub fn parse_line_to_map<I>(line: &mut I) -> AlmanacEntry
where
    I: Iterator<Item = String>,
{
    let name = line.next().unwrap();

    let mut entry = AlmanacEntry {
        name: name.split_whitespace().nth(0).unwrap_or_default().to_string(),
        map: vec![]
    };

    while let Some((dest_start, src_start, len)) = line
        .next()
        .unwrap_or_default()
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect_tuple()
    {
        entry.map.push(Map::new(src_start, dest_start, len));
    }


    entry
}

pub struct Almanac {
    seeds: Vec<usize>,
    seed_soil: AlmanacEntry,
    soil_fertilizer: AlmanacEntry,
    fertilizer_water: AlmanacEntry,
    water_light: AlmanacEntry,
    light_temperature: AlmanacEntry,
    temperature_humidity: AlmanacEntry,
    humidity_location: AlmanacEntry,
}

pub struct AlmanacEntry {
    name: String,
    map: Vec<Map>,
}

impl AlmanacEntry {
    /// Look in source range for value and return destination mapped value
    /// If no mapped value is found, mapped value is the same as the input, and we just return that.
    pub fn get(&self, value: usize) -> usize {
        for map in &self.map {
            if let Some(v) = map.get(value) {
                return v;
            }
        }
        value
    }

    /// Look in destination range for value and return source mapped value
    /// If no mapped value is found, mapped value is the same as the input, and we just return that.
    pub fn get_inverse(&self, value: usize) -> usize {
        for map in &self.map {
            if let Some(v) = map.get_inverse(value) {
                return v;
            }
        }
        value
    }

    /// get min value in source mapping
    pub fn get_min(&self) -> usize {
        let mut min = core::usize::MAX;
        for map in &self.map {
            if map.src_start < min {
                min = map.src_start;
            }
        }
        min
    }

    /// get min value in destination mapping
    pub fn get_min_inverse(&self) -> usize {
        let mut min = core::usize::MAX;
        for map in &self.map {
            if map.dest_start < min {
                min = map.dest_start;
            }
        }
        min
    }

    /// get max value in destination mapping
    pub fn get_max_inverse(&self) -> usize {
        let mut max = 0;
        for map in &self.map {
            if map.dest_end > max {
                max = map.dest_end;
            }
        }
        max
    }
}

impl Almanac {
    pub fn from_iter(mut input: &mut impl Iterator<Item = String>) -> Self {
        // Parse vector of seeds to be planted
        let seeds: Vec<_> = input.next()
            .unwrap() // unwrap the option
            .split_once(":")
            .unwrap()
            .1
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        input.next(); // consume empty line.

        Self {
            seeds,
            seed_soil: parse_line_to_map(&mut input),
            soil_fertilizer: parse_line_to_map(&mut input),
            fertilizer_water: parse_line_to_map(&mut input),
            water_light: parse_line_to_map(&mut input),
            light_temperature: parse_line_to_map(&mut input),
            temperature_humidity: parse_line_to_map(&mut input),
            humidity_location: parse_line_to_map(&mut input),
        }
    }

    pub fn seeds(&self) -> &Vec<usize> {
        &self.seeds
    }

    /// Look through the maps to find the location for the requested seed
    pub fn get_location_for_seed(&self, seed: usize) -> usize {
        let soil = self.seed_soil.get(seed);
        let fert = self.soil_fertilizer.get(soil);
        let water = self.fertilizer_water.get(fert);
        let light = self.water_light.get(water);
        let temp = self.light_temperature.get(light);
        let hum = self.temperature_humidity.get(temp);
        let loc = self.humidity_location.get(hum);
        // println!("Seed {seed} -> Soil: {soil} -> Fertilizer: {fert} -> Water {water} -> Light {light} -> Temperature {temp} -> Humidity {hum} -> Location {loc}");
        loc
    }

    /// Look through the maps to find the seed for the requested location, but doing reverse lookups.
    pub fn get_seed_for_location(&self, location: usize) -> usize {
        let humidity = self.humidity_location.get_inverse(location);
        let temperature = self.temperature_humidity.get_inverse(humidity);
        let light = self.light_temperature.get_inverse(temperature);
        let water = self.water_light.get_inverse(light);
        let fertilizer = self.fertilizer_water.get_inverse(water);
        let soil = self.soil_fertilizer.get_inverse(fertilizer);
        let seed = self.seed_soil.get_inverse(soil);
        // println!("Seed {seed} -> Soil: {soil} -> Fertilizer: {fert} -> Water {water} -> Light {light} -> Temperature {temp} -> Humidity {hum} -> Location {loc}");
        seed

    }

    pub fn get_min_location(&self) -> usize {
        self.humidity_location.get_min_inverse()
    }

    pub fn get_max_location(&self) -> usize {
        self.humidity_location.get_max_inverse()
    }
}

#[derive(Debug)]
pub struct Map {
    src_start: usize,
    src_end: usize,
    dest_start: usize,
    dest_end: usize,
}

/// Almanac Lookyp map. Contains src -> dest mapping for usize values.
/// Since lookups are linear, reverse lookups are also possible for dest -> src
/// using get_inverse
impl Map {
    pub fn new(src_start: usize, dest_start: usize, len: usize) -> Self {
        Self {
            src_start,
            src_end: src_start + len,
            dest_start,
            dest_end: dest_start + len,
        }
    }

    /// Look in source range for value and return destination mapped value
    /// Return None if no mapped value.
    pub fn get(&self, value: usize) -> Option<usize> {
        if value >= self.src_start && value < self.src_end {
            Some(self.dest_start + value - self.src_start)
        } else {
            None
        }
    }

    /// Look in destination range for value and return source mapped value
    /// Return None if no mapped value.
    pub fn get_inverse(&self, value: usize) -> Option<usize> {
        if value >= self.dest_start && value < self.dest_end {
            Some(self.src_start + value - self.dest_start)
        } else {
            None
        }
    }
}
