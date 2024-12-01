struct LocationPair { left: u32,  right: u32}
pub struct LocationLists {left: Vec<u32>, right: Vec<u32>}
const SPLITER: &str = "   ";
fn to_location_lists(left:Vec<u32>, right:Vec<u32>) -> LocationLists{
    LocationLists { left: left, right : right }
}
fn to_location_pair(left:u32, right:u32) -> LocationPair{
    LocationPair { left: left, right : right }
}
fn parse_line(line: &str) -> LocationPair {
    let mut location = line.trim()
        .split(SPLITER)
        .map(|d| d.parse::<u32>().unwrap());
        to_location_pair ( location.next().unwrap(), location.next().unwrap())
}
fn distance(l:u32 , r:u32) -> u32 {
    if l > r {
        l - r
    } else {
        r - l
    }
}
fn next(l:u32 , r:u32) -> u32 {
    if l > r {
        r
    } else {
        l
    }
}

fn greater(l:u32 , r:u32) -> u32 {
    if l > r {
        l
    } else {
        r
    }
}
#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> LocationLists {
    let mut left_locations = Vec::new();    
    let mut right_locations = Vec::new();   
    for line in input.lines()  {
        let pair:LocationPair = parse_line(line);
        left_locations.push(pair.left);
        right_locations.push(pair.right);
    }
    left_locations.sort();
    right_locations.sort();
    to_location_lists(left_locations,right_locations)
}
#[aoc(day1, part1)]
pub fn solve_part1(lists: &LocationLists) -> u32 {
    let mut sum:u32  = 0;
    for i in 0..lists.left.len() {
        sum += distance(lists.left[i],lists.right[i]);
    }   
    sum
}

#[aoc(day1, part2)]
pub fn solve_part2( lists: &LocationLists) -> u32 {
    let mut sum:u32  = 0;
    let mut i = 0;
    let mut j = 0;
    let left = &lists.left; 
    let right = &lists.right;
    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            i += 1;
        } else if left[i] == right[j] {
            sum += left[i];
            j += 1;
        } else {
            j += 1;
        }
    }
    sum
}
