use std::fs;

#[derive(Debug)]
struct Range { min: u32, max: u32 }

impl Range {
    fn is_fully_overlapping(&self, other: &Range) -> bool {
        self.min >= other.min && self.max <= other.max ||
        self.min <= other.min && self.max >= other.max
    }

    fn is_overlapping(&self, other: &Range) -> bool {
        self.min <= other.max && self.max >= other.min
    }
}

fn range_pair_from_line(line: &&str) -> (Range, Range) {
    let pairs: Vec<u32> = line.split(&[',', '-'])
        .map(|v| v.parse::<u32>().unwrap_or(0))
        .collect();
    assert!(pairs.len() == 4);

    (
        Range { min: pairs[0], max: pairs[1] },
        Range { min: pairs[2], max: pairs[3] } 
    )
}

fn main() {
    let file_content = fs::read_to_string("input.txt")
        .expect("Couldn't find input file.");
    let lines = file_content
        .split("\n")
        .collect::<Vec<_>>();

    let fully_overlapping: Vec<(Range, Range)> = lines.iter()
        .map(range_pair_from_line).filter(|range_pair| range_pair.0.is_fully_overlapping(&range_pair.1))
        .collect();

    println!("part 1: {}", fully_overlapping.len());

    let partly_overlapping: Vec<(Range, Range)> = lines.iter()
        .map(range_pair_from_line).filter(|range_pair| range_pair.0.is_overlapping(&range_pair.1))
        .collect();

    println!("part 2: {}", partly_overlapping.len());
}
