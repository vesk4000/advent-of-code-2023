use std::collections::{HashMap, HashSet, LinkedList};
use std::fs;
use petgraph::Direction;
use petgraph::dot::Dot;
use petgraph::graph::{DefaultIx, DiGraph, NodeIndex};
use replace_with::replace_with_or_abort;
use substring::Substring;
use crate::Machine::Broadcast;

#[derive(Eq, Hash, PartialEq, Clone, Debug, Default)]
enum Machine {
    FlipFlop(String, bool),
    Conjunction(String),
    Broadcast(String),
    #[default]
    Def,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pulse {
    Low,
    High,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/day20.txt")?;
    let lines = input.trim().split('\n');

    let mut graph = DiGraph::<Machine, Pulse>::new();
    let mut name2node = HashMap::new();
    let mut edges = HashSet::<(&str, &str)>::new();

    for line in lines {
        let (src, dst) = line.split_once(" -> ").unwrap();
        let machine: Machine;
        let mut name;
        if src == "broadcaster" {
            name = src;
            machine = Machine::Broadcast(name.to_string());
        } else if src.as_bytes()[0] == u8::try_from('%').unwrap() {
            name = &src.substring(1, src.len());
            machine = Machine::FlipFlop(name.to_string(), false);
        } else if src.as_bytes()[0] == u8::try_from('&').unwrap() {
            name = src.substring(1, src.len());
            machine = Machine::Conjunction(name.to_string());
        } else {
            return Err(Box::try_from("Uh oh, something's wrong, I can feel it").unwrap());
        }
        name2node.insert(name, graph.add_node(machine));
        edges.extend(dst.split(", ").map(|x| (name, x)).collect::<Vec<_>>());
    }

    dbg!(Dot::new(&graph));

    for edge in edges {
        graph.add_edge(*name2node.entry(edge.0).or_default(), *name2node.entry(edge.1).or_default(), Pulse::Low);
    }


    dbg!(Dot::new(&graph));

    let mut pulses_high = 0;
    let mut pulses_low = 0;

    // push button
    for i in 0..1000 {
        let mut q = LinkedList::<(NodeIndex<DefaultIx>, Pulse)>::new();
        q.push_back((name2node["broadcaster"], Pulse::Low));
        while !q.is_empty() {
            let (node_ix, pulse) = q.pop_front().unwrap();
            if pulse == Pulse::Low {
                pulses_low += 1;
            } else {
                pulses_high += 1;
            }
            let node = graph.node_weight(node_ix).unwrap();// &mut graph[node_ix];
            let mut new_pulse = None;
            let new_node = match node {
                Machine::FlipFlop(name, enabled) => {
                    if pulse == Pulse::Low {
                        if *enabled {
                            new_pulse = Some(Pulse::Low)
                        } else {
                            new_pulse = Some(Pulse::High)
                        }
                        Machine::FlipFlop(name.clone(), !(*enabled))
                    } else {
                        Machine::FlipFlop(name.clone(), *enabled)
                    }
                }
                Machine::Conjunction(name) => {
                    let mut nbs = graph.edges_directed(node_ix, Direction::Incoming);
                    if nbs.all(|nb| *(nb.weight()) == Pulse::High) {
                        new_pulse = Some(Pulse::Low)
                    } else {
                        new_pulse = Some(Pulse::High)
                    }
                    Machine::Conjunction(name.clone())
                }
                Machine::Broadcast(name) => {
                    new_pulse = Some(pulse);
                    Broadcast(name.clone())
                }
                Machine::Def => { Machine::Def }
            };

            let mut nbs = Vec::new();

            if new_pulse.is_some() {
                let pulse_new = new_pulse.unwrap();
                for nb in graph.neighbors_directed(node_ix, Direction::Outgoing) {
                    nbs.push(nb);
                    q.push_back((nb, pulse_new));
                }

                for nb in nbs {
                    graph.update_edge(node_ix, nb, pulse_new);
                }
            }

            let mut mut_node = &mut graph[node_ix];
            *mut_node = new_node;
        }

        //dbg!(pulses_high, pulses_low);
    }
    println!("{}", pulses_high * pulses_low);

    Ok(())
}

static EXAMPLE: &str = "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";