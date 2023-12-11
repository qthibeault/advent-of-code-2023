use std::collections::HashSet;
use std::hash::Hash;

use super::{One, Two};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Seed(u64);

impl From<u64> for Seed {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl Into<u64> for Seed {
    fn into(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Soil(u64);

impl Into<u64> for Soil {
    fn into(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fertilizer(u64);

impl Into<u64> for Fertilizer {
    fn into(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Water(u64);

impl Into<u64> for Water {
    fn into(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Light(u64);

impl Into<u64> for Light {
    fn into(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Temperature(u64);

impl Into<u64> for Temperature {
    fn into(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Humidity(u64);

impl Into<u64> for Humidity {
    fn into(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Location(u64);

impl Into<u64> for Location {
    fn into(self) -> u64 {
        self.0
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Range {
    src_start: u64,
    dest_start: u64,
    length: u64,
}

impl Range {
    fn contains(&self, n: u64) -> bool {
        let start = self.src_start;
        let end = start + self.length;

        start <= n && n < end
    }

    fn convert(&self, n: u64) -> Option<u64> {
        if self.contains(n) {
            Some((n - self.src_start) + self.dest_start)
        } else {
            None
        }
    }
}

impl From<&str> for Range {
    fn from(input: &str) -> Self {
        let values: Vec<u64> = input
            .split(" ")
            .map(|s| u64::from_str_radix(s, 10).expect("Could not parse range number"))
            .collect();

        assert_eq!(values.len(), 3);

        Self {
            src_start: values[1],
            dest_start: values[0],
            length: values[2],
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Map {
    ranges: HashSet<Range>,
}

impl Map {
    fn convert(&self, n: u64) -> u64 {
        self.ranges.iter().find_map(|range| range.convert(n)).unwrap_or(n)
    }
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let ranges = input
            .lines()
            .skip(1)
            .map(Range::from)
            .collect();

        Self { ranges }
    }
}

impl<const N: usize> From<[Range; N]> for Map {
    fn from(ranges: [Range; N]) -> Self {
        Self {
            ranges: HashSet::from(ranges),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct SeedSoilMap(Map);

impl SeedSoilMap {
    #[inline]
    fn convert(&self, seed: Seed) -> Soil {
        Soil(self.0.convert(seed.0))
    }
}

impl<const N: usize> From<[Range; N]> for SeedSoilMap {
    fn from(ranges: [Range; N]) -> Self {
        Self(Map::from(ranges))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct SoilFertilizerMap(Map);

impl SoilFertilizerMap {
    #[inline]
    fn convert(&self, soil: Soil) -> Fertilizer {
        Fertilizer(self.0.convert(soil.0))
    }
}

impl<const N: usize> From<[Range; N]> for SoilFertilizerMap {
    fn from(ranges: [Range; N]) -> Self {
        Self(Map::from(ranges))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct FertilizerWaterMap(Map);

impl FertilizerWaterMap {
    #[inline]
    fn convert(&self, fertilizer: Fertilizer) -> Water {
        Water(self.0.convert(fertilizer.0))
    }
}

impl<const N: usize> From<[Range; N]> for FertilizerWaterMap {
    fn from(ranges: [Range; N]) -> Self {
        Self(Map::from(ranges))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct WaterLightMap(Map);

impl WaterLightMap {
    #[inline]
    fn convert(&self, water: Water) -> Light {
        Light(self.0.convert(water.0))
    }
}

impl<const N: usize> From<[Range; N]> for WaterLightMap {
    fn from(ranges: [Range; N]) -> Self {
        Self(Map::from(ranges))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct LightTemperatureMap(Map);

impl LightTemperatureMap {
    #[inline]
    fn convert(&self, light: Light) -> Temperature {
        Temperature(self.0.convert(light.0))
    }
}

impl<const N: usize> From<[Range; N]> for LightTemperatureMap {
    fn from(ranges: [Range; N]) -> Self {
        Self(Map::from(ranges))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct TemperatureHumidityMap(Map);

impl TemperatureHumidityMap {
    #[inline]
    fn convert(&self, temperature: Temperature) -> Humidity {
        Humidity(self.0.convert(temperature.0))
    }
}

impl<const N: usize> From<[Range; N]> for TemperatureHumidityMap {
    fn from(ranges: [Range; N]) -> Self {
        Self(Map::from(ranges))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct HumidityLocationMap(Map);

impl HumidityLocationMap {
    #[inline]
    fn convert(&self, humidity: Humidity) -> Location {
        Location(self.0.convert(humidity.0))
    }
}

impl<const N: usize> From<[Range; N]> for HumidityLocationMap {
    fn from(ranges: [Range; N]) -> Self {
        Self(Map::from(ranges))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Almanac<T>
where
    T: Eq + Hash,
{
    seeds: HashSet<T>,
    seed_soil: SeedSoilMap,
    soil_fertilizer: SoilFertilizerMap,
    fertilizer_water: FertilizerWaterMap,
    water_light: WaterLightMap,
    light_temperature: LightTemperatureMap,
    temperature_humidity: TemperatureHumidityMap,
    humidity_location: HumidityLocationMap,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Conversion {
    pub seed: Seed,
    pub soil: Soil,
    pub fertilizer: Fertilizer,
    pub water: Water,
    pub light: Light,
    pub temperature: Temperature,
    pub humidity: Humidity,
    pub location: Location,
}

impl<T> Almanac<T>
where
    T: Eq + Hash,
{
    fn convert(&self, seed: Seed) -> Conversion {
        let soil = self.seed_soil.convert(seed);
        let fertilizer = self.soil_fertilizer.convert(soil);
        let water = self.fertilizer_water.convert(fertilizer);
        let light = self.water_light.convert(water);
        let temperature = self.light_temperature.convert(light);
        let humidity = self.temperature_humidity.convert(temperature);
        let location = self.humidity_location.convert(humidity);

        Conversion {
            seed,
            soil,
            fertilizer,
            water,
            light,
            temperature,
            humidity,
            location,
        }
    }
}

impl Almanac<Seed> {
    pub fn conversions(&self) -> impl Iterator<Item = Conversion> + '_ {
        self.seeds
            .iter()
            .map(|seed| self.convert(*seed))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SeedRange {
    start: u64,
    end: u64,
}

impl SeedRange {
    fn seeds(&self) -> impl Iterator<Item = Seed> {
        (self.start..=self.end)
            .into_iter()
            .map(|n| Seed(n))
    }
}

impl Almanac<SeedRange> {
    pub fn conversions(&self) -> impl Iterator<Item = Conversion> + '_ {
        self.seeds
            .iter()
            .map(|r| r.seeds())
            .flatten()
            .map(|s| self.convert(s))
    }
}

fn parse_seeds(block: &str) -> HashSet<Seed> {
    block
        .split(" ")
        .skip(1)
        .map(|s| Seed(u64::from_str_radix(s, 10).expect("Could not parse seed")))
        .collect()
}

impl<'a> From<One<'a>> for Almanac<Seed> {
    fn from(input: One<'a>) -> Self {
        let blocks: Vec<&str> = input.0.split("\n\n").collect();

        assert_eq!(blocks.len(), 8);

        Self {
            seeds: parse_seeds(blocks[0]) ,
            seed_soil: SeedSoilMap(Map::from(blocks[1])),
            soil_fertilizer: SoilFertilizerMap(Map::from(blocks[2])),
            fertilizer_water: FertilizerWaterMap(Map::from(blocks[3])),
            water_light: WaterLightMap(Map::from(blocks[4])),
            light_temperature: LightTemperatureMap(Map::from(blocks[5])),
            temperature_humidity: TemperatureHumidityMap(Map::from(blocks[6])),
            humidity_location: HumidityLocationMap(Map::from(blocks[7])),
        }
    }
}

fn parse_seed_ranges(block: &str) -> HashSet<SeedRange> {
    let mut values = block
        .split_whitespace()
        .skip(1)
        .map(|s| u64::from_str_radix(s, 10).expect("Could not parse seed"));

    let mut ranges = HashSet::new();

    while let Some(start) = values.next() {
        let length = values.next().expect("Must be even number of values to produce seed ranges");
        let end = start + length - 1;

        ranges.insert(SeedRange { start, end });
    }

    ranges
}


impl<'a> From<Two<'a>> for Almanac<SeedRange> {
    fn from(input: Two<'a>) -> Self {
        let blocks: Vec<&str> = input.0.split("\n\n").collect();

        assert_eq!(blocks.len(), 8);

        Self {
            seeds: parse_seed_ranges(blocks[0]) ,
            seed_soil: SeedSoilMap(Map::from(blocks[1])),
            soil_fertilizer: SoilFertilizerMap(Map::from(blocks[2])),
            fertilizer_water: FertilizerWaterMap(Map::from(blocks[3])),
            water_light: WaterLightMap(Map::from(blocks[4])),
            light_temperature: LightTemperatureMap(Map::from(blocks[5])),
            temperature_humidity: TemperatureHumidityMap(Map::from(blocks[6])),
            humidity_location: HumidityLocationMap(Map::from(blocks[7])),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, hash::Hash};

    use indoc::indoc;

    use super::{
        One,
        Two,
        Almanac,
        Seed,
        Range,
        SeedSoilMap,
        SoilFertilizerMap,
        FertilizerWaterMap,
        WaterLightMap,
        LightTemperatureMap,
        TemperatureHumidityMap,
        HumidityLocationMap,
        Conversion, SeedRange,
    };

    fn almanac<T, const N: usize>(seeds: [T; N]) -> Almanac<T>
    where
        T: Eq + Hash,
    {
        Almanac {
            seeds: HashSet::from(seeds),
            seed_soil: SeedSoilMap::from([
                Range { dest_start: 50, src_start: 98, length: 2 },
                Range { dest_start: 52, src_start: 50, length: 48 },
            ]),
            soil_fertilizer: SoilFertilizerMap::from([
                Range { dest_start: 0, src_start: 15, length: 37 },
                Range { dest_start: 37, src_start: 52, length: 2 },
                Range { dest_start: 39, src_start: 0, length: 15 },
            ]),
            fertilizer_water: FertilizerWaterMap::from([
                Range { dest_start: 49, src_start: 53, length: 8 },
                Range { dest_start: 0, src_start: 11, length: 42 },
                Range { dest_start: 42, src_start: 0, length: 7 },
                Range { dest_start: 57, src_start: 7, length: 4 },
            ]),
            water_light: WaterLightMap::from([
                Range { dest_start: 88, src_start: 18, length: 7 },
                Range { dest_start: 18, src_start: 25, length: 70 },
            ]),
            light_temperature: LightTemperatureMap::from([
                Range { dest_start: 45, src_start: 77, length: 23 },
                Range { dest_start: 81, src_start: 45, length: 19 },
                Range { dest_start: 68, src_start: 64, length: 13 },
            ]),
            temperature_humidity: TemperatureHumidityMap::from([
                Range { dest_start: 0, src_start: 69, length: 1 },
                Range { dest_start: 1, src_start: 0, length: 69 },
            ]),
            humidity_location: HumidityLocationMap::from([
                Range { dest_start: 60, src_start: 56, length: 37 },
                Range { dest_start: 56, src_start: 93, length: 4 },
            ]),
        }
    }

    fn seed_almanac() -> Almanac<Seed> {
        almanac([Seed(79), Seed(14), Seed(55), Seed(13)])
    }

    fn seed_range_almanac() -> Almanac<SeedRange> {
        almanac([SeedRange { start: 79, end: 92 }, SeedRange { start: 55, end: 67 }])
    }

    static INPUT: &str = indoc!{"
        seeds: 79 14 55 13

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
        56 93 4
    "};

    #[test]
    fn parsing() {
        assert_eq!(Almanac::from(One(INPUT)), seed_almanac());
        assert_eq!(Almanac::from(Two(INPUT)), seed_range_almanac());
    }

    #[test]
    fn conversion() {
        let expected = HashSet::from([
            Conversion {
                seed: Seed(79),
                soil: super::Soil(81),
                fertilizer: super::Fertilizer(81),
                water: super::Water(81),
                light: super::Light(74),
                temperature: super::Temperature(78),
                humidity: super::Humidity(78),
                location: super::Location(82)
            },
            Conversion {
                seed: Seed(14),
                soil: super::Soil(14),
                fertilizer: super::Fertilizer(53),
                water: super::Water(49),
                light: super::Light(42),
                temperature: super::Temperature(42),
                humidity: super::Humidity(43),
                location: super::Location(43)
            },
            Conversion {
                seed: Seed(55),
                soil: super::Soil(57),
                fertilizer: super::Fertilizer(57),
                water: super::Water(53),
                light: super::Light(46),
                temperature: super::Temperature(82),
                humidity: super::Humidity(82),
                location: super::Location(86)
            },
            Conversion {
                seed: Seed(13),
                soil: super::Soil(13),
                fertilizer: super::Fertilizer(52),
                water: super::Water(41),
                light: super::Light(34),
                temperature: super::Temperature(34),
                humidity: super::Humidity(35),
                location: super::Location(35)
            },
        ]);

        let conversions = seed_almanac()
            .conversions()
            .collect();

        assert_eq!(expected, conversions);
    }
}
