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