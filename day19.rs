use std::collections::HashMap;

#[derive(Debug,Clone,PartialEq)]
enum Condition {
    Gt,Lt,Un
}

#[derive(Debug,Clone)]
struct Rule<'a> {
    var: char,cond: Condition,val: usize,next: &'a str,
}

use Condition::*;

type Part = HashMap<char,usize>;
type Parts = Vec<Part>;
type PartRange = HashMap<char,(usize,usize)>;

type Workflow<'a> = Vec<Rule<'a>>;
type Workflows<'a> = HashMap<String,Workflow<'a>>;

fn parse(input: &str) -> (Workflows, Parts)  {
    let workflows_str: Vec<&str> = input
    .lines()
    .filter(|&line| !(line.starts_with("{") || line.is_empty()))
    .collect();

    let parts_str: Vec<&str> = input
    .lines()
    .filter(|&line| line.starts_with("{"))
    .collect();

    let mut workflows: Workflows = HashMap::new();

    for x in workflows_str {
        let start = x.find('{').unwrap();
        let name = x[..start].to_string();
        let temp: Vec<&str> = (&x[start+1..x.len()-1]).split(',').into_iter().collect();
        let mut workflow: Workflow = Vec::new();

        for y in temp {
            if let Some((temp1,temp2)) = y.split_once(":") {         
                let mut c = temp1.chars();
                workflow.push(Rule {
                    var : c.next().unwrap(),
                    cond: match c.next().unwrap() {
                        '>' => Gt,
                        '<' => Lt,
                         _  => Un,
                    },
                    val : temp1[2..].parse().unwrap(),
                    next: temp2,
                });
            }
            else {
                workflow.push(Rule{var:'z',cond:Un,val:0,next:y});
            }
            workflows.insert(name.clone(),workflow.clone());
        }
    }

    let mut parts: Parts = Vec::new();
    for x in parts_str {
        let vals = x
        .trim_start_matches('{')
        .trim_end_matches('}')
        .split(",").collect::<Vec<&str>>();
        let mut part: Part = HashMap::new();
        for y in vals {
            let (c,val) = y.split_once('=').unwrap();
            let num = val.parse().unwrap();
            part.insert(c.chars().nth(0).unwrap(), num);
        }
        parts.push(part);
    }
    (workflows,parts)
}

fn check_valid_part(workflows: &Workflows,part: &Part) -> bool {
    let mut next = "in";
    'a:loop {
        let workflow = workflows.get(next).unwrap(); 
        for rule in workflow {
            let result = match &rule.cond {
                Gt => if *part.get(&rule.var).unwrap() > rule.val {true} else {false},
                Lt => if *part.get(&rule.var).unwrap() < rule.val {true} else {false},
                Un => true
            };
            if result {
                match rule.next {
                    "A" => return true,
                    "R" => return false,
                     _  => {
                        next = rule.next;
                        continue 'a;
                    }
                }
            }
        }
    }
}

fn part1(workflows: &Workflows,parts: &Parts) -> usize {
    let valid_parts = parts.iter()
    .filter(|part: &&Part| check_valid_part(workflows,*part));

    valid_parts.map(|part: &Part| part.values().sum::<usize>()).sum()
}

fn find_valid_ranges(workflows: &Workflows) -> Vec<PartRange> {
    let mut search_ranges: PartRange = HashMap::new();
    ['x','m','a','s'].into_iter().for_each(|c| {search_ranges.insert(c,(1,4000));});

    let mut stack = Vec::new();
    stack.push((search_ranges,"in"));

    let mut valid_ranges: Vec<PartRange> = Vec::new();

    while let Some((mut ranges,name)) = stack.pop() {
        if name == "A" {
            valid_ranges.push(ranges);
            continue;
        }
        else if name == "R" {
            continue;
        }
        let workflow = workflows.get(name).unwrap();
        for Rule{var,cond,val,next} in workflow {
            let mut new_ranges: PartRange = ranges.clone();
            match cond {
                Gt => {
                    let (from,_) = *ranges.get(var).unwrap();
                    let (_,new_to) = *new_ranges.get(var).unwrap();
                    ranges.insert(*var, (from,*val));
                    new_ranges.insert(*var,(val+1,new_to));
                }
                Lt => {
                    let (_,to) = *ranges.get(var).unwrap();
                    let (new_from,_) = *new_ranges.get(var).unwrap();
                    ranges.insert(*var, (*val,to));
                    new_ranges.insert(*var,(new_from,*val-1));
                }
                Un => ()
            }
            stack.push((new_ranges,next));
        }
    }
    valid_ranges
}

fn part2(workflows: &Workflows) -> usize {
    let valid_ranges = find_valid_ranges(workflows);
    valid_ranges.iter()
    .map(|ranges| {
        ranges.values().map(|(from,to)| to - from + 1).product::<usize>()
    }).sum()
}

fn solve(input: &str) -> (usize,usize) {
    let (workflows,parts) = parse(input);
    (part1(&workflows,&parts),part2(&workflows))
}

fn main() {
    let input = include_str!("input.txt");
    let (part1,part2) = solve(input);
    println!("Part 1: {}\nPart 2: {}",part1,part2);
}
