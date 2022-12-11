use std::rc::Rc;
use std::str::FromStr;
use std::time::Instant;

use num::integer::lcm;

use aoc2022::parse_to_vec;

const INPUT: &str = include_str!("../input/2022/day11.txt");

fn main() {
    let start = Instant::now();
    let mut monkeys_1: Vec<Monkey> = parse_to_vec(INPUT, "\n\n").unwrap();
    let mut monkeys_2 = monkeys_1.clone();
    //least common multiple of all the monkey's divisors
    let lcm = monkeys_1.iter().fold(1, |acc, m| lcm(acc, m.divisor));

    for _ in 0..20 {
        for j in 0..monkeys_1.len() {
            monkeys_1[j].inspect_and_throw(lcm, 3).iter()
                .for_each(|(item, m_index)| monkeys_1[*m_index].receive(*item));
        }
    }
    monkeys_1.sort_by(|a, b| a.n_inspections.cmp(&b.n_inspections).reverse());
    println!("Part 1: {}", monkeys_1[0].n_inspections * monkeys_1[1].n_inspections);

    for _ in 0..10000 {
        for j in 0..monkeys_2.len() {
            monkeys_2[j].inspect_and_throw(lcm, 1).iter()
                .for_each(|(item, m_index)| monkeys_2[*m_index].receive(*item));
        }
    }
    monkeys_2.sort_by(|a, b| a.n_inspections.cmp(&b.n_inspections).reverse());
    println!("Part 2: {}", monkeys_2[0].n_inspections * monkeys_2[1].n_inspections);

    println!("Time: {}ms", start.elapsed().as_millis());
}

#[derive(Clone)]
struct Monkey {
    pub inventory: Vec<usize>,
    pub worry_op: Rc<dyn Fn(usize) -> usize>, //Rc instead of Box so we can clone Monkey
    pub throw_to: Rc<dyn Fn(usize) -> usize>,
    pub divisor: usize,
    pub n_inspections: usize,
}

impl Monkey {
    fn inspect_and_throw(&mut self, lcm: usize, divider: usize) -> Vec<(usize, usize)> {
        self.inventory.drain(..).map(|item| {
            self.n_inspections += 1;
            let new_item_value = ((self.worry_op)(item) / divider) % (lcm * divider);
            let throw_to = (self.throw_to)(new_item_value);
            (new_item_value, throw_to)
        }).collect()
    }

    fn receive(&mut self, item: usize) {
        self.inventory.push(item);
    }
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        lines.next();
        let inventory_line = lines.next().unwrap().split(':').nth(1).unwrap();
        let inventory: Vec<usize> = inventory_line.split(',').filter_map(|x| x.trim().parse().ok()).collect();
        let op_str = lines.next().unwrap().split(':').nth(1).unwrap();
        let operation_symbol = op_str.split(' ').nth(4).unwrap();
        let operand = op_str.split(' ').nth(5).unwrap();

        let worry_op: Rc<dyn Fn(usize) -> usize> = match operand {
            "old" => {
                match operation_symbol {
                    "*" => Rc::new(|x| x * x),
                    "+" => Rc::new(|x| x + x),
                    _ => { panic!("Unknown operation_symbol {}", operation_symbol) }
                }
            }
            number => {
                let number: usize = number.parse().unwrap();
                match operation_symbol {
                    "*" => Rc::new(move |x| x * number),
                    "+" => Rc::new(move |x| x + number),
                    _ => { panic!("Unknown operation_symbol {}", operation_symbol) }
                }
            }
        };

        let divisor: usize = lines.next().unwrap().trim().split(' ').nth(3).unwrap().parse().unwrap();
        let throw_if_true: usize = lines.next().unwrap().trim().split(' ').nth(5).unwrap().parse().unwrap();
        let throw_if_false: usize = lines.next().unwrap().trim().split(' ').nth(5).unwrap().parse().unwrap();

        let throw_to = Rc::new(move |x| if x % divisor == 0 { throw_if_true } else { throw_if_false });

        Ok(Monkey {
            inventory,
            worry_op,
            throw_to,
            divisor,
            n_inspections: 0,
        })
    }
}