use rand::prelude::*;
use std::collections;
use std::time;

mod avl_tree;
mod linked_list;

const NUMS: [usize; 4] = [25, 250, 2500, 25000];

fn main() {
    println!("Testing rust built in linked list vs mine");

    let arr = [100; 2000];
    println!("First Test: Initialization");
    {
        let start = time::Instant::now();
        let _: collections::LinkedList<i32> = collections::LinkedList::new();
        let end = start.elapsed().as_micros();
        println!("std::collections: {end}");

        let start = time::Instant::now();
        let _: linked_list::LinkedList<i32> = linked_list::LinkedList::new();
        let end = start.elapsed().as_micros();
        println!("mine: {end}");
    }

    println!("\nSecond Test: Initialization with 2000 values");
    {
        let start = time::Instant::now();
        let _: collections::LinkedList<i32> = collections::LinkedList::from(arr);
        let end = start.elapsed().as_micros();
        println!("std::collections: {end}");

        let start = time::Instant::now();
        let _: linked_list::LinkedList<i32> = linked_list::LinkedList::from(arr);
        let end = start.elapsed().as_micros();
        println!("mine: {end}");
    }

    let mut rng = rand::thread_rng();
    let mut vector = Vec::with_capacity(2500);
    for _ in 0..*NUMS.last().unwrap() {
        let value: u64 = rng.gen();
        vector.push(value)
    }

    println!("\nThird Test: Push Back with with 25, 250, 2500, 25000 values");
    {
        for num in NUMS {
            let mut list: collections::LinkedList<u64> = collections::LinkedList::new();
            let start = time::Instant::now();
            for i in 0..num {
                list.push_back(vector[i]);
            }
            let end = start.elapsed().as_micros();
            list.clear();
            println!("std::collections {num}: {end}");

            let mut list: linked_list::LinkedList<u64> = linked_list::LinkedList::new();
            let start = time::Instant::now();
            for i in 0..num {
                list.push_back(vector[i]);
            }
            let end = start.elapsed().as_micros();
            list.clear();
            println!("mine {num}: {end}");
        }
    }

    println!("\nFourth Test: Push Front with with 25, 250, 2500, 25000 values");
    {
        for num in NUMS {
            let mut list: collections::LinkedList<u64> = collections::LinkedList::new();
            let start = time::Instant::now();
            for i in 0..num {
                list.push_front(vector[i]);
            }
            let end = start.elapsed().as_micros();
            list.clear();
            println!("std::collections {num}: {end}");

            let mut list: linked_list::LinkedList<u64> = linked_list::LinkedList::new();
            let start = time::Instant::now();
            for i in 0..num {
                list.push_front(vector[i]);
            }
            let end = start.elapsed().as_micros();
            list.clear();
            println!("mine {num}: {end}");
        }
    }

    println!("\nFifth Test: Pop Back with with 25, 250, 2500, 25000 values");
    {
        for num in NUMS {
            let mut list: collections::LinkedList<u64> = collections::LinkedList::new();
            for i in 0..num {
                list.push_front(vector[i]);
            }
            let start = time::Instant::now();
            for _ in 0..num {
                list.pop_back();
            }
            let end = start.elapsed().as_micros();
            list.clear();
            println!("std::collections {num}: {end}");

            let mut list: linked_list::LinkedList<u64> = linked_list::LinkedList::new();
            for i in 0..num {
                list.push_front(vector[i]);
            }
            let start = time::Instant::now();
            for _ in 0..num {
                let _ = list.pop_back();
            }
            let end = start.elapsed().as_micros();
            list.clear();
            println!("mine {num}: {end}");
        }
    }

    println!("\nSixth Test: Pop Front with with 25, 250, 2500, 25000 values");
    {
        for num in NUMS {
            let mut list: collections::LinkedList<u64> = collections::LinkedList::new();
            for i in 0..num {
                list.push_front(vector[i]);
            }
            let start = time::Instant::now();
            for _ in 0..num {
                list.pop_front();
            }
            let end = start.elapsed().as_micros();
            list.clear();
            println!("std::collections {num}: {end}");

            let mut list: linked_list::LinkedList<u64> = linked_list::LinkedList::new();
            for i in 0..num {
                list.push_front(vector[i]);
            }
            let start = time::Instant::now();
            for _ in 0..num {
                let _ = list.pop_front();
            }
            let end = start.elapsed().as_micros();
            list.clear();
            println!("mine {num}: {end}");
        }
    }
}
