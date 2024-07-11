use core::panic;
use std::env;
use std::fmt::Display;

use colored::Colorize;
use divisors_fixed::Divisors;

#[derive(Debug)]
enum Op {
    Plus,
    Times,
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plus => write!(f, "+"),
            Self::Times => write!(f, "*"),
        }
    }
}

#[derive(Debug)]
enum Node {
    Leaf(u64),
    Operation(Box<Operation>),
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Leaf(n) => write!(f, "{}", n),
            Self::Operation(operation) => write!(f, "{}", *operation),
        }
    }
}

#[derive(Debug)]
struct Operation {
    a: Node,
    op: Op,
    b: Node,
}

impl Operation {
    pub fn new(a: Node, b: Node, op: Op) -> Self {
        Self { a, b, op }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.a, self.op, self.b)
    }
}

fn split_num(n: u64, extractable: &[u64]) -> Option<(Node, u64)> {
    fn split_num_recurse(n: u64, extractable: &[u64], mut max_depth: u64) -> Option<(Node, u64)> {
        if extractable.contains(&n) {
            return Some((Node::Leaf(n), 0));
        }

        let mut current_best = None;

        let addition_candidates = (1..n).filter(|x| *x < (n - x)).rev();
        for i in addition_candidates {
            if max_depth == 0 {
                return current_best;
            }

            let j = n - i;

            if let (Some((i, depth_i)), Some((j, depth_j))) = (
                split_num_recurse(i, extractable, max_depth - 1),
                split_num_recurse(j, extractable, max_depth - 1),
            ) {
                let depth = u64::max(depth_i, depth_j) + 1;
                if depth < max_depth {
                    max_depth = depth - 1;
                    current_best = Some((
                        Node::Operation(Box::new(Operation::new(i, j, Op::Plus))),
                        depth,
                    ));
                }
            }
        }

        let multiplication_candidates = n
            .divisors()
            .into_iter()
            .skip(1)
            .filter(|x| *x < (n / x))
            .rev();
        for i in multiplication_candidates {
            if max_depth == 0 {
                return current_best;
            }

            let j = n / i;

            if let (Some((i, depth_i)), Some((j, depth_j))) = (
                split_num_recurse(i, extractable, max_depth - 1),
                split_num_recurse(j, extractable, max_depth - 1),
            ) {
                let depth = u64::max(depth_i, depth_j) + 1;
                if depth < max_depth {
                    max_depth = depth - 1;
                    current_best = Some((
                        Node::Operation(Box::new(Operation::new(i, j, Op::Times))),
                        depth,
                    ));
                }
            }
        }
        current_best
    }
    split_num_recurse(n, extractable, u64::MAX)
}

fn main() {
    let mut args = env::args().skip(1).map(|str| match str.parse::<u64>() {
        Ok(result) => result,
        Err(_) => panic!("{} One or more arguments wasn't a number!", "ERROR:".red()),
    });

    let num = args.next().expect("USAGE: The first argument should be the number you wish to reach and all further arguments should be the numbers you have access to.");

    println!(
        "{} = {}",
        num,
        split_num(num, &args.collect::<Vec<_>>()).unwrap().0
    );
}
