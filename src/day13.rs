extern crate core;

use std::cmp::Ordering;
use std::str::FromStr;
use itertools::Itertools;

use serde_json::value::Value;

const INPUT: &str = include_str!("../input/2022/day13.txt");

fn main() {
    let packets = INPUT.replace("\n\n", "\n").lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<Packet>>();
    let part_1 = (1..=(packets.len() / 2)).zip(packets.iter().tuples().collect::<Vec<(&Packet, &Packet)>>())
        .filter(|(_, (p1, p2))| packet_pair_right_order(p1, p2).unwrap())
        .fold(0, |acc, (i, _)| acc + i);

    println!("Part 1: {}", part_1);

    let distress = (
        Packet { data: vec![Element::Vec(vec![Element::Data(2)])] },
        Packet { data: vec![Element::Vec(vec![Element::Data(6)])] }
    );

    let sorted_packets = packets.iter()
        .chain([&distress.0, &distress.1])
        .sorted_by(|p1, p2| {
            match packet_pair_right_order(p1, p2) {
                Some(true) => Ordering::Less,
                Some(false) => Ordering::Greater,
                None => Ordering::Equal,
            }
        }).collect::<Vec<&Packet>>();

    let part_2 = (sorted_packets.iter().position(|p| std::ptr::eq(*p, &distress.0)).unwrap() + 1) *
        (sorted_packets.iter().position(|p| std::ptr::eq(*p, &distress.1)).unwrap() + 1);

    println!("Part 2: {}", part_2);
}

fn packet_pair_right_order(p1: &Packet, p2: &Packet) -> Option<bool> {
    p1.data.iter().zip(p2.data.iter())
        .map(|(e1, e2)| {
            elements_right_order(e1, e2)
        })
        .find(|x| x.is_some())
        .unwrap_or(if p1.data.len() == p2.data.len() { None } else { Some(p1.data.len() < p2.data.len()) })
}

fn elements_right_order(e1: &Element, e2: &Element) -> Option<bool> {
    match (e1, e2) {
        (Element::Data(d1), Element::Data(d2)) => {
            if d1 == d2 { None } else { Some(d1 < d2) }
        }
        (Element::Vec(v1), Element::Vec(v2)) => {
            v1.iter().zip(v2.iter())
                .map(|(e1, e2)| elements_right_order(e1, e2))
                .find(|x| x.is_some())
                .unwrap_or(if v1.len() == v2.len() { None } else { Some(v1.len() < v2.len()) })
        }
        (Element::Vec(_), Element::Data(_)) => {
            elements_right_order(e1, &Element::Vec(vec![e2.clone()]))
        }
        (Element::Data(_), Element::Vec(_)) => {
            elements_right_order(&Element::Vec(vec![e1.clone()]), e2)
        }
    }
}

#[derive(Debug, Clone)]
struct Packet {
    pub data: Vec<Element>,
}

#[derive(Debug, Clone)]
enum Element {
    Data(usize),
    Vec(Vec<Element>),
}

impl FromStr for Packet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match serde_json::from_str(s) {
            Ok(Value::Array(v)) => {
                let data = v.iter().map(Element::from_json).collect::<Vec<Element>>();
                Ok(Packet { data })
            }
            _ => Err(()),
        }
    }
}

impl Element {
    fn from_json(val: &Value) -> Element {
        match val {
            Value::Array(v) => Element::Vec(v.iter().map(Element::from_json).collect::<Vec<Element>>()),
            Value::Number(v) => Element::Data(v.as_u64().unwrap() as usize),
            _ => panic!("unexpected json value")
        }
    }
}