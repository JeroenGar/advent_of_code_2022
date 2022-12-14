extern crate core;

use std::collections::HashMap;
use std::hash::BuildHasherDefault;

use fxhash::FxHasher;
use itertools::Itertools;

const INPUT: &str = include_str!("../input/2022/day21.txt");

fn main() {
    let start = std::time::Instant::now();
    let statements = INPUT.lines().filter(|l| !l.is_empty())
        .map(|l| {
            let statement = Statement::from_str(l);
            (statement.rhs().to_owned(), statement)
        }
        ).collect::<HashMap<[char;4], Statement, BuildHasherDefault<FxHasher>>>();

    let statements = Statements {
        list: statements
    };

    println!("Part 1: {}", statements.clone().resolve(&['r','o','o','t']).unwrap());

    let mut statements_pt2 = statements.clone();
    statements_pt2.list.remove(&['h','u','m','n']);
    let root = statements_pt2.list.remove(&['r','o','o','t']).unwrap();
    let equality = match root {
        Statement::Op(_, a, _, b) => (a, b),
        _ => panic!("root is not an op"),
    };

    println!("Part 2: {}", statements_pt2.resolve_advanced(&['h','u','m','n'], &equality).unwrap());
    println!("Time: {:?}", start.elapsed());
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Statement {
    Number([char;4], i64),
    Op([char;4], [char;4], char, [char;4]),
}

#[derive(Debug, Clone)]
struct Statements {
    list: HashMap<[char;4], Statement, BuildHasherDefault<FxHasher>>,
}

#[derive(Debug)]
struct MissingRHS([char;4]);

impl Statements {
    fn resolve(&mut self, name: &[char;4]) -> Result<i64, MissingRHS> {
        match self.list.get(name) {
            None => Err(MissingRHS(name.to_owned())),
            Some(Statement::Number(_, n)) => Ok(*n),
            Some(Statement::Op(_, a, op, b)) => {
                let (a,op,b) = (a.clone(),*op,b.clone());
                let res_a =self.resolve(&a)?;
                let res_b = self.resolve(&b)?;
                let res = do_op(res_a, op, res_b);

                //replace the statement with the result
                self.list.insert(name.to_owned(), Statement::Number(name.to_owned(), res));

                Ok(res)
            }
        }
    }

    fn resolve_advanced(&mut self, name: &[char;4], equality: &([char;4], [char;4])) -> Result<i64, MissingRHS> {
        let mut modified_statements = vec![];

        let mut to_resolve = name.to_owned();
        while let Err(MissingRHS(missing_rhs)) = self.resolve(&to_resolve) {
            //if the missing rhs is one of the equality, add the equality statement
            match (missing_rhs == equality.0, missing_rhs == equality.1) {
                (true, false) => {
                    self.list.insert(['d','m','m','y'], Statement::Number(['d','m','m','y'], 0));
                    self.list.insert(equality.0.to_owned(), Statement::Op(equality.0.to_owned(), equality.1.to_owned(), '+', ['d','m','m','y']));
                    continue;
                }
                (false, true) => {
                    self.list.insert(['d','m','m','y'], Statement::Number(['d','m','m','y'], 0));
                    self.list.insert(equality.1.to_owned(), Statement::Op(equality.1.to_owned(), equality.0.to_owned(), '+', ['d','m','m','y']));
                    continue;
                }
                (false, false) => (),
                (true, true) => panic!("equality invalid"),
            }

            //search for a statement which has not been modified before and where the lhs contains the missing operand
            let statement_to_mod = self.list.iter().find(|(_, s)|
                !modified_statements.contains(*s) && s.contains_operand(&missing_rhs))
                .ok_or_else(|| MissingRHS(missing_rhs.clone()))?.1;

            //modify the statement so the missing operand is the rhs
            let mod_statement = statement_to_mod.change_rhs(&missing_rhs);

            //update datastructures
            modified_statements.push(mod_statement.clone());
            self.list.remove(&statement_to_mod.rhs().to_owned());
            self.list.insert(mod_statement.rhs().to_owned(), mod_statement);

            to_resolve = missing_rhs;
        }

        self.resolve(name)
    }
}

fn do_op(a: i64, op: char, b: i64) -> i64 {
    match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => a / b,
        _ => panic!("Unknown operator {}", op)
    }
}

impl Statement {
    fn rhs(&self) -> &[char;4] {
        match self {
            Statement::Number(n, _) => n,
            Statement::Op(n, _, _, _) => n,
        }
    }

    fn contains_operand(&self, operand: &[char;4]) -> bool {
        match self {
            Statement::Number(_, _) => false,
            Statement::Op(_, a, _, b) => a == operand || b == operand,
        }
    }

    fn change_rhs(&self, new_rhs: &[char;4]) -> Statement {
        match self {
            Statement::Op(rhs, a, op, b) => {
                let (rhs, a, b) = (rhs.to_owned(), a.to_owned(), b.to_owned());
                match op {
                    '+' => { // rhs = a + b
                        match (a == *new_rhs, b == *new_rhs) {
                            (true, false) => Statement::Op(a, rhs, '-', b), //a = rhs - b
                            (false, true) => Statement::Op(b, rhs, '-', a), //b = rhs - a
                            (_, _) => panic!("invalid flip"),
                        }
                    }
                    '-' => { // rhs = a - b
                        match (a == *new_rhs, b == *new_rhs) {
                            (true, false) => Statement::Op(a, rhs, '+', b), //a = rhs + b
                            (false, true) => Statement::Op(b, a, '-', rhs), //b = a - rhs
                            (_, _) => panic!("invalid flip"),
                        }
                    }
                    '*' => { // rhs = a * b
                        match (a == *new_rhs, b == *new_rhs) {
                            (true, false) => Statement::Op(a, rhs, '/', b), //a = rhs / b
                            (false, true) => Statement::Op(b, rhs, '/', a), //b = rhs / a
                            (_, _) => panic!("invalid flip"),
                        }
                    }
                    '/' => { // rhs = a / b
                        match (a == *new_rhs, b == *new_rhs) {
                            (true, false) => Statement::Op(a, rhs, '*', b), //a = rhs * b
                            (false, true) => Statement::Op(b, rhs, '*', a), //b = rhs * a
                            (_, _) => panic!("invalid flip"),
                        }
                    }
                    _ => panic!("unknown op"),
                }
            }
            _ => panic!("Can't flip a number"),
        }
    }

    fn from_str(s: &str) -> Statement {
        let rhs = s.split(':').next().unwrap().chars().collect_vec().try_into().unwrap();
        match s.split(':').nth(1).unwrap().trim().parse() {
            Ok(n) => Statement::Number(rhs, n),
            Err(_) => {
                let a = s.trim().split(' ').nth(1).unwrap().chars().collect_vec().try_into().unwrap();
                let op = s.trim().split(' ').nth(2).unwrap().chars().next().unwrap();
                let b = s.trim().split(' ').nth(3).unwrap().chars().collect_vec().try_into().unwrap();
                Statement::Op(rhs, a, op, b)
            }
        }
    }
}