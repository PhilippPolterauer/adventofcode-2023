use std::{time::Instant, vec};

#[derive(Debug, Clone)]
struct Range {
    start: i64,
    length: i64,
}
impl Range {
    fn end(&self) -> i64 {
        self.start + self.length
    }
    fn empty() -> Self {
        Self {
            start: 0,
            length: 0,
        }
    }
    fn is_empty(&self) -> bool {
        self.length <= 0
    }
    fn intersect_remainder(&self, other: Range) -> (Range, Vec<Range>) {
        // there are four possible solution for the difference
        let (a, b, c, d) = (self.start, self.end(), other.start, other.end());
        // self fully contained in other
        if a >= c && b <= d {
            (
                Range {
                    start: self.start,
                    length: self.length,
                },
                vec![],
            )
        } else if c >= a && d <= b {
            // other fully contained in self
            (
                other,
                vec![Range::from_start_end(&a, &c), Range::from_start_end(&d, &b)],
            )
        } else if c < a {
            // left clip
            (
                Range::from_start_end(&a, &d),
                vec![Range::from_start_end(&d, &b)],
            )
        } else {
            // right clip C<B
            (
                Range::from_start_end(&c, &b),
                vec![Range::from_start_end(&a, &c)],
            )
        }
        // let A = Range::from_start_end(self.start, other.end())
        // let B = Range::from_start_end(other.start, self.end())
    }
    fn from_start_end(start: &i64, end: &i64) -> Range {
        Range {
            start: *start,
            length: end - start,
        }
    }
}

#[derive(Debug)]
struct RangeMap {
    dst: i64,
    src: i64,
    length: i64,
}
impl RangeMap {
    fn from_line(line: &str) -> Self {
        let data = iter2i64(line.split(' '));
        let src = data[1];
        let dst = data[0];
        let length = data[2];
        Self { dst, src, length }
    }
    fn offset(&self) -> i64 {
        self.dst - self.src
    }
    fn apply(&self, number: &i64) -> Option<i64> {
        let Self {
            src,
            dst: _,
            length,
        } = self;
        let offset = self.offset();
        if (*src..(src + length)).contains(number) {
            return Some(number + offset);
        }
        None
    }
    fn src_range(&self) -> Range {
        Range {
            start: self.src,
            length: self.length,
        }
    }
    fn apply2range(&self, range: &Range) -> (Range, Vec<Range>) {
        let (intersection, remainder) = range.intersect_remainder(self.src_range());
        if intersection.is_empty() {
            // if intersection is empty the mapped range keeps the same
            (Range::empty(), vec![range.clone()])
        } else {
            // if an intersection was found we shift it by offset and return the remaining range
            (
                Range {
                    start: intersection.start + self.offset(),
                    length: intersection.length,
                },
                remainder,
            )
        }
    }
    fn apply2ranges(&self, ranges: &[Range]) -> (Vec<Range>, Vec<Range>) {
        // takes a vector of ranges and returns a vector of ranges representing the mapped regions
        let mut mapped_ranges = vec![];
        let mut unmapped_ranges = vec![];
        for range in ranges.iter() {
            let (mapped, remainder) = self.apply2range(range);
            if !mapped.is_empty() {
                mapped_ranges.push(mapped);
            }
            for rem in remainder.iter().filter(|r| !r.is_empty()) {
                //adding non empty ranges
                unmapped_ranges.push(rem.clone())
            }
            // we add the non empty remaining ranges to the list of ranges
        }
        (mapped_ranges, unmapped_ranges)
    }
}
#[derive(Debug)]
struct Map {
    maps: Vec<RangeMap>,
}
impl Map {
    fn apply(&self, number: i64) -> i64 {
        self.maps
            .iter()
            .find_map(|map| map.apply(&number))
            .unwrap_or(number)
    }
    fn apply2ranges(&self, ranges: Vec<Range>) -> Vec<Range> {
        let mut unmapped = ranges;
        let mut mapped = vec![];
        for map in self.maps.iter() {
            let result = map.apply2ranges(&unmapped);
            unmapped = result.1;
            for range in result.0.iter() {
                mapped.push(range.clone());
            }
        }

        mapped.append(&mut unmapped);
        mapped
    }
}

fn iter2i64<'a>(seed_iter: impl Iterator<Item = &'a str>) -> Vec<i64> {
    let mut numbers = vec![];
    for seedstr in seed_iter {
        let seed = seedstr.to_string().parse::<i64>().unwrap();
        numbers.push(seed);
    }
    numbers
}
fn parse_map<'a>(mut lines: impl Iterator<Item = &'a str>, start_marker: &str) -> Map {
    lines.find(|s| *s == start_marker);
    let mut maps = vec![];
    for line in lines {
        if line.is_empty() {
            break;
        }
        maps.push(RangeMap::from_line(line));
    }
    Map { maps }
}
const MARKERS: [&str; 7] = [
    "seed-to-soil map:",
    "soil-to-fertilizer map:",
    "fertilizer-to-water map:",
    "water-to-light map:",
    "light-to-temperature map:",
    "temperature-to-humidity map:",
    "humidity-to-location map:",
];
pub fn part1(input: String) -> i64 {
    let mut lines = input.lines();

    let mut seed_iter = lines.next().unwrap().split(' ');
    seed_iter.next();
    let seeds = iter2i64(seed_iter);

    let mut map_chain = vec![];
    for marker in MARKERS.iter() {
        let map = parse_map(&mut lines, marker);
        map_chain.push(map);
    }

    let mut results = vec![];
    for seed in seeds.iter() {
        let mut maped = seed.to_owned();
        for map in map_chain.iter() {
            maped = map.apply(maped);
        }
        results.push(maped)
    }
    let solution = results.iter().min().unwrap();

    *solution
}
pub fn part2(input: String) -> i64 {
    let _start = Instant::now();
    let mut lines = input.lines();

    let mut seed_iter = lines.next().unwrap().split(' ');
    seed_iter.next();
    let seeds = iter2i64(seed_iter);

    let mut seed_ranges = vec![];
    for i in (0..seeds.len()).step_by(2) {
        seed_ranges.push(Range {
            start: seeds[i],
            length: seeds[i + 1],
        })
    }

    let mut map_chain = vec![];
    for marker in MARKERS.iter() {
        let map = parse_map(&mut lines, marker);
        map_chain.push(map);
    }

    let mut ranges = seed_ranges;

    for map in map_chain.iter() {
        // &map);
        ranges = map.apply2ranges(ranges);
        // &ranges);
    }

    let solution = ranges.iter().min_by_key(|r| r.start).unwrap().start;

    solution
}

#[cfg(test)]
mod tests {}
