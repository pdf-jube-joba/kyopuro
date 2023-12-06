use std::{cmp::Ordering, ops::RangeInclusive};

fn has_intersection(range2: RangeInclusive<usize>, range1: RangeInclusive<usize>) -> bool {
    !(range1.end() < range2.start() || range2.end() < range1.start())
}

fn range_cmp(range1: &RangeInclusive<usize>, range2: &RangeInclusive<usize>) -> Ordering {
    range1
        .start()
        .cmp(range2.start())
        .then(range1.end().cmp(&range2.end()))
}

// input ([l, r], d) return [l+d, r+d]
fn change(range: RangeInclusive<usize>, u: isize) -> RangeInclusive<usize> {
    let start = (*range.start() as isize + u) as usize;
    let end = (*range.end() as isize + u) as usize;
    start..=end
}

fn unite(mut ranges: Vec<RangeInclusive<usize>>) -> Vec<RangeInclusive<usize>> {
    if ranges.is_empty() {
        return vec![];
    }
    ranges.sort_by(|range1, range2| {
        range1
            .start()
            .cmp(range2.start())
            .then(range1.end().cmp(range2.end()))
    });

    let mut v = vec![];

    let (mut li, mut ri) = (*ranges[0].start(), *ranges[0].end());

    for i in 1..ranges.len() {
        if ri + 1 < *ranges[i].start() {
            v.push(li..=ri);
            (li, ri) = (*ranges[i].start(), *ranges[i].end());
        } else {
            ri = std::cmp::max(ri, *ranges[i].end());
        }
    }

    v.push(li..=ri);

    v
}

// ranges = [[l0, r0], [l1, r1], ..., [l{n-1}, r{n-1}]], range = [l, r]
fn enumerate_intersecting_range(
    ranges: &[RangeInclusive<usize>],
    range: &RangeInclusive<usize>,
) -> Option<RangeInclusive<usize>> {
    let n = ranges.len();
    // debug assertion
    for i in 0..n - 1 {
        debug_assert!(ranges[i].end() < ranges[i + 1].start());
    }

    // min { 0 <= i < n | l <= ri }
    let le_st_min: Option<usize> = {
        let is_ok = |i: usize| range.start() <= ranges[i].end();
        if is_ok(0) {
            Some(0)
        } else if !is_ok(n - 1) {
            None
        } else {
            let mut ok = n - 1;
            let mut ng = 0;
            while ok - ng > 1 {
                let mid = (ok + ng) / 2;
                if is_ok(mid) {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            Some(ok)
        }
    };
    // min <= i <=> l <= ri

    // max { 0 <= i < n | li <= r}
    let ge_end_max: Option<usize> = {
        let is_ok = |i: usize| ranges[i].start() <= range.end();
        if !is_ok(0) {
            None
        } else if is_ok(n - 1) {
            Some(n - 1)
        } else {
            let mut ok = 0;
            let mut ng = n - 1;
            while ng - ok > 1 {
                let mid = (ok + ng) / 2;
                if is_ok(mid) {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            Some(ok)
        }
    };
    // i <= max <=> li <= r

    // [lk, rk] \cap [l, r] != emptyset <=> li <= r && l <= ri <=> i <= max && min <= i
    match (le_st_min, ge_end_max) {
        (Some(min), Some(max)) => {
            // there exists possibility of max < min ( lmax < rmax < l < r < lmin < rmin)
            if min <= max {
                Some(min..=max)
            } else {
                None
            }
        }
        (None, Some(_)) | (Some(_), None) => None,
        (None, None) => unreachable!(),
    }
}

// contains ([r..=l], d) : [r..=l] => [r+d..=l+d]
struct OneMap {
    v: Vec<(RangeInclusive<usize>, isize)>,
}

impl OneMap {
    fn new(mut v: &Vec<(usize, usize, usize)>) -> Self {
        let mut v = v
            .into_iter()
            .map(|&(v0, v1, v2)| ((v1..=v1 + v2 - 1), v0 as isize - v1 as isize))
            .collect::<Vec<_>>();
        v.sort_by_key(|(range, _)| *range.start());
        // no intersection between range
        // i < j <=> v[i].start() < v[j].start() <=> v[i].end() < v[j].end()
        Self { v }
    }
    fn num_map(&self, from: usize) -> usize {
        let is_less = |i: usize| {
            if i == self.v.len() {
                return false;
            }
            *self.v[i].0.start() <= from
        };
        let mut less_eq: usize = {
            if is_less(0) {
                0
            } else {
                return from;
            }
        };
        let mut greater: usize = self.v.len();
        while greater - less_eq > 1 {
            let mid = (greater + less_eq) / 2;
            if is_less(mid) {
                less_eq = mid;
            } else {
                greater = mid;
            }
        }
        if self.v[less_eq].0.contains(&from) {
            (from as isize + self.v[less_eq].1) as usize
        } else {
            from
        }
    }
    fn range_map(&self, range: RangeInclusive<usize>) -> Vec<RangeInclusive<usize>> {
        // 1. find all i such that self.v[i].0 is intersecting with range
        let ranges = self
            .v
            .iter()
            .map(|(range, _)| range.clone())
            .collect::<Vec<_>>();
        let Some(range_of_intersect) = enumerate_intersecting_range(&ranges, &range) else {
            return vec![range];
        };
        let (s, t) = (*range_of_intersect.start(), *range_of_intersect.end());
        // let range = s..=t
        // ....[l.............................r]....
        // ..[ls..rs]......[li..ri].......[lt..rt]..
        //
        // ....[....]......[......].......[....]
        // ..........[....]........[.....]..........

        let mut v = vec![];

        let (l, r) = (*range.start(), *range.end());

        let ls = *ranges[s].start();
        if l < ls {
            v.push(l + 1..=ls);
        }

        // dbg!(&v);

        for i in s..=t {
            let (li, ri) = (*self.v[i].0.start(), *self.v[i].0.end());
            let li = if li <= l { l } else { li };
            let ri = if r <= ri { r } else { ri };
            v.push(change(li..=ri, self.v[i].1));
        }

        // dbg!(&v);

        for i in s..t {
            let l = self.v[i].0.end() + 1;
            let r = self.v[i + 1].0.start() - 1;
            if l <= r {
                v.push(l..=r);
            }
        }

        // dbg!(&v);

        let rt = *ranges[t].end();
        if rt < r {
            v.push(rt + 1..=r);
        }

        // dbg!(&v);

        unite(v)
    }
    fn ranges_map(&self, ranges: Vec<RangeInclusive<usize>>) -> Vec<RangeInclusive<usize>> {
        unite(ranges.into_iter().flat_map(|range| {
            self.range_map(range)
        }).collect::<Vec<_>>())
    }
}

struct AlmanacPart1 {
    seeds: Vec<usize>,
    seed_to_soil: OneMap,
    soil_to_fertilizer: OneMap,
    fertilizer_to_water: OneMap,
    water_to_light: OneMap,
    light_to_temperature: OneMap,
    temperature_to_humidity: OneMap,
    humidity_to_location: OneMap,
}

impl AlmanacPart1 {
    fn from_input(almanac: &AlmanacInput) -> Self {
        let AlmanacInput {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        } = almanac;
        Self {
            seeds: seeds.to_owned(),
            seed_to_soil: OneMap::new(seed_to_soil),
            soil_to_fertilizer: OneMap::new(soil_to_fertilizer),
            fertilizer_to_water: OneMap::new(fertilizer_to_water),
            water_to_light: OneMap::new(water_to_light),
            light_to_temperature: OneMap::new(light_to_temperature),
            temperature_to_humidity: OneMap::new(temperature_to_humidity),
            humidity_to_location: OneMap::new(humidity_to_location),
        }
    }
}

fn compute_part1(almanac: AlmanacPart1) -> usize {
    almanac
        .seeds
        .iter()
        .map(|&seed| {
            let soil = almanac.seed_to_soil.num_map(seed);
            let fertilizer = almanac.soil_to_fertilizer.num_map(soil);
            let water = almanac.fertilizer_to_water.num_map(fertilizer);
            let light = almanac.water_to_light.num_map(water);
            let temperature = almanac.light_to_temperature.num_map(light);
            let humidity = almanac.temperature_to_humidity.num_map(temperature);
            let location = almanac.humidity_to_location.num_map(humidity);
            location
        })
        .min()
        .unwrap()
}

struct AlmanacPart2 {
    seeds: Vec<RangeInclusive<usize>>,
    seed_to_soil: OneMap,
    soil_to_fertilizer: OneMap,
    fertilizer_to_water: OneMap,
    water_to_light: OneMap,
    light_to_temperature: OneMap,
    temperature_to_humidity: OneMap,
    humidity_to_location: OneMap,
}

impl AlmanacPart2 {
    fn from_input(almanac: &AlmanacInput) -> AlmanacPart2 {
        let AlmanacInput {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        } = almanac;
        let seeds = {
            let mut v: Vec<RangeInclusive<usize>> = vec![];
            for i in 0..seeds.len() / 2 {
                let range = seeds[2 * i]..=seeds[2 * i] + seeds[2 * i + 1] - 1;
                v.push(range);
            }
            v
        };
        Self {
            seeds,
            seed_to_soil: OneMap::new(seed_to_soil),
            soil_to_fertilizer: OneMap::new(soil_to_fertilizer),
            fertilizer_to_water: OneMap::new(fertilizer_to_water),
            water_to_light: OneMap::new(water_to_light),
            light_to_temperature: OneMap::new(light_to_temperature),
            temperature_to_humidity: OneMap::new(temperature_to_humidity),
            humidity_to_location: OneMap::new(humidity_to_location),
        }
    }
}

fn compute_part2(almanac: AlmanacPart2) -> usize {
    let AlmanacPart2 {
        seeds: ranges,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    } = almanac;

    if ranges.is_empty() {
        panic!()
    }

    let ranges = seed_to_soil.ranges_map(ranges);
    let ranges = soil_to_fertilizer.ranges_map(ranges);
    let ranges = fertilizer_to_water.ranges_map(ranges);
    let ranges = water_to_light.ranges_map(ranges);
    let ranges = light_to_temperature.ranges_map(ranges);
    let ranges = temperature_to_humidity.ranges_map(ranges);
    let ranges = humidity_to_location.ranges_map(ranges);
    
    *ranges[0].start()
}

fn main() {
    let almanac = input();
    println!("{}", compute_part1(AlmanacPart1::from_input(&almanac)));
    println!("{}", compute_part2(AlmanacPart2::from_input(&almanac)));
}

struct AlmanacInput {
    seeds: Vec<usize>,
    seed_to_soil: Vec<(usize, usize, usize)>,
    soil_to_fertilizer: Vec<(usize, usize, usize)>,
    fertilizer_to_water: Vec<(usize, usize, usize)>,
    water_to_light: Vec<(usize, usize, usize)>,
    light_to_temperature: Vec<(usize, usize, usize)>,
    temperature_to_humidity: Vec<(usize, usize, usize)>,
    humidity_to_location: Vec<(usize, usize, usize)>,
}

fn input() -> AlmanacInput {
    let string = std::fs::read_to_string("inout/day5.in").unwrap();
    let mut lines = string.lines();
    let seeds: Vec<usize> = {
        let line = lines.next().unwrap();
        line[6..]
            .split_whitespace()
            .map(|str| str.parse::<usize>().unwrap())
            .collect()
    };

    let _ = lines.next().unwrap();

    let mut take_until_empty = move |first_line_assert: &str| {
        assert_eq!(lines.next().unwrap(), first_line_assert);
        let mut v = vec![];
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            let [v0, v1, v2, ..] = line.split_whitespace().map(|str| str.parse::<usize>().unwrap()).collect::<Vec<_>>()[..] else {
                unreachable!()
            };
            v.push((v0, v1, v2));
        }
        v
    };

    let seed_to_soil: Vec<(usize, usize, usize)> = take_until_empty("seed-to-soil map:");

    let soil_to_fertilizer: Vec<(usize, usize, usize)> =
        take_until_empty("soil-to-fertilizer map:");

    let fertilizer_to_water: Vec<(usize, usize, usize)> =
        take_until_empty("fertilizer-to-water map:");

    let water_to_light: Vec<(usize, usize, usize)> = take_until_empty("water-to-light map:");

    let light_to_temperature: Vec<(usize, usize, usize)> =
        take_until_empty("light-to-temperature map:");

    let temperature_to_humidity: Vec<(usize, usize, usize)> =
        take_until_empty("temperature-to-humidity map:");

    let humidity_to_location: Vec<(usize, usize, usize)> =
        take_until_empty("humidity-to-location map:");

    AlmanacInput {
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    #[test]
    fn one_map_test() {
        let map = OneMap::new(&vec![(50, 98, 2), (52, 50, 48)]);
        assert_eq!(map.v, vec![(50..=97, 2), (98..=99, -48)]);
        let tests = vec![
            (0, 0),
            (1, 1),
            (2, 2),
            (48, 48),
            (49, 49),
            (50, 52),
            (51, 53),
            (97, 99),
            (98, 50),
            (99, 51),
        ];
        for test in tests {
            assert_eq!(map.num_map(test.0), test.1)
        }

        // 2..=3 => 5..=6, 5..=6 => 2..=3
        let map = OneMap::new(&vec![(5, 2, 2), (2, 5, 2)]);
        assert_eq!(map.v, vec![(2..=3, 3), (5..=6, -3)]);
        let tests = vec![
            (0, 0),
            (1, 1),
            (2, 5),
            (3, 6),
            (4, 4),
            (5, 2),
            (6, 3),
            (7, 7),
        ];
        for test in tests {
            assert_eq!(map.num_map(test.0), test.1)
        }
    }
    #[test]
    fn range_intersection_test_1() {
        let one_range = 1..=3;
        let ranges = vec![one_range.clone()];

        for i in 0..=4 {
            for j in i..=4 {
                let has_intersection = has_intersection(one_range.clone(), i..=j);
                let res = enumerate_intersecting_range(&ranges, &(i..=j)) == Some(0..=0);
                assert_eq!(has_intersection, res);
            }
        }
    }
    #[test]
    fn range_intersection_test_2() {
        let ranges = vec![(2..=4), (5..=6), (8..=9)];

        let tests = vec![
            ((0..=0), None),
            ((0..=2), Some(0..=0)),
            ((0..=5), Some(0..=1)),
            ((3..=5), Some(0..=1)),
            ((3..=8), Some(0..=2)),
            ((7..=7), None),
        ];

        for (range, res) in tests {
            assert_eq!(enumerate_intersecting_range(&ranges, &range), res);
        }
    }
    #[test]
    fn unite_test() {
        let ranges = vec![(0..=0), (0..=1)];
        assert_eq!(unite(ranges), vec![(0..=1)]);

        let ranges = vec![(0..=2), (1..=3)];
        assert_eq!(unite(ranges), vec![(0..=3)]);

        let ranges = vec![(0..=0), (0..=5), (1..=2)];
        assert_eq!(unite(ranges), vec![(0..=5)]);

        let ranges = vec![(0..=0), (1..=5), (3..=4)];
        assert_eq!(unite(ranges), vec![(0..=5)]);

        let ranges = vec![(0..=0), (1..=2)];
        assert_eq!(unite(ranges), vec![(0..=2)]);
    }
    #[test]
    fn map_range_test() {
        // 2 3 4 5 6 7 8
        // 6 7 8 5 4 2 3
        let map = OneMap {
            v: vec![(2..=4, 4), (6..=6, -2), (7..=8, -5)],
        };
        let tests = vec![
            ((2..=4), vec![(6..=8)]),
            ((2..=5), vec![(5..=8)]),
            ((4..=6), vec![(4..=5), (8..=8)]),
            ((4..=7), vec![(2..=2), (4..=5), (8..=8)]),
            ((4..=8), vec![(2..=5), (8..=8)]),
        ];
        for (range, res) in tests {
            assert_eq!(map.range_map(range), res);
        }
    }
}
