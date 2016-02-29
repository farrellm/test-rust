use std::collections::BinaryHeap;
use std::iter::Iterator;
// use std::boxed::Box;

enum Primes {
    Two,
    Three,
    Sieve{q: BinaryHeap<(i32, i32)>, i: i32},
}

fn advance_top(q: &mut BinaryHeap<(i32, i32)>, v: i32) -> bool {
    match q.peek() {
        None => unreachable!(),
        Some(&(v2, p)) => {
            if v == v2 {
                q.pop();
                q.push((v2 - 2 * p, p));
                true
            } else {
                false
            }
        }
    }
}

fn try_next(q: &mut BinaryHeap<(i32, i32)>, i: i32) -> bool {
    match q.peek() {
        None => unreachable!(),
        Some(&(v, _)) => {
            if i < -v {
                q.push((-i*i, i));
                true
            } else {
                while advance_top(q, v) {}
                false
            }
        }
    }
}

impl Iterator for Primes {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        match *self {
            Primes::Two => {
                *self = Primes::Three;
                Some(2)
            }
            Primes::Three => {
                let mut q = BinaryHeap::new();
                q.push((-9, 3));
                *self = Primes::Sieve {q: q, i: 3};
                Some(3)
            }
            Primes::Sieve {ref mut q, ref mut i} => {
                while !try_next(q, {*i += 2; *i}) {}
                Some(*i)
            }
        }
    }
}

fn primes() -> Primes {
    Primes::Two
}

fn main() {
    for p in primes() {
        println!("{}", p);
        if p > 100 {
            break
        }
    }

    println!("Hello, world!");
}
