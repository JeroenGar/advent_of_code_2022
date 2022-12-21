extern crate core;

use std::collections::HashMap;
use std::hash::BuildHasherDefault;

use fxhash::FxHasher;

const INPUT: &str = include_str!("../input/2022/day21.txt");

fn main() {
    let start = std::time::Instant::now();
    let statements = INPUT.lines().filter(|l| !l.is_empty())
        .map(|l| {
            let statement = Statement::from_str(l);
            (statement.rhs().to_owned(), statement)
        }
        ).collect::<HashMap<String, Statement, BuildHasherDefault<FxHasher>>>();

    let statements = Statements {
        list: statements
    };

    println!("Part 1: {}", statements.clone().resolve("root").unwrap());

    let mut statements_pt2 = statements.clone();
    statements_pt2.list.remove("humn");
    let root = statements_pt2.list.remove("root").unwrap();
    let equality = match root {
        Statement::Op(_, a, _, b) => (a, b),
        _ => panic!("root is not an op"),
    };

    println!("Part 2: {}", statements_pt2.resolve_advanced("humn", &equality).unwrap());
    println!("Time: {:?}", start.elapsed());
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Statement {
    Number(String, i64),
    Op(String, String, char, String),
}

#[derive(Debug, Clone)]
struct Statements {
    list: HashMap<String, Statement, BuildHasherDefault<FxHasher>>,
}

#[derive(Debug)]
struct MissingStatement(String);

impl Statements {
    fn resolve(&mut self, name: &str) -> Result<i64, MissingStatement> {
        match self.list.get(name) {
            None => Err(MissingStatement(name.to_owned())),
            Some(Statement::Number(_, n)) => Ok(*n),
            Some(Statement::Op(_, a, op, b)) => {
                let (a,op,b) = (a.clone(),*op,b.clone());
                let res = do_op(self.resolve(&a)?, op, self.resolve(&b)?);

                //replace the statement with the result
                self.list.insert(name.to_owned(), Statement::Number(name.to_owned(), res));

                Ok(res)
            }
        }
    }

    fn resolve_advanced(&mut self, name: &str, equality: &(String, String)) -> Result<i64, MissingStatement> {
        //IDEA: add statements, or flip them around until we can resolve name

        let mut flipped_statements = vec![];

        while let Err(MissingStatement(missing)) = self.resolve(name) {
            //if the missing statement is one of the equality, add the equality statement
            match (missing == equality.0, missing == equality.1) {
                (false, false) => (),
                (true, false) => {
                    self.list.insert("dummy".to_owned(), Statement::Number("dummy".to_owned(), 0));
                    self.list.insert(equality.0.to_owned(), Statement::Op(equality.0.to_owned(), equality.1.to_owned(), '+', "dummy".to_owned()));
                    continue;
                }
                (false, true) => {
                    self.list.insert("dummy".to_owned(), Statement::Number("dummy".to_owned(), 0));
                    self.list.insert(equality.1.to_owned(), Statement::Op(equality.1.to_owned(), equality.0.to_owned(), '+', "dummy".to_owned()));
                    continue;
                }
                (true, true) => panic!("equality invalid"),
            }

            //search for a statement which has not been flipped before that contains the missing operand and modify it's rhs
            let statement_to_flip = self.list.iter().find(|(_, s)|
                !flipped_statements.contains(*s) && s.contains_operand(&missing)).unwrap().1;

            //update
            let flipped_statement = statement_to_flip.change_rhs(&missing);
            flipped_statements.push(flipped_statement.clone());

            self.list.remove(&statement_to_flip.rhs().to_owned());
            self.list.insert(flipped_statement.rhs().to_owned(), flipped_statement);
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
    fn rhs(&self) -> &str {
        match self {
            Statement::Number(n, _) => n,
            Statement::Op(n, _, _, _) => n,
        }
    }

    fn contains_operand(&self, operand: &str) -> bool {
        match self {
            Statement::Number(_, _) => false,
            Statement::Op(_, a, _, b) => a == operand || b == operand,
        }
    }

    fn change_rhs(&self, new_rhs: &str) -> Statement {
        match self {
            Statement::Op(rhs, a, op, b) => {
                let (rhs, a, b) = (rhs.to_owned(), a.to_owned(), b.to_owned());
                match op {
                    '+' => { // rhs = a + b
                        match (a == new_rhs, b == new_rhs) {
                            (true, false) => Statement::Op(a, rhs, '-', b), //a = rhs - b
                            (false, true) => Statement::Op(b, rhs, '-', a), //b = rhs - a
                            (_, _) => panic!("invalid flip"),
                        }
                    }
                    '-' => { // rhs = a - b
                        match (a == new_rhs, b == new_rhs) {
                            (true, false) => Statement::Op(a, rhs, '+', b), //a = rhs + b
                            (false, true) => Statement::Op(b, a, '-', rhs), //b = a - rhs
                            (_, _) => panic!("invalid flip"),
                        }
                    }
                    '*' => { // rhs = a * b
                        match (a == new_rhs, b == new_rhs) {
                            (true, false) => Statement::Op(a, rhs, '/', b), //a = rhs / b
                            (false, true) => Statement::Op(b, rhs, '/', a), //b = rhs / a
                            (_, _) => panic!("invalid flip"),
                        }
                    }
                    '/' => { // rhs = a / b
                        match (a == new_rhs, b == new_rhs) {
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
        let rhs = s.split(':').next().unwrap().to_owned();
        match s.split(':').nth(1).unwrap().trim().parse() {
            Ok(n) => Statement::Number(rhs, n),
            Err(_) => {
                let a = s.trim().split(' ').nth(1).unwrap().to_owned();
                let op = s.trim().split(' ').nth(2).unwrap().chars().next().unwrap();
                let b = s.trim().split(' ').nth(3).unwrap().to_owned();
                Statement::Op(rhs, a, op, b)
            }
        }
    }
}