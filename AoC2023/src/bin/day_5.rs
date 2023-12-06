use std::ops::RangeInclusive;

// input ([l, r], d) return [l+d, r+d]
fn shift(range: RangeInclusive<usize>, u: isize) -> RangeInclusive<usize> {
    let start = (*range.start() as isize + u) as usize;
    let end = (*range.end() as isize + u) as usize;
    start..=end
}

fn dest(range: &RangeInclusive<usize>) -> (usize, usize) {
    (*range.start(), *range.end())
}

fn cap(
    range1: RangeInclusive<usize>,
    range2: RangeInclusive<usize>,
) -> Option<RangeInclusive<usize>> {
    let (l1, r1) = (*range1.start(), *range1.end());
    let (l2, r2) = (*range2.start(), *range2.end());
    if l1 <= r2 && l2 <= r1 {
        Some(std::cmp::max(l1, l2)..=std::cmp::min(r1, r2))
    } else {
        None
    }
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

    let (mut li, mut ri) = dest(&ranges[0]);

    for i in 1..ranges.len() {
        if ri + 1 < *ranges[i].start() {
            v.push(li..=ri);
            (li, ri) = dest(&ranges[i]);
        } else {
            ri = std::cmp::max(ri, *ranges[i].end());
        }
    }

    v.push(li..=ri);

    v
}

// input :
//  - ranges = [[l0, r0], [l1, r1], ..., [l{n-1}, r{n-1}]]
//    such that ri < l{i+1} so ranges is sorted by both
//      - |l1..=r1, l2..=r2| (l1, r1) < (l2, r2) (dictionary order of tuple)
//      - |l1..=r1, l2..=r2| (r1, l1) < (r2 < l2) (dictionary order of reversed tuple)
//  - range = [l, r]
// output: {k | [lk, rk] has intersection with [l, r]} as range (if not empty)
fn enumerate_intersecting_range(
    ranges: &[RangeInclusive<usize>],
    range: &RangeInclusive<usize>,
) -> Option<RangeInclusive<usize>> {
    let n = ranges.len();
    // debug assertion
    for i in 0..n - 1 {
        debug_assert!(ranges[i].end() < ranges[i + 1].start());
    }

    // le = min { 0 <= i < n | l <= ri }
    let le = ranges.partition_point(|i_th_range| range.start() > i_th_range.end());
    // k in [le..] satisfies l <= rk
    // le <= i <=> l <= ri

    // ge = min { 0 <= i < n | li > r}
    let ge = ranges.partition_point(|i_th_range| i_th_range.start() <= range.end());
    // k in [0..ge] satisfies lk <= r
    // i < ge <=> li <= r

    if 0 < ge && le < ge && le < n {
        Some(le..=ge - 1)
    } else {
        None
    }
}

// contains ([r..=l], d) : [r..=l] => [r+d..=l+d]
struct OneMap(Vec<(RangeInclusive<usize>, isize)>);

impl OneMap {
    fn new(v: &Vec<(usize, usize, usize)>) -> Self {
        let mut v = v
            .iter()
            .map(|&(v0, v1, v2)| ((v1..=v1 + v2 - 1), v0 as isize - v1 as isize))
            .collect::<Vec<_>>();
        v.sort_by_key(|(range, _)| *range.start());
        // no intersection between range
        // i < j <=> v[i].start() < v[j].start() <=> v[i].end() < v[j].end()
        Self(v)
    }
    fn num_map(&self, from: usize) -> usize {
        // m = min { 0 <= i < n-1 | from < li }
        let m = self.0.partition_point(|(range, _)| *range.start() <= from);
        if m == 0 {
            from
        } else if self.0[m - 1].0.contains(&from) {
            (from as isize + self.0[m - 1].1) as usize
        } else {
            from
        }
    }
    fn range_map(&self, range: RangeInclusive<usize>) -> Vec<RangeInclusive<usize>> {
        let (l, r) = (*range.start(), *range.end());

        let ranges = self
            .0
            .iter()
            .map(|(range, _)| range.clone())
            .collect::<Vec<_>>();

        let Some(range_of_intersect) = enumerate_intersecting_range(&ranges, &range) else {
            return vec![range];
        };
        let (s, t) = dest(&range_of_intersect);

        let mut v = vec![];

        let ls = *ranges[s].start();
        // if l < ls, [l..ls-1] is mapped to self
        if l < ls {
            v.push(l..=ls-1);
        }

        // [li..ri] cap [l, r] is mapped to [li+di..ri+di] cap [l+di, r+di]
        for i in s..=t {
            let (li, ri) = dest(&self.0[i].0);
            v.extend(cap(li..=ri, l..=r).map(|range| shift(range, self.0[i].1)));
        }

        // [l.........................r]
        //   [li..ri] [l{i+1}..r{i+1}]
        // [{ri}+1..l{i+1}-1] mapped to self
        for i in s..t {
            let l = self.0[i].0.end() + 1;
            let r = self.0[i + 1].0.start() - 1;
            if l <= r {
                v.push(l..=r);
            }
        }

        // if rs < r, [rs+1..r] is mapped to self
        let rt = *ranges[t].end();
        if rt < r {
            v.push(rt+1..=r);
        }

        unite(v)
    }
    fn ranges_map(&self, ranges: Vec<RangeInclusive<usize>>) -> Vec<RangeInclusive<usize>> {
        unite(
            ranges
                .into_iter()
                .flat_map(|range| self.range_map(range))
                .collect::<Vec<_>>(),
        )
    }
}

struct AlmanacPart1 {
    seeds: Vec<usize>,
    maps: Vec<OneMap>,
}

impl AlmanacPart1 {
    fn from_input(almanac: &AlmanacInput) -> Self {
        let AlmanacInput { seeds, maps } = almanac;
        Self {
            seeds: seeds.to_owned(),
            maps: maps.iter().map(OneMap::new).collect(),
        }
    }
}

fn compute_part1(almanac: AlmanacPart1) -> usize {
    let mut numbers = almanac.seeds;
    for map in almanac.maps {
        for num in numbers.iter_mut() {
            *num = map.num_map(*num);
        }
    }
    numbers.into_iter().min().unwrap()
}

struct AlmanacPart2 {
    seeds: Vec<RangeInclusive<usize>>,
    maps: Vec<OneMap>,
}

impl AlmanacPart2 {
    fn from_input(almanac: &AlmanacInput) -> AlmanacPart2 {
        let AlmanacInput { seeds, maps } = almanac;
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
            maps: maps.iter().map(OneMap::new).collect(),
        }
    }
}

fn compute_part2(almanac: AlmanacPart2) -> usize {
    let AlmanacPart2 {
        seeds: mut ranges,
        maps,
    } = almanac;

    for map in maps {
        ranges = map.ranges_map(ranges);
    }

    *ranges[0].start()
}

fn main() {
    let almanac = input();
    println!("{}", compute_part1(AlmanacPart1::from_input(&almanac)));
    println!("{}", compute_part2(AlmanacPart2::from_input(&almanac)));
}

struct AlmanacInput {
    seeds: Vec<usize>,
    maps: Vec<Vec<(usize, usize, usize)>>,
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
        for line in lines.by_ref() {
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
        maps: vec![
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        ],
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn one_map_test() {
        let map = OneMap::new(&vec![(50, 98, 2), (52, 50, 48)]);
        assert_eq!(map.0, vec![(50..=97, 2), (98..=99, -48)]);
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
        assert_eq!(map.0, vec![(2..=3, 3), (5..=6, -3)]);
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
    fn enumerate_test() {
        let ranges = vec![(0..=0)];
        let range = 0..=0;
        let res = Some(0..=0);
        assert_eq!(enumerate_intersecting_range(&ranges, &range), res);

        let ranges = vec![(1..=2)];
        let tests = vec![
            ((0..=0), None),
            ((1..=1), Some(0..=0)),
            ((2..=2), Some(0..=0)),
            ((3..=3), None),
        ];
        for (range, res) in tests {
            assert_eq!(enumerate_intersecting_range(&ranges, &range), res);
        }

        let ranges = vec![(1..=2), (4..=5)];
        let tests = vec![
            ((0..=0), None),
            ((1..=1), Some(0..=0)),
            ((2..=2), Some(0..=0)),
            ((3..=3), None),
            ((4..=4), Some(1..=1)),
            ((5..=5), Some(1..=1)),
            ((6..=6), None),
            ((2..=4), Some(0..=1)),
        ];
        for (range, res) in tests {
            assert_eq!(enumerate_intersecting_range(&ranges, &range), res);
        }
    }
    #[test]
    fn map_num_test() {
        // 2 3 4 5 6 7 8
        // 6 7 8 5 4 2 3
        let map = OneMap(vec![(2..=4, 4), (6..=6, -2), (7..=8, -5)]);
        let tests = vec![(2, 6), (3, 7), (4, 8), (5, 5), (6, 4), (7, 2), (8, 3)];
        for (num, res) in tests {
            assert_eq!(map.num_map(num), res)
        }
    }
    #[test]
    fn map_range_test() {
        // 2 3 4 5 6 7 8
        // 6 7 8 5 4 2 3
        let map = OneMap(vec![(2..=4, 4), (6..=6, -2), (7..=8, -5)]);
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
