use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::io;
use std::io::prelude::*;

/// Kattis calls main function to run your solution.
fn main() {
    // get standard input stream
    let input = io::stdin();

    let f = std::fs::File::open("input.in").unwrap();
    let input = io::BufReader::new(f);
    let lines: Vec<String> = input.lines().map(|_line| _line.ok().unwrap()).collect();
    // get input lines as iterative
    //let lines: Vec<String> = input
    //    .lock()
    //    .lines()
    //    .map(|_line| _line.ok().unwrap())
    //    .collect();

    // loop over each entry
    let mut i = 0;
    while i < lines.len() {
        // initialize collections, and their booleans
        let mut stack = vec![];
        let mut stack_check = true;
        let mut queue = VecDeque::new();
        let mut queue_check = true;
        let mut priority = BinaryHeap::<usize>::new();
        let mut priority_check = true;

        for _ in 0..(lines[i].parse().unwrap()) {
            i += 1;

            let command: Vec<usize> = lines[i]
                .split(' ')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();

            let value = command[1];

            // enter values into the collections
            if command[0] == 1 {
                stack.push(value);
                queue.push_front(value);
                priority.push(value);
            }
            // see if the real popped value matches our collections
            else {
                stack_check = stack_check && (stack.pop() == Some(value));
                queue_check = queue_check && (queue.pop_back() == Some(value));
                priority_check = priority_check && (priority.pop() == Some(value));
            }
        }
        i += 1;
        match (stack_check, priority_check, queue_check) {
            (false, false, false) => println!("impossible"),
            (true, false, false) => println!("stack"),
            (false, true, false) => println!("priority queue"),
            (false, false, true) => println!("queue"),
            (_, _, _) => println!("not sure"),
        }
    }
}
